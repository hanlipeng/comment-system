use crate::storage::entities::{EventEntity, CommentEntity};
use crate::storage::{EventStorage, CommentStorage};
use crate::storage::db::SqliteConnection;
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteEventStore {
    connection: SqliteConnection,
}

impl SqliteEventStore {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl EventStorage for SqliteEventStore {
    async fn create_event(&self, mut entity: EventEntity) -> Result<String> {
        let new_id = Uuid::new_v4().to_string();
        entity.id = new_id.clone();

        sqlx::query(
            "INSERT INTO events (id, title, status, winner_comment_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&entity.id)
        .bind(&entity.title)
        .bind(&entity.status)
        .bind(&entity.winner_comment_id)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.connection.pool)
        .await?;
        
        Ok(new_id)
    }

    async fn get_event(&self, id: &str) -> Result<Option<EventEntity>> {
        let result = sqlx::query_as::<_, EventEntity>(
            "SELECT id, title, status, winner_comment_id, created_at, updated_at FROM events WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn list_events(&self) -> Result<Vec<EventEntity>> {
        let result = sqlx::query_as::<_, EventEntity>(
            "SELECT id, title, status, winner_comment_id, created_at, updated_at FROM events ORDER BY created_at DESC",
        )
        .fetch_all(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn update_event(&self, entity: EventEntity) -> Result<()> {
        sqlx::query(
            "UPDATE events SET title = ?, status = ?, winner_comment_id = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&entity.title)
        .bind(&entity.status)
        .bind(&entity.winner_comment_id)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.connection.pool)
        .await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct SqliteCommentStore {
    connection: SqliteConnection,
}

impl SqliteCommentStore {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl CommentStorage for SqliteCommentStore {
    async fn create_comment(&self, mut entity: CommentEntity) -> Result<String> {
        let new_id = Uuid::new_v4().to_string();
        entity.id = new_id.clone();

        sqlx::query(
            "INSERT INTO comments (id, event_id, nickname, content, phone, is_winner, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&entity.id)
        .bind(&entity.event_id)
        .bind(&entity.nickname)
        .bind(&entity.content)
        .bind(&entity.phone)
        .bind(entity.is_winner)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.connection.pool)
        .await?;
        
        Ok(new_id)
    }

    async fn get_comment(&self, id: &str) -> Result<Option<CommentEntity>> {
        let result = sqlx::query_as::<_, CommentEntity>(
            "SELECT id, event_id, nickname, content, phone, is_winner, created_at, updated_at FROM comments WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn list_comments_by_event(&self, event_id: &str) -> Result<Vec<CommentEntity>> {
        let result = sqlx::query_as::<_, CommentEntity>(
            "SELECT id, event_id, nickname, content, phone, is_winner, created_at, updated_at FROM comments WHERE event_id = ? ORDER BY created_at DESC",
        )
        .bind(event_id)
        .fetch_all(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn list_recent_comments(&self, event_id: &str, limit: i64) -> Result<Vec<CommentEntity>> {
        let result = sqlx::query_as::<_, CommentEntity>(
            "SELECT id, event_id, nickname, content, phone, is_winner, created_at, updated_at FROM comments WHERE event_id = ? ORDER BY created_at DESC LIMIT ?",
        )
        .bind(event_id)
        .bind(limit)
        .fetch_all(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn find_comment_by_event_and_phone(&self, event_id: &str, phone: &str) -> Result<Option<CommentEntity>> {
        let result = sqlx::query_as::<_, CommentEntity>(
            "SELECT id, event_id, nickname, content, phone, is_winner, created_at, updated_at FROM comments WHERE event_id = ? AND phone = ?",
        )
        .bind(event_id)
        .bind(phone)
        .fetch_optional(&self.connection.pool)
        .await?;
        Ok(result)
    }

    async fn update_comment(&self, entity: CommentEntity) -> Result<()> {
        sqlx::query(
            "UPDATE comments SET nickname = ?, content = ?, phone = ?, is_winner = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&entity.nickname)
        .bind(&entity.content)
        .bind(&entity.phone)
        .bind(entity.is_winner)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.connection.pool)
        .await?;
        Ok(())
    }
}
