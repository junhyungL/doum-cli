use crate::llm::anthropic::payloads::{
    AnthropicConfig, AnthropicError, AnthropicRequest, AnthropicResponse,
};
use crate::llm::client::{LLMClient, LLMRequest};
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

/// Anthropic LLM Client
pub struct AnthropicClient {
    http_client: Client,
    config: AnthropicConfig,
}

impl AnthropicClient {
    /// Anthropic API URL and Version
    const API_URL: &'static str = "https://api.anthropic.com/v1/messages";
    const API_VERSION: &'static str = "2023-06-01";

    /// Create a new AnthropicClient
    pub fn new(config: AnthropicConfig, timeout: u64) -> Result<Self> {
        if config.api_key.is_empty() {
            anyhow::bail!(
                "Anthropic API key is not set. Please configure it in the interactive config menu (doum config)."
            );
        }

        let http_client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            http_client,
            config,
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn generate(&self, request: LLMRequest) -> Result<String> {
        let request_body = AnthropicRequest {
            model: self.config.model.clone(),
            system: Some(request.system),
            messages: request.messages,
            max_tokens: 4096,
        };

        let response = self
            .http_client
            .post(Self::API_URL)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", Self::API_VERSION)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    anyhow::anyhow!("Request timeout")
                } else if e.is_connect() {
                    anyhow::anyhow!("Failed to connect to Anthropic API")
                } else {
                    anyhow::anyhow!("Failed to send request to Anthropic API: {}", e)
                }
            })?;

        // Check response status
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse Anthropic error format
            if let Ok(anthropic_error) = serde_json::from_str::<AnthropicError>(&error_text) {
                anyhow::bail!(
                    "Anthropic API Error ({}): {}",
                    status,
                    anthropic_error.error.message
                );
            }

            anyhow::bail!("Anthropic API Error: {} - {}", status, error_text);
        }

        // Parse response body
        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        // Extract and return the generated content
        anthropic_response
            .content
            .first()
            .map(|block| block.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No content in Anthropic response"))
    }
}
