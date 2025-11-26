use crate::llm::client::{LLMClient, LLMRequest};
use crate::llm::openai::payloads::{
    OpenAIConfig, OpenAIError, OpenAIOutput, OpenAIRequest, OpenAIResponse, OpenAIWebSearchTool,
};
use crate::system::error::{DoumError, DoumResult};
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
    pub fn new(config: OpenAIConfig, timeout: u64) -> DoumResult<Self> {
        if config.api_key.is_empty() {
            return Err(DoumError::InvalidConfig(
                "OpenAI API key is not set. Please configure it in the interactive config menu (doum config).".to_string()
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
impl LLMClient for OpenAIClient {
    async fn generate(&self, request: LLMRequest) -> DoumResult<String> {
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
                DoumError::Timeout
            } else if e.is_connect() {
                DoumError::LLM("Failed to connect to OpenAI API".to_string())
            } else {
                DoumError::LLM(format!("Failed to send request to OpenAI API: {}", e))
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
                return Err(DoumError::LLM(format!(
                    "OpenAI API Error ({}): {}",
                    status, openai_error.error.message
                )));
            }

            return Err(DoumError::LLM(format!(
                "OpenAI API Error: {} - {}",
                status, error_text
            )));
        }

        // Parse response body
        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| DoumError::Parse(format!("Failed to parse OpenAI response: {}", e)))?;

        // Extract message content
        for output in openai_response.output {
            if let OpenAIOutput::Message { content } = output
                && let Some(first_content) = content.first()
            {
                return Ok(first_content.text.clone());
            }
        }

        Err(DoumError::Parse(
            "No content in OpenAI response".to_string(),
        ))
    }
}
