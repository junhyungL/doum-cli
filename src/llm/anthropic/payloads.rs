use serde::{Deserialize, Serialize};

/// Anthropic 프로바이더 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub model: String,
    pub api_key: String,
}

/// Anthropic API 요청 구조체
#[derive(Debug, Serialize)]
pub(crate) struct AnthropicRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<crate::llm::Message>,
    pub max_tokens: u32,
}

/// Anthropic API 응답 구조체
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

/// Anthropic API 에러 응답
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
