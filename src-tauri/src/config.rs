use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub server_port: u16,
    pub db_path: String,

    // New Fields
    pub wifi_ssid: Option<String>,
    pub wifi_password: Option<String>,
    pub wifi_encryption: Option<String>, // "WPA", "WEP", "nopass" (Default: "WPA")
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_port: 3000,
            db_path: "./db/data.db".to_string(),
            wifi_ssid: None,
            wifi_password: None,
            wifi_encryption: Some("WPA".to_string()),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = "settings.toml";
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read {}", config_path))?;
        let settings: Settings =
            toml::from_str(&content).context("Failed to parse settings.toml")?;
        Ok(settings)
    }

    pub fn load_or_create() -> Result<Self> {
        let config_path = "settings.toml";

        if Path::new(config_path).exists() {
            let content = fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read {}", config_path))?;
            let settings: Settings =
                toml::from_str(&content).context("Failed to parse settings.toml")?;
            return Ok(settings);
        }

        let default = Settings::default();

        let content = r#"# Server Configuration
server_port = 3000
db_path = "./db/data.db"

# Wi-Fi Configuration
# Uncomment and set these values to display Wi-Fi info on the Desktop Result View
# wifi_ssid = "Your_SSID"
# wifi_password = "Your_Password"
# wifi_encryption = "WPA" # Options: WPA, WEP, nopass
"#;

        fs::write(config_path, content)?;

        Ok(default)
    }
}
