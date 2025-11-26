use crate::llm::anthropic::payloads::{
    AnthropicConfig, AnthropicError, AnthropicRequest, AnthropicResponse,
};
use crate::llm::client::{LLMClient, LLMRequest};
use crate::system::error::{DoumError, DoumResult};
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
    pub fn new(config: AnthropicConfig, timeout: u64) -> DoumResult<Self> {
        if config.api_key.is_empty() {
            return Err(DoumError::InvalidConfig(
                "Anthropic API key is not set. Please configure it in the interactive config menu (doum config).".to_string()
            ));
        }

        let http_client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|e| DoumError::LLM(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            http_client,
            config,
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn generate(&self, request: LLMRequest) -> DoumResult<String> {
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
                    DoumError::Timeout
                } else if e.is_connect() {
                    DoumError::LLM("Failed to connect to Anthropic API".to_string())
                } else {
                    DoumError::LLM(format!("Failed to send request to Anthropic API: {}", e))
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
                return Err(DoumError::LLM(format!(
                    "Anthropic API Error ({}): {}",
                    status, anthropic_error.error.message
                )));
            }

            return Err(DoumError::LLM(format!(
                "Anthropic API Error: {} - {}",
                status, error_text
            )));
        }

        // Parse response body
        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| DoumError::Parse(format!("Failed to parse Anthropic response: {}", e)))?;

        // Extract and return the generated content
        anthropic_response
            .content
            .first()
            .map(|block| block.text.clone())
            .ok_or_else(|| DoumError::Parse("No content in Anthropic response".to_string()))
    }
}
