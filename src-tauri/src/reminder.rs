// Reminder module - Intelligent notification system
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::info;

/// Reminder rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderRule {
    pub id: String,
    pub name: String,
    pub trigger_type: String,
    pub trigger_config: TriggerConfig,
    pub content: String,
    pub target_url: Option<String>,
    pub enabled: bool,
}

/// Trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    pub trigger_type: String, // "time", "page", "keyword"
    pub value: String,
    pub interval_minutes: Option<i32>,
}

/// Reminder record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderRecord {
    pub id: String,
    pub rule_id: String,
    pub user_id: String,
    pub content: String,
    pub target_url: Option<String>,
    pub triggered_at: String,
    pub is_read: bool,
}

/// Reminder state
pub struct ReminderState {
    pub rules: Mutex<Vec<ReminderRule>>,
    pub records: Mutex<Vec<ReminderRecord>>,
}

impl Default for ReminderState {
    fn default() -> Self {
        Self {
            rules: Mutex::new(Vec::new()),
            records: Mutex::new(Vec::new()),
        }
    }
}

/// Tauri commands for reminders

#[tauri::command]
pub fn create_reminder_rule(rule: ReminderRule) -> Result<ReminderRule, String> {
    info!("Creating reminder rule: {}", rule.name);
    let mut new_rule = rule;
    if new_rule.id.is_empty() {
        new_rule.id = uuid::Uuid::new_v4().to_string();
    }
    // TODO: Save to database
    Ok(new_rule)
}

#[tauri::command]
pub fn get_reminder_rules() -> Result<Vec<ReminderRule>, String> {
    // TODO: Load from database
    Ok(Vec::new())
}

#[tauri::command]
pub fn update_reminder_rule(rule: ReminderRule) -> Result<(), String> {
    info!("Updating reminder rule: {}", rule.id);
    // TODO: Update in database
    Ok(())
}

#[tauri::command]
pub fn delete_reminder_rule(rule_id: String) -> Result<(), String> {
    info!("Deleting reminder rule: {}", rule_id);
    // TODO: Delete from database
    Ok(())
}

#[tauri::command]
pub fn get_reminder_records(_user_id: String) -> Result<Vec<ReminderRecord>, String> {
    // TODO: Load from database
    Ok(Vec::new())
}

#[tauri::command]
pub fn mark_reminder_read(record_id: String) -> Result<(), String> {
    info!("Marking reminder as read: {}", record_id);
    // TODO: Update in database
    Ok(())
}
