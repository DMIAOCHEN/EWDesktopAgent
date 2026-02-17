// Browser module - WebView2 management
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabManager {
    pub tabs: Vec<BrowserTab>,
    pub active_tab_id: Option<String>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
        }
    }

    pub fn create_tab(&mut self, url: String) -> BrowserTab {
        let tab = BrowserTab {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            title: String::from("New Tab"),
            is_active: true,
        };
        self.tabs.push(tab.clone());
        tab
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
