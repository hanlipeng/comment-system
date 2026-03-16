use crate::core::EventServicePort;
use crate::core::models::{Event, Comment, EventStatus};
use crate::server::HttpServer;
use crate::config::Settings;
use tauri::State;
use uuid::Uuid;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub service: Arc<dyn EventServicePort>,
    pub server: Arc<HttpServer>,
    pub settings: RwLock<Settings>,
}

use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;

// ==========================================
// 服务器控制 Commands
// ==========================================

pub fn resolve_network_url(port: u16) -> Result<String, String> {
    let interfaces = list_afinet_netifas().map_err(|e| e.to_string())?;
    
    let mut ip_addr = "127.0.0.1".to_string();

    // 1. Try to find a non-loopback IPv4 address
    for (_name, ip) in interfaces.iter() {
        if let IpAddr::V4(ipv4) = ip {
            if !ipv4.is_loopback() {
                ip_addr = ipv4.to_string();
                break;
            }
        }
    }
    
    // 2. Format as URL with port
    Ok(format!("http://{}:{}", ip_addr, port))
}

#[tauri::command]
pub async fn get_network_ip(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let settings = state.settings.read().await;
    resolve_network_url(settings.server_port)
}

#[tauri::command]
pub async fn start_server(
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 直接使用配置中的端口
    let settings = state.settings.read().await.clone();
    state.server.start(settings, state.service.clone()).await
}


#[tauri::command]
pub async fn stop_server(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.server.stop().await
}

#[tauri::command]
pub async fn is_server_running(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(state.server.is_running().await)
}

#[tauri::command]
pub async fn reload_settings(
    state: State<'_, AppState>,
) -> Result<ReloadSettingsResponse, String> {
    let new_settings = Settings::load()
        .map_err(|e| format!("重新加载配置失败: {}", e))?;
    let old_settings = state.settings.read().await.clone();

    let changed = ReloadSettingsChanged {
        server_port: old_settings.server_port != new_settings.server_port,
        db_path: old_settings.db_path != new_settings.db_path,
        wifi: old_settings.wifi_ssid != new_settings.wifi_ssid
            || old_settings.wifi_password != new_settings.wifi_password
            || old_settings.wifi_encryption != new_settings.wifi_encryption,
    };

    let mut settings_guard = state.settings.write().await;
    *settings_guard = new_settings.clone();

    Ok(ReloadSettingsResponse {
        settings: new_settings,
        changed,
    })
}

// ==========================================
// 活动管理 Commands (Admin 功能调用)
// ==========================================

#[tauri::command]
pub async fn admin_create_event(
    title: String,
    state: State<'_, AppState>,
) -> Result<Event, String> {
    state.service.create_event(title).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_list_events(
    state: State<'_, AppState>,
) -> Result<Vec<Event>, String> {
    state.service.list_events().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_get_comments(
    event_id: Uuid,
    state: State<'_, AppState>,
) -> Result<Vec<Comment>, String> {
    state.service.list_comments(event_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn admin_draw_winner(
    event_id: Uuid,
    state: State<'_, AppState>,
) -> Result<Comment, String> {
    
    state.service.draw_winner(event_id).await.map_err(|e| e.to_string())
}

#[tauri::command]

pub async fn admin_update_event_status(

    event_id: Uuid,

    status: EventStatus, 

    state: State<'_, AppState>,

) -> Result<(), String> {

    

    state.service.update_event_status(event_id, status).await.map_err(|e| e.to_string())

}



#[tauri::command]
pub async fn admin_delete_event(
    event_id: Uuid,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.service.delete_event(event_id).await.map_err(|e| e.to_string())
}
#[derive(Serialize)]
pub struct ReloadSettingsResponse {
    pub settings: Settings,
    pub changed: ReloadSettingsChanged,
}

#[derive(Serialize)]
pub struct ReloadSettingsChanged {
    pub server_port: bool,
    pub db_path: bool,
    pub wifi: bool,
}
