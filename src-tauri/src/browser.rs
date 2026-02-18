// Browser module - WebView2 management
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tracing::info;

/// Maximum memory per tab (200MB)
const MAX_TAB_MEMORY_MB: u64 = 200;

/// Browser tab representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub is_active: bool,
}

/// Tab manager for handling multiple browser tabs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabManager {
    pub tabs: Vec<BrowserTab>,
    pub active_tab_id: Option<String>,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
        }
    }

    /// Create a new browser tab
    pub fn create_tab(&mut self, url: String) -> BrowserTab {
        let tab = BrowserTab {
            id: uuid::Uuid::new_v4().to_string(),
            url: url.clone(),
            title: String::from("New Tab"),
            is_active: true,
        };
        self.tabs.push(tab.clone());
        self.active_tab_id = Some(tab.id.clone());
        info!("Created new tab: {} for URL: {}", tab.id, url);
        tab
    }

    /// Close a tab by ID
    pub fn close_tab(&mut self, tab_id: &str) -> Option<BrowserTab> {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            let tab = self.tabs.remove(pos);
            info!("Closed tab: {}", tab_id);

            // If closed tab was active, activate another
            if tab.is_active && !self.tabs.is_empty() {
                self.tabs.last_mut().map(|t| t.is_active = true);
                self.active_tab_id = self.tabs.last().map(|t| t.id.clone());
            }
            Some(tab)
        } else {
            None
        }
    }

    /// Set active tab
    pub fn set_active_tab(&mut self, tab_id: &str) -> bool {
        for tab in &mut self.tabs {
            tab.is_active = tab.id == tab_id;
            if tab.is_active {
                self.active_tab_id = Some(tab.id.clone());
            }
        }
        self.tabs.iter().any(|t| t.id == tab_id)
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}

/// Browser state managed by Tauri
pub struct BrowserState {
    pub tab_manager: Mutex<TabManager>,
}

impl Default for BrowserState {
    fn default() -> Self {
        Self {
            tab_manager: Mutex::new(TabManager::new()),
        }
    }
}

/// Tauri commands for browser management

#[tauri::command]
pub fn create_browser_tab(
    state: tauri::State<BrowserState>,
    url: String,
) -> Result<BrowserTab, String> {
    let mut manager = state.tab_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.create_tab(url))
}

#[tauri::command]
pub fn close_browser_tab(
    state: tauri::State<BrowserState>,
    tab_id: String,
) -> Result<Option<BrowserTab>, String> {
    let mut manager = state.tab_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.close_tab(&tab_id))
}

#[tauri::command]
pub fn set_active_tab(
    state: tauri::State<BrowserState>,
    tab_id: String,
) -> Result<bool, String> {
    let mut manager = state.tab_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.set_active_tab(&tab_id))
}

#[tauri::command]
pub fn get_tabs(state: tauri::State<BrowserState>) -> Result<Vec<BrowserTab>, String> {
    let manager = state.tab_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.tabs.clone())
}

#[tauri::command]
pub fn navigate_tab(
    app: AppHandle,
    state: tauri::State<BrowserState>,
    tab_id: String,
    url: String,
) -> Result<(), String> {
    let mut manager = state.tab_manager.lock().map_err(|e| e.to_string())?;

    // Update tab URL
    for tab in &mut manager.tabs {
        if tab.id == tab_id {
            tab.url = url.clone();
            break;
        }
    }

    // Emit event to frontend to update the webview
    app.emit("browser-navigate", serde_json::json!({
        "tabId": tab_id,
        "url": url
    })).map_err(|e: tauri::Error| e.to_string())?;

    info!("Navigating tab {} to {}", tab_id, url);
    Ok(())
}

/// Initialize browser state
pub fn init_browser_state() -> BrowserState {
    BrowserState::default()
}
