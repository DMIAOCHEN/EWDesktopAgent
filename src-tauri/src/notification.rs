// Notification module - Windows Toast notifications
use serde::{Deserialize, Serialize};
use tracing::info;

/// Notification message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub target_url: Option<String>,
}

/// Notification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    pub success: bool,
    pub message: String,
}

/// Tauri command for showing notifications
/// Note: In production, this would use tauri-plugin-notification

#[tauri::command]
pub fn show_notification(notification: Notification) -> Result<NotificationResult, String> {
    info!("Showing notification: {}", notification.title);

    // TODO: Use tauri-plugin-notification for actual Windows toast
    // For now, log the notification
    println!("[NOTIFICATION] {} - {}", notification.title, notification.body);

    Ok(NotificationResult {
        success: true,
        message: "Notification shown".to_string(),
    })
}

#[tauri::command]
pub fn request_notification_permission() -> Result<bool, String> {
    // TODO: Check/request notification permission
    Ok(true)
}
