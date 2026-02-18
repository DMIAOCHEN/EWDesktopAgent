// Download handler module
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Download request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub filename: Option<String>,
    pub save_directory: Option<String>,
}

/// Download result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub message: String,
}

/// Download service
pub struct DownloadService;

impl DownloadService {
    /// Handle file download
    pub async fn download(request: DownloadRequest) -> Result<DownloadResult, String> {
        info!("Downloading file from: {}", request.url);

        let client = reqwest::Client::new();

        let response = client.get(&request.url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Ok(DownloadResult {
                success: false,
                file_path: None,
                message: format!("Download failed: {}", response.status()),
            });
        }

        // Determine save directory
        let save_dir = if let Some(dir) = &request.save_directory {
            PathBuf::from(dir)
        } else {
            dirs::download_dir().unwrap_or_else(|| PathBuf::from("."))
        };

        // Determine filename
        let filename = if let Some(name) = &request.filename {
            name.clone()
        } else {
            // Try to get filename from header
            response.headers()
                .get("content-disposition")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| {
                    if v.contains("filename=") {
                        v.split("filename=")
                            .nth(1)
                            .map(|s| s.trim_matches('"').to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    // Extract from URL
                    url::Url::parse(&request.url)
                        .ok()
                        .and_then(|u| u.path_segments().map(|s| s.last().map(|p| p.to_string())).flatten())
                        .unwrap_or_else(|| "download".to_string())
                })
        };

        let save_path = save_dir.join(&filename);

        // Create directory if needed
        if let Some(parent) = save_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        // Download and save
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        std::fs::write(&save_path, &bytes).map_err(|e| e.to_string())?;

        info!("File saved to: {:?}", save_path);

        Ok(DownloadResult {
            success: true,
            file_path: Some(save_path.to_string_lossy().to_string()),
            message: format!("File downloaded successfully: {}", filename),
        })
    }
}

/// Tauri command for downloading

#[tauri::command]
pub async fn download_file(request: DownloadRequest) -> Result<DownloadResult, String> {
    DownloadService::download(request).await
}
