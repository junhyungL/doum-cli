use crate::llm::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub model: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct OpenAIRequest {
    pub model: String,
    pub instructions: Option<String>,
    pub input: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAIWebSearchTool>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct OpenAIWebSearchTool {
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIResponse {
    pub output: Vec<OpenAIOutput>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum OpenAIOutput {
    Reasoning {
        #[allow(dead_code)]
        summary: Vec<String>,
    },
    Message {
        content: Vec<OutputContent>,
    },
    WebSearchCall {
        #[allow(dead_code)]
        status: String,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputContent {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIError {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
