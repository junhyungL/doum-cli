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

/// Concrete LLM Client enum to support different providers
pub enum Client {
    OpenAI(OpenAIClient),
    Anthropic(AnthropicClient),
}

impl Client {
    /// Generate response from LLM
    pub async fn generate(&self, request: LLMRequest) -> Result<String> {
        match self {
            Client::OpenAI(client) => client.generate(request).await,
            Client::Anthropic(client) => client.generate(request).await,
        }
    }

    /// Generate response with parsing and retry logic (3 attempts for parsing failures only)
    pub async fn generate_with_parser<T, P>(&self, request: LLMRequest, parser: P) -> Result<T>
    where
        P: Fn(&str) -> Result<T>,
    {
        const MAX_RETRIES: u32 = 3;

        // Call LLM once - if this fails, return immediately
        let response = self.generate(request).await?;

        // Retry parsing up to MAX_RETRIES times
        for attempt in 1..=MAX_RETRIES {
            match parser(&response) {
                Ok(parsed) => return Ok(parsed),
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        tracing::warn!(
                            "Parse failed (attempt {}/{}): Retrying...",
                            attempt,
                            MAX_RETRIES
                        );
                        continue;
                    } else {
                        tracing::error!("All parse retry attempts exhausted.");
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }
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
pub fn create_client(config: &LLMConfig) -> Result<Client> {
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
            Ok(Client::OpenAI(client))
        }
        Provider::Anthropic => {
            let secret: AnthropicSecret =
                SecretManager::load(&config.provider).context("Failed to load Anthropic secret")?;

            let anthropic_config = AnthropicConfig {
                model: config.model.clone(),
                api_key: secret.api_key,
            };
            let client = AnthropicClient::new(anthropic_config, config.timeout)?;
            Ok(Client::Anthropic(client))
        }
    }
}
