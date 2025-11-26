use crate::llm::{AnthropicClient, AnthropicConfig, AnthropicSecret, OpenAIClient, OpenAIConfig};
use crate::system::{DoumError, DoumResult};
use crate::{llm::OpenAISecret, system::LLMConfig};
use serde::{Deserialize, Serialize};

/// LLM Message Role
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// LLM Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub system: String,
    pub messages: Vec<Message>,
    pub use_websearch: bool,
}

/// LLM Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    /// create user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// create assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// LLM Client Trait
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    /// Generate response from LLM
    async fn generate(&self, request: LLMRequest) -> DoumResult<String>;

    /// Verify LLM client connectivity
    async fn verify(&self) -> DoumResult<bool> {
        let test_request = LLMRequest {
            system: "This is a test, please respond shortly.".to_string(),
            messages: vec![Message::user("Hello")],
            use_websearch: false,
        };

        match self.generate(test_request).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Create LLM client based on configuration
pub fn create_client(config: &LLMConfig) -> DoumResult<Box<dyn LLMClient>> {
    let provider = &config.provider;

    match provider.as_str() {
        "openai" => {
            let secret = OpenAISecret::load().map_err(|e| DoumError::Config(e.to_string()))?;

            let openai_config = OpenAIConfig {
                model: config.model.clone(),
                api_key: secret.api_key,
                organization: secret.organization,
                project: secret.project,
            };
            let client = OpenAIClient::new(openai_config, config.timeout)?;
            Ok(Box::new(client))
        }
        "anthropic" => {
            let secret = AnthropicSecret::load().map_err(|e| DoumError::Config(e.to_string()))?;

            let anthropic_config = AnthropicConfig {
                model: config.model.clone(),
                api_key: secret.api_key,
            };
            let client = AnthropicClient::new(anthropic_config, config.timeout)?;
            Ok(Box::new(client))
        }
        _ => Err(crate::system::DoumError::Config(format!(
            "Unknown provider: {}",
            provider
        ))),
    }
}
