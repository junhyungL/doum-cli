use crate::llm::client::{LLMClient, LLMRequest};
use crate::llm::openai::payloads::{
    OpenAIConfig, OpenAIError, OpenAIOutput, OpenAIRequest, OpenAIResponse, OpenAIWebSearchTool,
};
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

/// OpenAI LLM Client
pub struct OpenAIClient {
    http_client: Client,
    config: OpenAIConfig,
}

impl OpenAIClient {
    /// OpenAI API URL
    const API_URL: &'static str = "https://api.openai.com/v1/responses";

    /// Create a new OpenAIClient
    pub fn new(config: OpenAIConfig, timeout: u64) -> Result<Self> {
        if config.api_key.is_empty() {
            anyhow::bail!(
                "OpenAI API key is not set. Please configure it in the interactive config menu (doum config)."
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
impl LLMClient for OpenAIClient {
    async fn generate(&self, request: LLMRequest) -> Result<String> {
        // create OpenAI request payload
        let openai_request = OpenAIRequest {
            model: self.config.model.clone(),
            instructions: Some(request.system),
            input: request.messages,
            tools: vec![OpenAIWebSearchTool {
                tool_type: "web_search".to_string(),
            }]
            .into(),
        };

        // build request with headers
        let mut builder = self
            .http_client
            .post(Self::API_URL)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json");

        // Optional headers
        if let Some(ref org) = self.config.organization {
            builder = builder.header("OpenAI-Organization", org);
        }
        if let Some(ref proj) = self.config.project {
            builder = builder.header("OpenAI-Project", proj);
        }

        // send request
        let response = builder.json(&openai_request).send().await.map_err(|e| {
            if e.is_timeout() {
                anyhow::anyhow!("Request timeout")
            } else if e.is_connect() {
                anyhow::anyhow!("Failed to connect to OpenAI API")
            } else {
                anyhow::anyhow!("Failed to send request to OpenAI API: {}", e)
            }
        })?;

        // check response status
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse OpenAI error format
            if let Ok(openai_error) = serde_json::from_str::<OpenAIError>(&error_text) {
                anyhow::bail!(
                    "OpenAI API Error ({}): {}",
                    status,
                    openai_error.error.message
                );
            }

            anyhow::bail!("OpenAI API Error: {} - {}", status, error_text);
        }

        // Parse response body
        let openai_response: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        // Extract message content
        for output in openai_response.output {
            if let OpenAIOutput::Message { content } = output
                && let Some(first_content) = content.first()
            {
                return Ok(first_content.text.clone());
            }
        }

        anyhow::bail!("No content in OpenAI response")
    }
}
