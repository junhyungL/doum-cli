use crate::llm::{
    AnthropicClient, AnthropicConfig, AnthropicSecret, OpenAIClient, OpenAIConfig, Provider,
};
use crate::system::SecretManager;
use crate::{llm::OpenAISecret, system::LLMConfig};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// LLM Message Role
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMRole {
    User,
    Assistant,
}

/// LLM Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub system: String,
    pub messages: Vec<LLMMessage>,
}

/// LLM Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: LLMRole,
    pub content: String,
}

impl LLMMessage {
    /// create user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: LLMRole::User,
            content: content.into(),
        }
    }

    /// create assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: LLMRole::Assistant,
            content: content.into(),
        }
    }
}

/// LLM Client Trait
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    /// Generate response from LLM
    async fn generate(&self, request: LLMRequest) -> Result<String>;
}

/// Verify LLM configuration without creating a persistent client
pub async fn verify_client(config: &LLMConfig) -> Result<bool> {
    let client = create_client(config)?;
    let request = LLMRequest {
        system: "This is a test, please respond shortly.".to_string(),
        messages: vec![LLMMessage::user("Hello")],
    };

    match client.generate(request).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Create LLM client based on configuration
pub fn create_client(config: &LLMConfig) -> Result<Box<dyn LLMClient>> {
    match config.provider {
        Provider::OpenAI => {
            let secret: OpenAISecret =
                SecretManager::load(&config.provider).context("Failed to load OpenAI secret")?;

            let openai_config = OpenAIConfig {
                model: config.model.clone(),
                api_key: secret.api_key,
                organization: secret.organization,
                project: secret.project,
            };
            let client = OpenAIClient::new(openai_config, config.timeout)?;
            Ok(Box::new(client))
        }
        Provider::Anthropic => {
            let secret: AnthropicSecret =
                SecretManager::load(&config.provider).context("Failed to load Anthropic secret")?;

            let anthropic_config = AnthropicConfig {
                model: config.model.clone(),
                api_key: secret.api_key,
            };
            let client = AnthropicClient::new(anthropic_config, config.timeout)?;
            Ok(Box::new(client))
        }
    }
}
