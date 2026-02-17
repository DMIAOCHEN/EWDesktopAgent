// Configuration module
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            business_systems: Vec::new(),
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
    pub fn load() -> Self {
        // TODO: Load from config file
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        // TODO: Save to config file
        Ok(())
    }
}
