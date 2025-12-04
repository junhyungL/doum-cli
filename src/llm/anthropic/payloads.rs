use crate::llm::LLMMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AnthropicRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<LLMMessage>,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicResponse {
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ContentBlock {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub block_type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicError {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}
