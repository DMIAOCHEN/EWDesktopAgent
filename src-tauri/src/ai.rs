// AI module - FastGPT integration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub messages: Vec<AiMessage>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub actions: Vec<AiAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAction {
    pub action_type: String,
    pub target: String,
    pub value: Option<String>,
}

pub struct AiClient {
    pub api_endpoint: String,
    pub api_key: String,
}

impl AiClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            api_endpoint: endpoint,
            api_key,
        }
    }

    // TODO: Implement FastGPT API calls
    // TODO: Implement intent parsing
    // TODO: Implement action execution
}
