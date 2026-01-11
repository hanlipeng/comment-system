pub mod core;
pub mod api;
pub mod storage;
pub mod server;

use std::sync::Arc;
use crate::core::logic::EventService;
use crate::storage::sqlite::{SqliteEventStore, SqliteCommentStore};
use crate::storage::db::SqliteConnection;
use crate::server::HttpServer;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db_url = "sqlite:comment_system.db";
                let conn = SqliteConnection::connect(db_url).await.expect("Failed to connect to DB");
                
                let event_store = Arc::new(SqliteEventStore::new(conn.clone()));
                let comment_store = Arc::new(SqliteCommentStore::new(conn));
                
                let service = Arc::new(EventService::new(event_store, comment_store));
                let server = Arc::new(HttpServer::new());

                handle.manage(api::admin::AppState {
                    service,
                    server,
                });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            api::admin::start_server,
            api::admin::stop_server,
            api::admin::is_server_running,
            api::admin::admin_create_event,
            api::admin::admin_list_events,
            api::admin::admin_get_comments,
            api::admin::admin_draw_winner,
            api::admin::admin_update_event_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
