use tauri_app_lib::storage::entities::{EventEntity, CommentEntity};
use tauri_app_lib::storage::db::SqliteConnection;
use tauri_app_lib::storage::sqlite::{SqliteEventStore, SqliteCommentStore};
use tauri_app_lib::storage::{EventStorage, CommentStorage};
use chrono::Utc;

async fn setup() -> (SqliteEventStore, SqliteCommentStore) {
    let conn = SqliteConnection::connect("sqlite::memory:").await.expect("Failed to create memory db");
    (SqliteEventStore::new(conn.clone()), SqliteCommentStore::new(conn))
}

#[tokio::test]
async fn test_event_lifecycle() {
    let (event_store, _) = setup().await;
    let now = Utc::now().timestamp();
    
    let event = EventEntity {
        id: "".to_string(), 
        title: "Test Event".into(),
        status: "active".into(),
        winner_comment_id: None,
        created_at: now,
        updated_at: now,
    };

    let created_id = event_store.create_event(event).await.unwrap();
    assert!(!created_id.is_empty());

    let fetched = event_store.get_event(&created_id).await.unwrap().expect("Event should exist");
    assert_eq!(fetched.title, "Test Event");

    let mut updated = fetched.clone();
    updated.status = "closed".into();
    event_store.update_event(updated).await.unwrap();

    let re_fetched = event_store.get_event(&created_id).await.unwrap().unwrap();
    assert_eq!(re_fetched.status, "closed");
}

#[tokio::test]
async fn test_comment_lifecycle() {
    let (event_store, comment_store) = setup().await;
    let now = Utc::now().timestamp();

    let event = EventEntity {
        id: "".into(),
        title: "Comment Host".into(),
        status: "active".into(),
        winner_comment_id: None,
        created_at: now,
        updated_at: now,
    };
    let event_id = event_store.create_event(event).await.unwrap();

    let comment = CommentEntity {
        id: "".into(),
        event_id: event_id.clone(),
        nickname: "User".into(),
        content: "Hi".into(),
        phone: "123".into(),
        is_winner: false,
        created_at: now,
        updated_at: now,
    };

    let comment_id = comment_store.create_comment(comment).await.unwrap();
    assert!(!comment_id.is_empty());

    let fetched = comment_store.get_comment(&comment_id).await.unwrap().unwrap();
    assert_eq!(fetched.content, "Hi");

    let mut updated = fetched.clone();
    updated.is_winner = true;
    comment_store.update_comment(updated).await.unwrap();

    let list = comment_store.list_comments_by_event(&event_id).await.unwrap();
    assert_eq!(list.len(), 1);
    assert!(list[0].is_winner);
}

#[tokio::test]
async fn test_event_winner_linking() {
    let (event_store, _) = setup().await;

    let event = EventEntity {
        id: "".into(),
        title: "Linked Event".into(),
        status: "active".into(),
        winner_comment_id: None,
        created_at: 0,
        updated_at: 0,
    };
    let event_id = event_store.create_event(event).await.unwrap();
    
    let comment_id = "some_random_comment_id"; 

    let mut fetched_event = event_store.get_event(&event_id).await.unwrap().unwrap();
    fetched_event.winner_comment_id = Some(comment_id.into());
    event_store.update_event(fetched_event).await.unwrap();

    let final_event = event_store.get_event(&event_id).await.unwrap().unwrap();
    assert_eq!(final_event.winner_comment_id, Some(comment_id.into()));
}

#[tokio::test]
async fn test_missing_records() {
    let (event_store, comment_store) = setup().await;
    
    let event = event_store.get_event("non_existent").await.unwrap();
    assert!(event.is_none());

    let comment = comment_store.get_comment("non_existent").await.unwrap();
    assert!(comment.is_none());
}
