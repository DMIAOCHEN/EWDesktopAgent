// Configuration module
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSystem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub business_systems: Vec<BusinessSystem>,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub theme: String,
    pub voice_enabled: bool,
    pub memory_limit_mb: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            business_systems: Self::default_business_systems(),
            user_preferences: UserPreferences {
                language: "zh-CN".to_string(),
                theme: "light".to_string(),
                voice_enabled: true,
                memory_limit_mb: 500,
            },
        }
    }
}

impl AppConfig {
    fn default_business_systems() -> Vec<BusinessSystem> {
        vec![
            BusinessSystem {
                id: "ris".to_string(),
                name: "RIS (放射信息系统)".to_string(),
                url: "http://localhost:8080/ris".to_string(),
                icon: None,
                enabled: true,
            },
            BusinessSystem {
                id: "pis".to_string(),
                name: "PIS (病理信息系统)".to_string(),
                url: "http://localhost:8080/pis".to_string(),
                icon: None,
                enabled: true,
            },
            BusinessSystem {
                id: "eis".to_string(),
                name: "EIS (检验信息系统)".to_string(),
                url: "http://localhost:8080/eis".to_string(),
                icon: None,
                enabled: true,
            },
        ]
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("EWDesktopAgent")
            .join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => {
                            info!("Loaded config from {:?}", path);
                            return config;
                        }
                        Err(e) => {
                            error!("Failed to parse config: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read config file: {}", e);
                }
            }
        }
        info!("Using default configuration");
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        info!("Saved config to {:?}", path);
        Ok(())
    }
}

/// Tauri commands for configuration

#[tauri::command]
pub fn load_business_systems() -> Result<Vec<BusinessSystem>, String> {
    let config = AppConfig::load();
    Ok(config.business_systems)
}

#[tauri::command]
pub fn save_business_system(system: BusinessSystem) -> Result<(), String> {
    let mut config = AppConfig::load();

    // Update or add the business system
    if let Some(existing) = config.business_systems.iter_mut().find(|s| s.id == system.id) {
        *existing = system;
    } else {
        config.business_systems.push(system);
    }

    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_app_config() -> Result<AppConfig, String> {
    Ok(AppConfig::load())
}
