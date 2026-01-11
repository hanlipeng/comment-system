pub mod entities;
pub mod sqlite;
pub mod db;

use async_trait::async_trait;
use anyhow::Result;
use crate::storage::entities::{EventEntity, CommentEntity};

#[async_trait]
pub trait EventStorage: Send + Sync {
    async fn create_event(&self, entity: EventEntity) -> Result<String>;
    async fn get_event(&self, id: &str) -> Result<Option<EventEntity>>;
    async fn list_events(&self) -> Result<Vec<EventEntity>>;
    async fn update_event(&self, entity: EventEntity) -> Result<()>;
}

#[async_trait]
pub trait CommentStorage: Send + Sync {
    async fn create_comment(&self, entity: CommentEntity) -> Result<String>;
    async fn get_comment(&self, id: &str) -> Result<Option<CommentEntity>>;
    async fn list_comments_by_event(&self, event_id: &str) -> Result<Vec<CommentEntity>>;
    async fn update_comment(&self, entity: CommentEntity) -> Result<()>;
}
