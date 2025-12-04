use crate::llm::{AnthropicSecret, OpenAISecret, Provider};
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

/// Get secret configuration for a provider
pub fn get_provider_config(provider: &Provider) -> Result<SecretConfigData> {
    match provider {
        Provider::OpenAI => Ok(SecretConfigData {
            provider: provider.as_str().to_string(),
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
        Provider::Anthropic => Ok(SecretConfigData {
            provider: provider.as_str().to_string(),
            fields: vec![SecretField {
                name: "api_key".to_string(),
                label: "Anthropic API Key (required)".to_string(),
                required: true,
                is_password: true,
            }],
        }),
    }
}

/// Save secrets for a provider
pub fn save_secrets(provider: &Provider, values: HashMap<String, String>) -> Result<()> {
    match provider {
        Provider::OpenAI => {
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
            SecretManager::save(provider.as_str(), &secret)
        }
        Provider::Anthropic => {
            let secret = AnthropicSecret {
                api_key: values
                    .get("api_key")
                    .ok_or_else(|| anyhow::anyhow!("API key is required"))?
                    .clone(),
            };
            SecretManager::save(provider.as_str(), &secret)
        }
    }
}
