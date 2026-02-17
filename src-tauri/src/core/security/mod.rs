// Security module - Risk assessment and operation control
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{error, info};

/// Risk level enum
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

/// Risk rule pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskPattern {
    pub action_type: String,
    pub target_match: String,
    pub reason: String,
}

/// Risk rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub patterns: Vec<RiskPattern>,
    pub requires_confirmation: bool,
    pub allowed_on_whitelist: bool,
}

/// Whitelist configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Whitelist {
    pub enabled: bool,
    pub domains: Vec<String>,
}

/// Risk rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRulesConfig {
    pub version: String,
    pub rules: Vec<RiskRule>,
    pub whitelist: Whitelist,
}

/// Risk assessment engine
pub struct RiskEngine {
    config: RiskRulesConfig,
}

impl RiskEngine {
    /// Load risk rules from JSON file
    pub fn load() -> Self {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("core")
            .join("security")
            .join("risk_rules.json");

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => {
                            info!("Loaded risk rules from {:?}", config_path);
                            return Self { config };
                        }
                        Err(e) => {
                            error!("Failed to parse risk rules: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read risk rules: {}", e);
                }
            }
        }

        // Return default config if file not found or parse error
        info!("Using default risk rules");
        Self::default()
    }

    /// Assess risk for an action
    pub fn assess(&self, action_type: &str, target: &str) -> RiskAssessment {
        // Check whitelist first
        if self.config.whitelist.enabled {
            if self.is_whitelisted(target) {
                return RiskAssessment {
                    level: RiskLevel::Low,
                    reason: "Target is on whitelist".to_string(),
                    requires_confirmation: false,
                };
            }
        }

        // Check rules in order (High -> Medium -> Low)
        for rule in &self.config.rules {
            for pattern in &rule.patterns {
                if self.matches_pattern(action_type, target, pattern) {
                    let level = match rule.risk_level.as_str() {
                        "High" => RiskLevel::High,
                        "Medium" => RiskLevel::Medium,
                        _ => RiskLevel::Low,
                    };

                    return RiskAssessment {
                        level,
                        reason: pattern.reason.clone(),
                        requires_confirmation: rule.requires_confirmation,
                    };
                }
            }
        }

        // Default to medium risk if no match
        RiskAssessment {
            level: RiskLevel::Medium,
            reason: "No specific rule matched, assuming medium risk".to_string(),
            requires_confirmation: false,
        }
    }

    /// Check if target is whitelisted
    fn is_whitelisted(&self, target: &str) -> bool {
        for domain in &self.config.whitelist.domains {
            if domain.starts_with("*.") {
                // Wildcard match
                let suffix = &domain[2..];
                if target.contains(suffix) {
                    return true;
                }
            } else if target.contains(domain) {
                return true;
            }
        }
        false
    }

    /// Check if action matches a pattern
    fn matches_pattern(&self, action_type: &str, target: &str, pattern: &RiskPattern) -> bool {
        // Check action type
        if !pattern.action_type.is_empty() && pattern.action_type != "*" {
            if !action_type.to_lowercase().contains(&pattern.action_type.to_lowercase()) {
                return false;
            }
        }

        // Check target match
        if pattern.target_match != "*" {
            let targets: Vec<&str> = pattern.target_match.split(',').collect();
            let matches = targets.iter().any(|t| {
                let t = t.trim();
                if t.starts_with('*') && t.ends_with('*') {
                    // Contains match
                    let inner = &t[1..t.len()-1];
                    target.to_lowercase().contains(&inner.to_lowercase())
                } else if t.starts_with('*') {
                    // Ends with
                    let suffix = &t[1..];
                    target.to_lowercase().ends_with(&suffix.to_lowercase())
                } else if t.ends_with('*') {
                    // Starts with
                    let prefix = &t[..t.len()-1];
                    target.to_lowercase().starts_with(&prefix.to_lowercase())
                } else {
                    // Exact match
                    target.to_lowercase() == t.to_lowercase()
                }
            });

            if !matches {
                return false;
            }
        }

        true
    }

    /// Get default configuration
    fn default() -> Self {
        Self {
            config: RiskRulesConfig {
                version: "1.0".to_string(),
                rules: vec![
                    RiskRule {
                        id: "high_risk".to_string(),
                        name: "High Risk".to_string(),
                        description: "High risk operations".to_string(),
                        risk_level: "High".to_string(),
                        patterns: vec![
                            RiskPattern {
                                action_type: "submit".to_string(),
                                target_match: "*".to_string(),
                                reason: "Form submission may change data".to_string(),
                            },
                            RiskPattern {
                                action_type: "delete".to_string(),
                                target_match: "*".to_string(),
                                reason: "Delete is irreversible".to_string(),
                            },
                        ],
                        requires_confirmation: true,
                        allowed_on_whitelist: false,
                    },
                ],
                whitelist: Whitelist {
                    enabled: true,
                    domains: vec!["localhost".to_string()],
                },
            },
        }
    }
}

/// Tauri command for risk assessment
#[tauri::command]
pub fn assess_risk(action_type: String, target: String) -> RiskAssessment {
    let engine = RiskEngine::load();
    engine.assess(&action_type, &target)
}
