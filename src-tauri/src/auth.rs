// Authentication module
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub institution_id: Option<String>,
}

/// Authentication state
pub struct AuthState {
    pub current_user: Mutex<Option<User>>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            current_user: Mutex::new(None),
        }
    }
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub user: Option<User>,
    pub token: Option<String>,
    pub message: String,
}

/// Tauri commands for authentication

#[tauri::command]
pub fn login(request: LoginRequest) -> LoginResponse {
    info!("Login attempt for user: {}", request.username);

    // TODO: Validate credentials against backend
    // For now, simulate successful login
    if request.username.is_empty() || request.password.is_empty() {
        return LoginResponse {
            success: false,
            user: None,
            token: None,
            message: "用户名和密码不能为空".to_string(),
        };
    }

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: request.username.clone(),
        display_name: request.username.clone(),
        role: "user".to_string(),
        institution_id: None,
    };

    info!("Login successful for user: {}", request.username);

    LoginResponse {
        success: true,
        user: Some(user),
        token: Some("mock-jwt-token".to_string()),
        message: "登录成功".to_string(),
    }
}

#[tauri::command]
pub fn logout() -> Result<(), String> {
    info!("User logged out");
    Ok(())
}

#[tauri::command]
pub fn get_current_user() -> Option<User> {
    // TODO: Get current user from auth state
    None
}
