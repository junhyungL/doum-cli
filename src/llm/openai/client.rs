use crate::system::error::{DoumError, Result};
use crate::llm::client::{LLMClient, LLMRequest};
use crate::llm::openai::payloads::{
    OpenAIConfig, OpenAIRequest, OpenAIResponse, OpenAIOutput, OpenAIError, OpenAIWebSearchTool,
};
use reqwest::Client;
use std::time::Duration;

/// OpenAI 클라이언트
pub struct OpenAIClient {
    http_client: Client,
    config: OpenAIConfig,
}

impl OpenAIClient {
    /// OpenAI API 엔드포인트
    const API_URL: &'static str = "https://api.openai.com/v1/responses";
    
    /// 새 OpenAI 클라이언트 생성
    pub fn new(config: OpenAIConfig, timeout: u64) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(DoumError::InvalidConfig(
                "OpenAI API key is not set. Please configure it in the interactive config menu (doum config).".to_string()
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
impl LLMClient for OpenAIClient {
    async fn generate(&self, request: LLMRequest) -> Result<String> {
        // OpenAI API 요청 생성
        let openai_request = OpenAIRequest {
            model: self.config.model.clone(),
            instructions: Some(request.system),
            input: request.messages,
            tools: vec![
                OpenAIWebSearchTool {
                    tool_type: "web_search".to_string(),
                }
            ].into(),
        };

        // API 요청 빌더
        let mut builder = self.http_client
            .post(Self::API_URL)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json");
        
        // 선택적 헤더 추가
        if let Some(ref org) = self.config.organization {
            builder = builder.header("OpenAI-Organization", org);
        }
        if let Some(ref proj) = self.config.project {
            builder = builder.header("OpenAI-Project", proj);
        }

        let response = builder
            .json(&openai_request)
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
            let error_text = response.text().await
                .unwrap_or_else(|_| "알 수 없는 에러".to_string());
            
            // OpenAI 에러 응답 파싱 시도
            if let Ok(openai_error) = serde_json::from_str::<OpenAIError>(&error_text) {
                return Err(DoumError::LLM(format!(
                    "OpenAI API 에러 ({}): {}",
                    status,
                    openai_error.error.message
                )));
            }
            
            return Err(DoumError::LLM(format!(
                "API 요청 실패 ({}): {}",
                status,
                error_text
            )));
        }

        // 응답 본문을 먼저 텍스트로 읽어서 로깅
        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| DoumError::Parse(format!("응답 본문 읽기 실패: {}", e)))?;

        // message 타입의 output에서 content 추출
        for output in openai_response.output {
            if let OpenAIOutput::Message { content } = output {
                if let Some(first_content) = content.first() {
                    return Ok(first_content.text.clone());
                }
            }
        }
        
        Err(DoumError::Parse(
            "API 응답에 메시지 내용이 없습니다".to_string()
        ))
    }
}
