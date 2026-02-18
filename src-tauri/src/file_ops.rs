// File operations module - Local file system management
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::info;

/// File operation permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePermission {
    pub path: String,
    pub allowed_operations: HashSet<String>,
}

/// Permission manager state
pub struct PermissionManager {
    permissions: Mutex<HashSet<String>>,
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self {
            permissions: Mutex::new(HashSet::new()),
        }
    }
}

impl PermissionManager {
    /// Check if operation is allowed
    pub fn check_permission(&self, path: &str, _operation: &str) -> bool {
        let permissions = self.permissions.lock().unwrap();

        // Check exact match
        if permissions.contains(path) {
            return true;
        }

        // Check parent directory
        let path_obj = PathBuf::from(path);
        if let Some(parent) = path_obj.parent() {
            let parent_str = parent.to_string_lossy().to_string();
            if permissions.contains(&parent_str) {
                return true;
            }
        }

        // Default: allow downloads directory
        if let Some(downloads) = dirs::download_dir() {
            if path.starts_with(downloads.to_string_lossy().as_ref()) {
                return true;
            }
        }

        false
    }

    /// Grant permission
    pub fn grant_permission(&self, path: &str) {
        let mut permissions = self.permissions.lock().unwrap();
        permissions.insert(path.to_string());
        info!("Granted permission for: {}", path);
    }

    /// Revoke permission
    pub fn revoke_permission(&self, path: &str) {
        let mut permissions = self.permissions.lock().unwrap();
        permissions.remove(path);
        info!("Revoked permission for: {}", path);
    }
}

/// File operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
    Move { source: String, destination: String },
    Copy { source: String, destination: String },
    Delete { path: String },
    Organize { source_dir: String, rule: String },
}

/// File organization rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeRule {
    pub rule_type: String, // "by_date", "by_type", "by_name"
    pub target_directory: String,
    pub options: Option<OrganizeOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeOptions {
    pub recursive: bool,
    pub extensions: Option<Vec<String>>,
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub files_processed: u32,
}

/// File operations service
pub struct FileService;

impl FileService {
    /// Execute file operation
    pub fn execute_operation(operation: FileOperation) -> Result<OperationResult, String> {
        match operation {
            FileOperation::Move { source, destination } => {
                info!("Moving file: {} -> {}", source, destination);
                fs::rename(&source, &destination).map_err(|e| e.to_string())?;
                Ok(OperationResult {
                    success: true,
                    message: "File moved successfully".to_string(),
                    files_processed: 1,
                })
            }
            FileOperation::Copy { source, destination } => {
                info!("Copying file: {} -> {}", source, destination);
                fs::copy(&source, &destination).map_err(|e| e.to_string())?;
                Ok(OperationResult {
                    success: true,
                    message: "File copied successfully".to_string(),
                    files_processed: 1,
                })
            }
            FileOperation::Delete { path } => {
                info!("Deleting file: {}", path);
                fs::remove_file(&path).map_err(|e| e.to_string())?;
                Ok(OperationResult {
                    success: true,
                    message: "File deleted successfully".to_string(),
                    files_processed: 1,
                })
            }
            FileOperation::Organize { source_dir, rule } => {
                Self::organize_files(&source_dir, &rule)
            }
        }
    }

    /// Organize files by rule
    fn organize_files(source_dir: &str, rule: &str) -> Result<OperationResult, String> {
        let source_path = PathBuf::from(source_dir);
        if !source_path.exists() {
            return Err("Source directory does not exist".to_string());
        }

        let mut files_processed = 0u32;

        match rule {
            "by_date" => {
                // Group files by modification date
                for entry in fs::read_dir(&source_path).map_err(|e| e.to_string())? {
                    let entry = entry.map_err(|e| e.to_string())?;
                    let path = entry.path();

                    if path.is_file() {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                let datetime: chrono::DateTime<chrono::Local> = modified.into();
                                let date_dir = source_path.join(datetime.format("%Y-%m-%d").to_string());
                                fs::create_dir_all(&date_dir).map_err(|e| e.to_string())?;

                                let new_path = date_dir.join(path.file_name().unwrap());
                                fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
                                files_processed += 1;
                            }
                        }
                    }
                }
            }
            "by_type" => {
                // Group files by extension
                for entry in fs::read_dir(&source_path).map_err(|e| e.to_string())? {
                    let entry = entry.map_err(|e| e.to_string())?;
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let type_dir = source_path.join(ext.to_str().unwrap_or("unknown"));
                            fs::create_dir_all(&type_dir).map_err(|e| e.to_string())?;

                            let new_path = type_dir.join(path.file_name().unwrap());
                            fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
                            files_processed += 1;
                        }
                    }
                }
            }
            _ => {
                return Err(format!("Unknown organization rule: {}", rule));
            }
        }

        Ok(OperationResult {
            success: true,
            message: format!("Organized {} files", files_processed),
            files_processed,
        })
    }

    /// Preview organization without executing
    pub fn preview_organization(source_dir: &str, rule: &str) -> Result<Vec<String>, String> {
        let source_path = PathBuf::from(source_dir);
        if !source_path.exists() {
            return Err("Source directory does not exist".to_string());
        }

        let mut preview = Vec::new();
        preview.push(format!("Organization preview for: {} (rule: {})", source_dir, rule));
        preview.push("---".to_string());

        for entry in fs::read_dir(&source_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                preview.push(format!("  - {}", filename));
            }
        }

        Ok(preview)
    }
}

/// Tauri commands for file operations

#[tauri::command]
pub fn execute_file_operation(operation: FileOperation) -> Result<OperationResult, String> {
    FileService::execute_operation(operation)
}

#[tauri::command]
pub fn preview_organization(source_dir: String, rule: String) -> Result<Vec<String>, String> {
    FileService::preview_organization(&source_dir, &rule)
}

#[tauri::command]
pub fn check_file_permission(path: String, operation: String) -> bool {
    let manager = PermissionManager::default();
    manager.check_permission(&path, &operation)
}

#[tauri::command]
pub fn grant_file_permission(path: String) -> Result<(), String> {
    let manager = PermissionManager::default();
    manager.grant_permission(&path);
    Ok(())
}

#[tauri::command]
pub fn revoke_file_permission(path: String) -> Result<(), String> {
    let manager = PermissionManager::default();
    manager.revoke_permission(&path);
    Ok(())
}
