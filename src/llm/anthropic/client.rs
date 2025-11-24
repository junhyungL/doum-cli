use crate::llm::anthropic::payloads::{
    AnthropicConfig, AnthropicError, AnthropicRequest, AnthropicResponse,
};
use crate::llm::client::{LLMClient, LLMRequest};
use crate::system::error::{DoumError, Result};
use reqwest::Client;
use std::time::Duration;

/// Anthropic 클라이언트
pub struct AnthropicClient {
    http_client: Client,
    config: AnthropicConfig,
}

impl AnthropicClient {
    /// Anthropic API 엔드포인트
    const API_URL: &'static str = "https://api.anthropic.com/v1/messages";
    const API_VERSION: &'static str = "2023-06-01";

    /// 새 Anthropic 클라이언트 생성
    pub fn new(config: AnthropicConfig, timeout: u64) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(DoumError::InvalidConfig(
                "Anthropic API key is not set. Please configure it in the interactive config menu (doum config).".to_string()
            ));
        }

        let http_client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|e| DoumError::LLM(format!("HTTP 클라이언트 생성 실패: {}", e)))?;

        Ok(Self {
            http_client,
            config,
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn generate(&self, request: LLMRequest) -> Result<String> {
        // 요청 본문 구성
        let request_body = AnthropicRequest {
            model: self.config.model.clone(),
            system: Some(request.system),
            messages: request.messages,
            max_tokens: 4096,
        };

        // API 요청
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
                    DoumError::LLM("네트워크 연결 실패. 인터넷 연결을 확인하세요.".to_string())
                } else {
                    DoumError::LLM(format!("API 요청 실패: {}", e))
                }
            })?;

        // HTTP 상태 코드 확인
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "알 수 없는 에러".to_string());

            // Anthropic 에러 응답 파싱 시도
            if let Ok(anthropic_error) = serde_json::from_str::<AnthropicError>(&error_text) {
                return Err(DoumError::LLM(format!(
                    "Anthropic API 에러 ({}): {}",
                    status, anthropic_error.error.message
                )));
            }

            return Err(DoumError::LLM(format!(
                "API 요청 실패 ({}): {}",
                status, error_text
            )));
        }

        // 응답 파싱
        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| DoumError::Parse(format!("API 응답 파싱 실패: {}", e)))?;

        // 첫 번째 content block의 text 추출
        anthropic_response
            .content
            .first()
            .map(|block| block.text.clone())
            .ok_or_else(|| DoumError::Parse("API 응답에 컨텐츠가 없습니다".to_string()))
    }
}
