use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::oneshot;
use crate::core::EventServicePort;

pub struct HttpServer {
    shutdown_tx: Mutex<Option<oneshot::Sender<()>>>,
}

impl HttpServer {
    pub fn new() -> Self {
        Self {
            shutdown_tx: Mutex::new(None),
        }
    }

    pub async fn start(&self, port: u16, service: Arc<dyn EventServicePort>) -> Result<(), String> {
        let mut tx_guard = self.shutdown_tx.lock().await;
        if tx_guard.is_some() {
            return Err("Server is already running".to_string());
        }

        let (tx, rx) = oneshot::channel();
        let app = crate::api::web::router(service);
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
