use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Active,
    Closed,
    Deleted,
}

impl Default for EventStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub status: EventStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: Uuid,
    pub event_id: Uuid,
    pub nickname: String,
    pub content: String,
    pub phone_masked: String,
    pub is_winner: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicWinner {
    pub nickname: String,
    pub content: String,
    pub phone_masked: String,
}