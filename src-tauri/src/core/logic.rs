use anyhow::{Result, bail, Context};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use async_trait::async_trait;

use crate::core::models::{Event, Comment, EventStatus, PublicWinner};
use crate::core::EventServicePort;
use crate::storage::{EventStorage, CommentStorage};
use crate::storage::entities::{EventEntity, CommentEntity};

pub struct EventService {
    event_storage: Arc<dyn EventStorage>,
    comment_storage: Arc<dyn CommentStorage>,
}

impl EventService {
    pub fn new(event_storage: Arc<dyn EventStorage>, comment_storage: Arc<dyn CommentStorage>) -> Self {
        Self { 
            event_storage,
            comment_storage,
        }
    }
}

#[async_trait]
impl EventServicePort for EventService {
    /// 创建新活动
    async fn create_event(&self, title: String) -> Result<Event> {
        if title.trim().is_empty() {
            bail!("Title cannot be empty");
        }
        
        let now = Utc::now().timestamp();
        // ID 暂时传空串，由 Storage 生成
        let entity = EventEntity {
            id: "".into(),
            title: title.clone(),
            status: "active".into(),
            winner_comment_id: None,
            created_at: now,
            updated_at: now,
        };

        let generated_id = self.event_storage.create_event(entity).await?;
        let id = Uuid::parse_str(&generated_id).context("Failed to parse generated ID")?;

        Ok(Event {
            id,
            title,
            status: EventStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }

    /// 获取指定活动
    async fn get_event(&self, event_id: Uuid) -> Result<Option<Event>> {
        let entity = self.event_storage.get_event(&event_id.to_string()).await?;
        if let Some(e) = entity {
            return Ok(Some(map_event_from_entity(e)?));
        }
        Ok(None)
    }

    /// 获取当前活跃活动（返回最新的一个活跃活动）
    async fn get_active_event(&self) -> Result<Option<Event>> {
        let events = self.event_storage.list_events().await?;
        // 假设 list_events 按时间倒序排列，找到第一个 Active 的即可
        let active_entity = events.into_iter().find(|e| e.status == "active");
        
        if let Some(e) = active_entity {
             return Ok(Some(map_event_from_entity(e)?));
        }
        Ok(None)
    }

    /// 添加留言
    async fn add_comment(&self, event_id: Uuid, nickname: String, content: String, phone: String) -> Result<Comment> {
        let event_entity = self.event_storage.get_event(&event_id.to_string()).await?;
        match event_entity {
            Some(e) if e.status == "active" => (),
            Some(_) => bail!("Event is closed"),
            None => bail!("Event not found"),
        }

        if nickname.trim().is_empty() || content.trim().is_empty() {
            bail!("Nickname and content cannot be empty");
        }

        let now = Utc::now().timestamp();
        let entity = CommentEntity {
            id: "".into(),
            event_id: event_id.to_string(),
            nickname: nickname.clone(),
            content: content.clone(),
            phone: phone.clone(),
            is_winner: false,
            created_at: now,
            updated_at: now,
        };

        let generated_id = self.comment_storage.create_comment(entity).await?;
        let id = Uuid::parse_str(&generated_id).context("Failed to parse generated ID")?;

        Ok(Comment {
            id,
            event_id,
            nickname,
            content,
            phone,
            is_winner: false,
            created_at: now,
            updated_at: now,
        })
    }

    /// 随机抽奖
    async fn draw_winner(&self, event_id: Uuid) -> Result<Comment> {
        let event_entity = self.event_storage.get_event(&event_id.to_string()).await?
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        // 如果已经有中奖者，直接返回
        if let Some(winner_id) = event_entity.winner_comment_id {
            let winner_entity = self.comment_storage.get_comment(&winner_id).await?
                .ok_or_else(|| anyhow::anyhow!("Winner comment record lost"))?;
            return Ok(map_comment_from_entity(winner_entity)?);
        }

        // 获取所有留言
        let comments = self.comment_storage.list_comments_by_event(&event_id.to_string()).await?;
        if comments.is_empty() {
            bail!("No comments available for drawing");
        }

        // 随机挑选
        // Scope the RNG usage to ensure it is dropped before the await
        let winner_entity = {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            comments.choose(&mut rng).ok_or_else(|| anyhow::anyhow!("Draw failed"))?.clone()
        };

        // 更新状态：Comment 标记为 winner，Event 记录 winner_id
        let mut updated_comment = winner_entity.clone();
        updated_comment.is_winner = true;
        updated_comment.updated_at = Utc::now().timestamp();
        self.comment_storage.update_comment(updated_comment.clone()).await?;

        let mut updated_event = event_entity;
        updated_event.winner_comment_id = Some(updated_comment.id.clone());
        updated_event.updated_at = Utc::now().timestamp();
        self.event_storage.update_event(updated_event).await?;
        
        Ok(map_comment_from_entity(updated_comment)?)
    }

    /// 获取活动脱敏后的中奖信息
    async fn get_public_winner(&self, event_id: Uuid) -> Result<Option<PublicWinner>> {
        let event_entity = self.event_storage.get_event(&event_id.to_string()).await?;
        if let Some(e) = event_entity {
            if let Some(winner_id) = e.winner_comment_id {
                if let Some(c) = self.comment_storage.get_comment(&winner_id).await? {
                    return Ok(Some(PublicWinner {
                        nickname: c.nickname,
                        content: c.content,
                        phone_masked: mask_phone(&c.phone),
                    }));
                }
            }
        }
        Ok(None)
    }

    /// [Admin] 获取所有活动列表
    async fn list_events(&self) -> Result<Vec<Event>> {
        let entities = self.event_storage.list_events().await?;
        let mut events = Vec::new();
        for e in entities {
            events.push(map_event_from_entity(e)?);
        }
        Ok(events)
    }

    /// [Admin] 获取指定活动的所有留言
    async fn list_comments(&self, event_id: Uuid) -> Result<Vec<Comment>> {
        let entities = self.comment_storage.list_comments_by_event(&event_id.to_string()).await?;
        let mut comments = Vec::new();
        for c in entities {
            comments.push(map_comment_from_entity(c)?);
        }
        Ok(comments)
    }

    /// [Admin] 更新活动状态
    async fn update_event_status(&self, event_id: Uuid, status: EventStatus) -> Result<()> {
        let mut event_entity = self.event_storage.get_event(&event_id.to_string()).await?
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        let status_str = match status {
            EventStatus::Active => "active",
            EventStatus::Closed => "closed",
        };
        
        event_entity.status = status_str.to_string();
        event_entity.updated_at = Utc::now().timestamp();
        
        self.event_storage.update_event(event_entity).await?;
        Ok(())
    }
}

fn map_event_from_entity(entity: EventEntity) -> Result<Event> {
    let status = match entity.status.as_str() {
        "active" => EventStatus::Active,
        _ => EventStatus::Closed,
    };
    Ok(Event {
        id: Uuid::parse_str(&entity.id)?,
        title: entity.title,
        status,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    })
}

fn map_comment_from_entity(entity: CommentEntity) -> Result<Comment> {
    Ok(Comment {
        id: Uuid::parse_str(&entity.id)?,
        event_id: Uuid::parse_str(&entity.event_id)?,
        nickname: entity.nickname,
        content: entity.content,
        phone: entity.phone,
        is_winner: entity.is_winner,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    })
}

pub fn mask_phone(phone: &str) -> String {
    if phone.len() < 7 {
        return "****".to_string();
    }
    let mut masked = phone.to_string();
    masked.replace_range(3..7, "****");
    masked
}