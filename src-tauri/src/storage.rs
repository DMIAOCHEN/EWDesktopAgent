// Storage module - SQLite database management
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub department: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantSession {
    pub id: String,
    pub user_id: String,
    pub messages: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub theme: String,
    pub voice_enabled: bool,
    pub recent_systems: Vec<String>,
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                department TEXT,
                role TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS business_systems (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                enabled INTEGER DEFAULT 1,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS assistant_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                messages TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS reminder_rules (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                trigger_config TEXT,
                content TEXT,
                target_url TEXT,
                enabled INTEGER DEFAULT 1,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS audit_logs (
                id TEXT PRIMARY KEY,
                user_id TEXT,
                action TEXT NOT NULL,
                details TEXT,
                risk_level TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS user_preferences (
                user_id TEXT PRIMARY KEY,
                preferences TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS behavior_logs (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                action_type TEXT NOT NULL,
                target TEXT,
                metadata TEXT,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_audit_logs_user ON audit_logs(user_id);
            CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at);
            CREATE INDEX IF NOT EXISTS idx_behavior_logs_user ON behavior_logs(user_id);
            CREATE INDEX IF NOT EXISTS idx_behavior_logs_created ON behavior_logs(created_at);
            ",
        )?;
        info!("Database schema initialized");
        Ok(())
    }

    /// Log an audit entry
    pub fn log_audit(&self, user_id: Option<&str>, action: &str, details: &str, risk_level: &str) -> Result<(), String> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO audit_logs (id, user_id, action, details, risk_level, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&id, &user_id, &action, &details, &risk_level, &created_at),
        ).map_err(|e| e.to_string())?;

        info!("Audit log: {} - {} by {:?}", action, details, user_id);
        Ok(())
    }

    /// Query audit logs
    pub fn query_audit_logs(&self, user_id: Option<&str>, limit: i32) -> Result<Vec<AuditLogEntry>, String> {
        let mut stmt = if let Some(uid) = user_id {
            self.conn.prepare(
                "SELECT id, user_id, action, details, risk_level, created_at
                 FROM audit_logs WHERE user_id = ?1 ORDER BY created_at DESC LIMIT ?2"
            ).map_err(|e| e.to_string())?

        } else {
            self.conn.prepare(
                "SELECT id, user_id, action, details, risk_level, created_at
                 FROM audit_logs ORDER BY created_at DESC LIMIT ?1"
            ).map_err(|e| e.to_string())?
        };

        let logs = stmt.query_map(if let Some(uid) = user_id { [uid, &limit.to_string()] } else { [&limit.to_string()] }, |row| {
            Ok(AuditLogEntry {
                id: row.get(0)?,
                user_id: row.get(1)?,
                action: row.get(2)?,
                details: row.get(3)?,
                risk_level: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

        Ok(logs)
    }
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub details: String,
    pub risk_level: String,
    pub created_at: String,
}

/// Database state managed by Tauri
pub struct StorageState {
    pub db: Mutex<Option<Database>>,
}

impl Default for StorageState {
    fn default() -> Self {
        Self {
            db: Mutex::new(None),
        }
    }
}

fn get_db_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("EWDesktopAgent")
        .join("data.db")
}

/// Tauri commands for database operations

#[tauri::command]
pub fn init_database() -> Result<String, String> {
    let db_path = get_db_path();

    // Create directory if needed
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;
    db.init_schema().map_err(|e| e.to_string())?;

    info!("Database initialized at {:?}", db_path);
    Ok(db_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn save_session(session: AssistantSession) -> Result<(), String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;

    db.conn.execute(
        "INSERT OR REPLACE INTO assistant_sessions (id, user_id, messages, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        (&session.id, &session.user_id, &session.messages, &session.created_at),
    ).map_err(|e| e.to_string())?;

    info!("Saved session: {}", session.id);
    Ok(())
}

#[tauri::command]
pub fn load_session(session_id: String) -> Result<Option<AssistantSession>, String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;

    let mut stmt = db.conn
        .prepare("SELECT id, user_id, messages, created_at FROM assistant_sessions WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row([&session_id], |row| {
        Ok(AssistantSession {
            id: row.get(0)?,
            user_id: row.get(1)?,
            messages: row.get(2)?,
            created_at: row.get(3)?,
        })
    });

    match result {
        Ok(session) => Ok(Some(session)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn save_user_preferences(user_id: String, preferences: UserPreferences) -> Result<(), String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;

    let prefs_json = serde_json::to_string(&preferences).map_err(|e| e.to_string())?;

    db.conn.execute(
        "INSERT OR REPLACE INTO user_preferences (user_id, preferences, updated_at)
         VALUES (?1, ?2, datetime('now'))",
        (&user_id, &prefs_json),
    ).map_err(|e| e.to_string())?;

    info!("Saved preferences for user: {}", user_id);
    Ok(())
}

#[tauri::command]
pub fn load_user_preferences(user_id: String) -> Result<Option<UserPreferences>, String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;

    let mut stmt = db.conn
        .prepare("SELECT preferences FROM user_preferences WHERE user_id = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row([&user_id], |row| {
        let prefs_json: String = row.get(0)?;
        Ok(prefs_json)
    });

    match result {
        Ok(prefs_json) => {
            let prefs: UserPreferences = serde_json::from_str(&prefs_json).map_err(|e| e.to_string())?;
            Ok(Some(prefs))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Tauri commands for audit logging

#[tauri::command]
pub fn log_audit(user_id: Option<String>, action: String, details: String, risk_level: String) -> Result<(), String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;
    db.log_audit(user_id.as_deref(), &action, &details, &risk_level)
}

#[tauri::command]
pub fn query_audit_logs(user_id: Option<String>, limit: i32) -> Result<Vec<AuditLogEntry>, String> {
    let db_path = get_db_path();
    let db = Database::new(db_path.to_str().unwrap()).map_err(|e| e.to_string())?;
    db.query_audit_logs(user_id.as_deref(), limit)
}
