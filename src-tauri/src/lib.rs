pub mod core;
pub mod api;
pub mod storage;
pub mod server;
pub mod config;

use env_logger;
use std::sync::Arc;
use crate::core::logic::EventService;
use crate::storage::sqlite::{SqliteEventStore, SqliteCommentStore};
use crate::storage::db::SqliteConnection;
use crate::server::HttpServer;
use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 检查 RUST_LOG 环境变量. 例如: `RUST_LOG=info,sqlx=warn`
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 1. Load Settings
            let settings = config::Settings::load_or_create().expect("Failed to load settings.toml");
            println!("Loaded settings: {:?}", settings);

            let handle = app.handle().clone();
            let settings_clone = settings.clone();

            tauri::async_runtime::block_on(async move {
                // 2. Initialize DB
                // Ensure the directory exists
                let db_path = std::path::Path::new(&settings_clone.db_path);
                if let Some(parent) = db_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).expect("Failed to create DB directory");
                    }
                }

                // Append ?mode=rwc to create file if missing
                let db_url = format!("sqlite:{}?mode=rwc", settings_clone.db_path);
                let conn = SqliteConnection::connect(&db_url).await.expect("Failed to connect to DB");
                
                // 3. Initialize Stores & Service
                let event_store = Arc::new(SqliteEventStore::new(conn.clone()));
                let comment_store = Arc::new(SqliteCommentStore::new(conn));
                
                let service = Arc::new(EventService::new(event_store, comment_store));
                let server = Arc::new(HttpServer::new());

                // 4. Manage State
                handle.manage(api::admin::AppState {
                    service,
                    server,
                    settings: RwLock::new(settings_clone),
                });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::admin::start_server,
            api::admin::stop_server,
            api::admin::is_server_running,
            api::admin::get_network_ip,
            api::admin::reload_settings,
            api::admin::admin_create_event,
            api::admin::admin_list_events,
            api::admin::admin_get_comments,
            api::admin::admin_draw_winner,
            api::admin::admin_update_event_status,
            api::admin::admin_delete_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
