use crate::system::error::Result;
use crate::system::LLMConfig;
use serde::{Deserialize, Serialize};

/// 메시지 역할
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// LLM 요청문
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub system: String,
    pub messages: Vec<Message>,
    pub use_websearch: bool,
}

/// LLM 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    /// 사용자 메시지 생성
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// 어시스턴트 메시지 생성
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// LLM 클라이언트 trait
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    /// 채팅 완성 요청
    async fn generate(&self, request: LLMRequest) -> Result<String>;
    
    /// API 키 및 설정 검증
    async fn verify(&self) -> Result<bool> {
        // 간단한 테스트 메시지 전송
        let test_request = LLMRequest {
            system: "This is a test, please respond shortly.".to_string(),
            messages: vec![Message::user("Hello")],
            use_websearch: false,
        };
        
        match self.generate(test_request).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// 설정에 따라 LLM 클라이언트 생성
pub fn create_client(config: &LLMConfig) -> Result<Box<dyn LLMClient>> {
    let provider_config = config.get_current_provider()?;
    
    match provider_config {
        crate::system::ProviderConfig::Openai(openai_config) => {
            let client = crate::llm::openai::OpenAIClient::new(
                openai_config.clone(), 
                config.timeout
            )?;
            Ok(Box::new(client))
        }
        crate::system::ProviderConfig::Anthropic(anthropic_config) => {
            let client = crate::llm::anthropic::AnthropicClient::new(
                anthropic_config.clone(),
                config.timeout
            )?;
            Ok(Box::new(client))
        }
    }
}
