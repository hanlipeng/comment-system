use tauri_app_lib::core::*;
use tauri_app_lib::storage::{EventStorage, CommentStorage};
use tauri_app_lib::storage::entities::{EventEntity, CommentEntity};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use anyhow::Result;

struct MockStorage {
    events: Mutex<Vec<EventEntity>>,
    comments: Mutex<Vec<CommentEntity>>,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            events: Mutex::new(vec![]),
            comments: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl EventStorage for MockStorage {
    async fn create_event(&self, mut entity: EventEntity) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        entity.id = id.clone();
        self.events.lock().unwrap().push(entity);
        Ok(id)
    }
    async fn get_event(&self, id: &str) -> Result<Option<EventEntity>> {
        Ok(self.events.lock().unwrap().iter().find(|e| e.id == id).cloned())
    }
    async fn list_events(&self) -> Result<Vec<EventEntity>> {
        Ok(self.events.lock().unwrap().clone())
    }
    async fn update_event(&self, entity: EventEntity) -> Result<()> {
        let mut events = self.events.lock().unwrap();
        if let Some(e) = events.iter_mut().find(|e| e.id == entity.id) {
            *e = entity;
        }
        Ok(())
    }
}

#[async_trait]
impl CommentStorage for MockStorage {
    async fn create_comment(&self, mut entity: CommentEntity) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        entity.id = id.clone();
        self.comments.lock().unwrap().push(entity);
        Ok(id)
    }
    async fn get_comment(&self, id: &str) -> Result<Option<CommentEntity>> {
        Ok(self.comments.lock().unwrap().iter().find(|c| c.id == id).cloned())
    }
    async fn list_comments_by_event(&self, event_id: &str) -> Result<Vec<CommentEntity>> {
        Ok(self.comments.lock().unwrap().iter().filter(|c| c.event_id == event_id).cloned().collect())
    }
    async fn update_comment(&self, entity: CommentEntity) -> Result<()> {
        let mut comments = self.comments.lock().unwrap();
        if let Some(c) = comments.iter_mut().find(|c| c.id == entity.id) {
            *c = entity;
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_create_event() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage);

    let event = service.create_event("Test Event".to_string()).await.unwrap();
    assert_eq!(event.title, "Test Event");
    assert_eq!(event.status, EventStatus::Active);
}

#[tokio::test]
async fn test_add_comment() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Test Event".to_string()).await.unwrap();
    let comment = service.add_comment(event.id, "User1".to_string(), "Nice!".to_string(), "13800001111".to_string()).await.unwrap();

    assert_eq!(comment.nickname, "User1");
    assert_eq!(comment.event_id, event.id);
    
    let comments = storage.list_comments_by_event(&event.id.to_string()).await.unwrap();
    assert_eq!(comments.len(), 1);
}

#[tokio::test]
async fn test_draw_winner() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Draw Event".to_string()).await.unwrap();
    service.add_comment(event.id, "U1".to_string(), "C1".to_string(), "111".to_string()).await.unwrap();
    service.add_comment(event.id, "U2".to_string(), "C2".to_string(), "222".to_string()).await.unwrap();

    let winner = service.draw_winner(event.id).await.unwrap();
    assert!(winner.is_winner);

    let winner_again = service.draw_winner(event.id).await.unwrap();
    assert_eq!(winner.id, winner_again.id);
}

#[tokio::test]
async fn test_create_event_empty_title() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage);

    let result = service.create_event("  ".to_string()).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Title cannot be empty");
}

#[tokio::test]
async fn test_add_comment_to_closed_event() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Closed Event".to_string()).await.unwrap();
    
    // 手动修改状态为 closed
    let mut events = storage.events.lock().unwrap();
    if let Some(e) = events.iter_mut().find(|e| e.id == event.id.to_string()) {
        e.status = "closed".to_string();
    }
    drop(events);

    let result = service.add_comment(event.id, "User".to_string(), "Hi".to_string(), "123".to_string()).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Event is closed");
}

#[tokio::test]
async fn test_draw_winner_no_comments() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Empty Event".to_string()).await.unwrap();
    let result = service.draw_winner(event.id).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "No comments available for drawing");
}

#[test]
fn test_mask_phone() {
    assert_eq!(mask_phone("13812345678"), "138****5678");
    assert_eq!(mask_phone("123456"), "****");
}

#[tokio::test]
async fn test_get_public_winner_masking() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());
    
    let event = service.create_event("Winner Event".to_string()).await.unwrap();
    let _comment = service.add_comment(event.id, "Winner".to_string(), "Win!".to_string(), "13812345678".to_string()).await.unwrap();
    
    service.draw_winner(event.id).await.unwrap();
    
    let public_winner = service.get_public_winner(event.id).await.unwrap().unwrap();
    assert_eq!(public_winner.phone_masked, "138****5678");
}

#[tokio::test]
async fn test_list_events() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage);

    service.create_event("Event 1".to_string()).await.unwrap();
    service.create_event("Event 2".to_string()).await.unwrap();

    let events = service.list_events().await.unwrap();
    assert_eq!(events.len(), 2);
}

#[tokio::test]
async fn test_list_comments() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Comment Event".to_string()).await.unwrap();
    service.add_comment(event.id, "User1".to_string(), "Hi".to_string(), "123456789".to_string()).await.unwrap();

    let comments = service.list_comments(event.id).await.unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].phone, "123456789"); // Should be full phone
}

#[tokio::test]
async fn test_update_event_status() {
    let storage = Arc::new(MockStorage::new());
    let service = EventService::new(storage.clone(), storage.clone());

    let event = service.create_event("Status Event".to_string()).await.unwrap();
    assert_eq!(event.status, EventStatus::Active);

    service.update_event_status(event.id, EventStatus::Closed).await.unwrap();
    
    let updated_event = service.get_event(event.id).await.unwrap().unwrap();
    assert_eq!(updated_event.status, EventStatus::Closed);
}