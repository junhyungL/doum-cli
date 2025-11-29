use crate::llm::{AnthropicSecret, OpenAISecret};
use crate::system::SecretManager;
use anyhow::Result;
use std::collections::HashMap;

pub struct SecretField {
    pub name: String,
    pub label: String,
    pub required: bool,
    pub is_password: bool,
}

pub struct SecretConfigData {
    pub provider: String,
    pub fields: Vec<SecretField>,
}

pub struct SecretService;

impl SecretService {
    /// Get secret configuration for a provider
    pub fn get_provider_config(provider: &str) -> Result<SecretConfigData> {
        match provider {
            "openai" => Ok(SecretConfigData {
                provider: "openai".to_string(),
                fields: vec![
                    SecretField {
                        name: "api_key".to_string(),
                        label: "OpenAI API Key (required)".to_string(),
                        required: true,
                        is_password: true,
                    },
                    SecretField {
                        name: "organization".to_string(),
                        label: "Organization ID (optional, press Enter to skip)".to_string(),
                        required: false,
                        is_password: false,
                    },
                    SecretField {
                        name: "project".to_string(),
                        label: "Project ID (optional, press Enter to skip)".to_string(),
                        required: false,
                        is_password: false,
                    },
                ],
            }),
            "anthropic" => Ok(SecretConfigData {
                provider: "anthropic".to_string(),
                fields: vec![SecretField {
                    name: "api_key".to_string(),
                    label: "Anthropic API Key (required)".to_string(),
                    required: true,
                    is_password: true,
                }],
            }),
            _ => anyhow::bail!("Unknown provider: {}", provider),
        }
    }

    /// Save secrets for a provider
    pub fn save_secrets(provider: &str, values: HashMap<String, String>) -> Result<()> {
        match provider {
            "openai" => {
                let secret = OpenAISecret {
                    api_key: values
                        .get("api_key")
                        .ok_or_else(|| anyhow::anyhow!("API key is required"))?
                        .clone(),
                    organization: values
                        .get("organization")
                        .cloned()
                        .filter(|s| !s.is_empty()),
                    project: values.get("project").cloned().filter(|s| !s.is_empty()),
                };
                SecretManager::save("openai", &secret)
            }
            "anthropic" => {
                let secret = AnthropicSecret {
                    api_key: values
                        .get("api_key")
                        .ok_or_else(|| anyhow::anyhow!("API key is required"))?
                        .clone(),
                };
                SecretManager::save("anthropic", &secret)
            }
            _ => anyhow::bail!("Unknown provider: {}", provider),
        }
    }

    /// List all available providers
    pub fn list_providers() -> Vec<String> {
        vec!["openai".to_string(), "anthropic".to_string()]
    }
}
