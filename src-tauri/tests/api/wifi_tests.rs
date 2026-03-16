use tauri_app_lib::api::web::router;
use tauri_app_lib::config::Settings;
use tauri_app_lib::core::{EventServicePort, models::{Event, Comment, EventStatus, PublicWinner}};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; 
use http_body_util::BodyExt; 

struct DummyService;

#[async_trait]
impl EventServicePort for DummyService {
    async fn create_event(&self, _title: String) -> Result<Event> { unimplemented!() }
    async fn get_event(&self, _event_id: Uuid) -> Result<Option<Event>> { unimplemented!() }
    async fn get_active_event(&self) -> Result<Option<Event>> { unimplemented!() }
    async fn add_comment(&self, _event_id: Uuid, _nickname: String, _content: String, _phone: String) -> Result<Comment> { unimplemented!() }
    async fn get_recent_comments(&self, _event_id: Uuid, _limit: usize) -> Result<Vec<Comment>> { unimplemented!() }
    async fn draw_winner(&self, _event_id: Uuid) -> Result<Comment> { unimplemented!() }
    async fn get_public_winner(&self, _event_id: Uuid) -> Result<Option<PublicWinner>> { unimplemented!() }
    async fn list_events(&self) -> Result<Vec<Event>> { unimplemented!() }
    async fn list_comments(&self, _event_id: Uuid) -> Result<Vec<Comment>> { unimplemented!() }
    async fn update_event_status(&self, _event_id: Uuid, _status: EventStatus) -> Result<()> { unimplemented!() }
    async fn delete_event(&self, _event_id: Uuid) -> Result<()> { unimplemented!() }
}

#[tokio::test]
async fn test_get_wifi_config_success() {
    let mut settings = Settings::default();
    settings.wifi_ssid = Some("TestWiFi".to_string());
    settings.wifi_password = Some("password123".to_string());
    settings.wifi_encryption = Some("WPA2".to_string());

    let service = Arc::new(DummyService);
    let app = router(service, settings);

    let response = app
        .oneshot(Request::builder().uri("/wifi").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json["ssid"], "TestWiFi");
    assert_eq!(body_json["password"], "password123");
    assert_eq!(body_json["encryption"], "WPA2");
}

#[tokio::test]
async fn test_get_wifi_config_not_found() {
    let mut settings = Settings::default();
    settings.wifi_ssid = None;

    let service = Arc::new(DummyService);
    let app = router(service, settings);

    let response = app
        .oneshot(Request::builder().uri("/wifi").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
