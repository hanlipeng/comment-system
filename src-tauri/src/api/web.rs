use axum::{
    extract::{Path, State},
    http::{StatusCode, Method},
    response::Json,
    Router, routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::core::EventServicePort;
use crate::core::models::EventStatus;
use crate::config::Settings;
use tower_http::cors::{CorsLayer, Any};

// ==========================================
// DTOs (Data Transfer Objects)
// ==========================================

#[derive(Debug, Serialize)]
pub struct WebWinner {
    pub nickname: String,
    pub content: String,
    pub phone_masked: String,
}

#[derive(Debug, Serialize)]
pub struct WebComment {
    pub nickname: String,
    pub content: String,
    pub phone_masked: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct WebEventResponse {
    pub id: String,
    pub title: String,
    pub status: String,
    pub winner: Option<WebWinner>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub nickname: String,
    pub content: String,
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct WifiConfigResponse {
    pub ssid: String,
    pub password: Option<String>,
    pub encryption: String,
}

// ==========================================
// App State
// ==========================================

#[derive(Clone)]
pub struct WebAppState {
    /// 注入 EventServicePort 接口，支持解耦和 Mock
    pub service: Arc<dyn EventServicePort>,
    pub settings: Settings,
}

// ==========================================
// Handlers (Web API 实现需求)
// ==========================================

/// [GET /events/active] 获取当前活跃活动
/// 
/// 需求描述：
/// 1. 调用 `service.get_active_event()` 查找状态为 `Active` 的活动。
/// 2. 若存在，则调用 `service.get_public_winner(event.id)` 获取已脱敏的中奖信息。
/// 3. 将结果映射为 `WebEventResponse` 并返回。
/// 4. 若无活跃活动，应返回 `404 Not Found`。
pub async fn get_active_event(
    State(state): State<WebAppState>,
) -> Result<Json<Option<WebEventResponse>>, StatusCode> {
    // 此处逻辑由其他 Agent 根据需求完善
    let event = state.service.get_active_event().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match event {
        Some(e) => {
            let winner = state.service.get_public_winner(e.id).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let web_winner = winner.map(|w| WebWinner {
                nickname: w.nickname,
                content: w.content,
                phone_masked: w.phone_masked,
            });

            Ok(Json(Some(WebEventResponse {
                id: e.id.to_string(),
                title: e.title,
                status: match e.status { 
                    EventStatus::Active => "active".to_string(), 
                    EventStatus::Closed => "closed".to_string(),
                    EventStatus::Deleted => "deleted".to_string(),
                },
                winner: web_winner,
            })))
        },
        None => Ok(Json(None)),
    }
}

/// [GET /events/{id}] 获取指定活动详情
/// 
/// 需求描述：
/// 1. 根据路径参数 `id` 调用 `service.get_event(id)`。
/// 2. 若活动存在，获取其中奖信息（如有）并返回 200。
/// 3. 若活动不存在，返回 404。
pub async fn get_event(
    State(state): State<WebAppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WebEventResponse>, StatusCode> {
    let event = state.service.get_event(id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match event {
        Some(e) => {
            let winner = state.service.get_public_winner(e.id).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let web_winner = winner.map(|w| WebWinner {
                nickname: w.nickname,
                content: w.content,
                phone_masked: w.phone_masked,
            });

            Ok(Json(WebEventResponse {
                id: e.id.to_string(),
                title: e.title,
                status: match e.status { 
                    EventStatus::Active => "active".to_string(), 
                    EventStatus::Closed => "closed".to_string(),
                    EventStatus::Deleted => "deleted".to_string(),
                },
                winner: web_winner,
            }))
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// [POST /events/{id}/comments] 提交留言
/// 
/// 需求描述：
/// 1. 验证请求体字段是否为空。
/// 2. 校验该手机号在此活动中是否已存在记录 (409 Conflict)。
/// 3. 调用 `service.add_comment` 尝试保存留言。
/// 4. 成功后返回 201 Created。
pub async fn create_comment(
    State(state): State<WebAppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<StatusCode, StatusCode> {
    // 基础校验
    if payload.nickname.trim().is_empty() || payload.content.trim().is_empty() || payload.phone.trim().is_empty() {
         return Err(StatusCode::BAD_REQUEST);
    }

    match state.service.add_comment(id, payload.nickname, payload.content, payload.phone).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Event not found") {
                Err(StatusCode::NOT_FOUND)
            } else if msg.contains("Event is closed") {
                Err(StatusCode::FORBIDDEN)
            } else if msg.contains("Phone number already used") {
                Err(StatusCode::CONFLICT)
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}

/// [GET /events/{id}/comments] 获取最新留言列表
///
/// 需求描述：
/// 1. 根据路径参数 `id` 验证活动是否存在。
/// 2. 调用 `service.get_recent_comments(id, limit=20)` 获取最新的留言。
/// 3. 对手机号进行脱敏处理（与 WebWinner 逻辑一致）。
/// 4. 返回 `Vec<WebComment>`。
pub async fn get_comments(
    State(state): State<WebAppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<WebComment>>, StatusCode> {
    // 验证活动是否存在
    let event = state.service.get_event(id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if event.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // 获取最新 20 条评论
    let comments = state.service.get_recent_comments(id, 20).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let web_comments = comments.into_iter().map(|c| WebComment {
        nickname: c.nickname,
        content: c.content,
        phone_masked: c.phone_masked,
        created_at: c.created_at,
    }).collect();

    Ok(Json(web_comments))
}

// ==========================================
// Router Configuration
// ==========================================

pub fn router(service: Arc<dyn EventServicePort>, settings: Settings) -> Router {
    let state = WebAppState { service , settings};

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/events/active", get(get_active_event))
        .route("/events/{id}", get(get_event))
        .route("/events/{id}/comments", post(create_comment).get(get_comments))
        .layer(cors)
        .with_state(state)
}