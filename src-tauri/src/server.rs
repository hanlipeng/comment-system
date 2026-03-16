use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::oneshot;
use crate::core::EventServicePort;
use crate::config::Settings;
use axum::{
    http::{StatusCode, header, Uri},
    response::IntoResponse,
    Router,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../web-client/dist"]
struct Assets;

pub struct HttpServer {
    shutdown_tx: Mutex<Option<oneshot::Sender<()>>>,
}

impl HttpServer {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Mutex::new(None),
        }
    }

    pub async fn start(&self, settings: Settings, service: Arc<dyn EventServicePort>) -> Result<(), String> {
        let mut tx_guard = self.shutdown_tx.lock().await;
        if tx_guard.is_some() {
            return Err("Server is already running".to_string());
        }

        let port = settings.server_port;
        let (tx, rx) = oneshot::channel();
        
        // API Router (nested under /api/v1)
        let api_routes = crate::api::web::router(service, settings);
        
        // Main Router
        let app = Router::new()
            .nest("/api/v1", api_routes)
            .fallback(static_handler);

        let addr = format!("0.0.0.0:{}", port);
        
        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;
            
        println!("Server listening on {}", addr);

        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    rx.await.ok();
                })
                .await 
            {
                eprintln!("Server error: {}", e);
            }
        });

        *tx_guard = Some(tx);
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        let mut tx_guard = self.shutdown_tx.lock().await;
        if let Some(tx) = tx_guard.take() {
            tx.send(()).map_err(|_| "Failed to send shutdown signal".to_string())?;
            Ok(())
        } else {
            Err("Server is not running".to_string())
        }
    }

    pub async fn is_running(&self) -> bool {
        self.shutdown_tx.lock().await.is_some()
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    
    // 1. Try to serve the file directly
    if let Some(content) = Assets::get(&path) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        return ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response();
    }

    // 2. If it's the root path or looks like an SPA route (no extension), serve index.html
    if path.is_empty() || !path.contains('.') {
        if let Some(content) = Assets::get("index.html") {
             return ([(header::CONTENT_TYPE, "text/html")], content.data).into_response();
        }
    }

    // 3. Not found
    StatusCode::NOT_FOUND.into_response()
}