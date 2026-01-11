use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventEntity {
    pub id: String,
    pub title: String,
    pub status: String, // "active", "closed"
    pub winner_comment_id: Option<String>, // New field for bidirectional linking
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommentEntity {
    pub id: String,
    pub event_id: String,
    pub nickname: String,
    pub content: String,
    pub phone: String,
    pub is_winner: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
