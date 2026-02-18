// AI module - FastGPT integration
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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

/// FastGPT API request structure
#[derive(Debug, Serialize)]
struct FastGPTRequest {
    query: String,
    history: Vec<AiMessage>,
}

/// FastGPT API response structure
#[derive(Debug, Deserialize)]
struct FastGPTResponse {
    #[serde(rename = "data")]
    data: Option<FastGPTData>,
}

#[derive(Debug, Deserialize)]
struct FastGPTData {
    #[serde(rename = "content")]
    content: String,
}

pub struct AiClient {
    pub api_endpoint: String,
    pub api_key: String,
    http_client: Client,
}

impl AiClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            api_endpoint: endpoint,
            api_key,
            http_client: Client::new(),
        }
    }

    /// Send chat request to FastGPT
    pub async fn chat(&self, request: AiRequest) -> Result<AiResponse, String> {
        let url = format!("{}/api/v1/chat/completion", self.api_endpoint);

        let last_message = request.messages.last()
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let fastgpt_request = FastGPTRequest {
            query: last_message,
            history: request.messages[..request.messages.len().saturating_sub(1)].to_vec(),
        };

        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&fastgpt_request)
            .send()
            .await
            .map_err(|e| {
                error!("FastGPT API request failed: {}", e);
                e.to_string()
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("FastGPT API error: {} - {}", status, error_text);
            return Err(format!("API error: {}", status));
        }

        let fastgpt_response: FastGPTResponse = response.json().await
            .map_err(|e| e.to_string())?;

        let content = fastgpt_response.data
            .map(|d| d.content)
            .unwrap_or_default();

        // Parse actions from response
        let actions = Self::parse_actions(&content);

        info!("Received AI response with {} actions", actions.len());

        Ok(AiResponse {
            content,
            actions,
        })
    }

    /// Parse actions from AI response
    fn parse_actions(content: &str) -> Vec<AiAction> {
        // Simple action parsing - in production, this would be more sophisticated
        let actions = Vec::new();

        // Look for action patterns like [action:click:value]
        if content.contains("[action:") {
            // Simplified parsing - return empty for now
            // Real implementation would parse structured response
        }

        actions
    }

    /// Get intent from user message
    pub async fn parse_intent(&self, message: &str) -> Result<String, String> {
        // Simplified intent parsing
        // In production, this would call FastGPT with specific prompt
        Ok(message.to_string())
    }
}

/// Risk levels for AI operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub level: RiskLevel,
    pub reason: String,
    pub requires_confirmation: bool,
}

impl RiskLevel {
    pub fn from_action(action_type: &str, target: &str) -> Self {
        // High risk: form submissions, data deletion, financial operations
        let high_risk_patterns = ["submit", "delete", "payment", "transfer", "confirm"];

        // Medium risk: navigation to external sites, file downloads
        let medium_risk_patterns = ["download", "navigate_external", "open_new_tab"];

        let lower_action = action_type.to_lowercase();
        let lower_target = target.to_lowercase();

        if high_risk_patterns.iter().any(|p| lower_action.contains(p)) {
            RiskLevel::High
        } else if medium_risk_patterns.iter().any(|p| lower_action.contains(p) || lower_target.contains(p)) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }
}
