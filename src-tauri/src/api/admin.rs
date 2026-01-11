use crate::core::EventServicePort;
use crate::core::models::{Event, Comment, EventStatus};
use crate::server::HttpServer;
use tauri::State;
use uuid::Uuid;
use std::sync::Arc;

pub struct AppState {
    /// 使用接口而非具体实现，支持 Mock 和解耦
    pub service: Arc<dyn EventServicePort>,
    /// HTTP 服务器实例
    pub server: Arc<HttpServer>,
}

// ==========================================
// 服务器控制 Commands (ServerMgr 相关需求)
// ==========================================

#[tauri::command]
pub async fn start_server(
    port: u16,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.server.start(port, state.service.clone()).await
}

#[tauri::command]
pub async fn stop_server(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.server.stop().await
}

#[tauri::command]
pub async fn is_server_running(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(state.server.is_running().await)
}

// ==========================================
// 活动管理 Commands (Admin 功能调用)
// ==========================================

#[tauri::command]
pub async fn admin_create_event(
    title: String,
    state: State<'_, AppState>,
) -> Result<Event, String> {
    state.service.create_event(title).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_list_events(
    state: State<'_, AppState>,
) -> Result<Vec<Event>, String> {
    state.service.list_events().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_comments(
    event_id: Uuid,
    state: State<'_, AppState>,
) -> Result<Vec<Comment>, String> {
    state.service.list_comments(event_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_draw_winner(
    event_id: Uuid,
    state: State<'_, AppState>,
) -> Result<Comment, String> {
    state.service.draw_winner(event_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_update_event_status(
    event_id: Uuid,
    status: EventStatus, 
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.service.update_event_status(event_id, status).await.map_err(|e| e.to_string())
}