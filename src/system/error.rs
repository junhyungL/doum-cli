use thiserror::Error;

/// doum-cli에서 사용하는 결과 타입
pub type DoumResult<T> = std::result::Result<T, DoumError>;

/// doum-cli의 모든 에러 타입
#[derive(Error, Debug)]
pub enum DoumError {
    /// 설정 관련 에러
    #[error("Configuration error: {0}")]
    Config(String),

    /// LLM API 관련 에러
    #[error("LLM API error: {0}")]
    LLM(String),

    /// 응답 파싱 에러
    #[error("Parse error: {0}")]
    Parse(String),

    /// IO 에러
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 명령 실행 에러
    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    /// 사용자 취소
    #[error("User cancelled operation")]
    UserCancelled,

    /// 잘못된 설정
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// 타임아웃
    #[error("Request timeout")]
    Timeout,

    /// Reqwest 에러
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// JSON 직렬화/역직렬화 에러
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML 직렬화/역직렬화 에러
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Dialoguer 에러
    #[error("User interaction error: {0}")]
    Dialoguer(String),
}

impl DoumError {
    /// Return a short, user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            DoumError::Config(msg) => {
                format!("Configuration error: {}", msg)
            }
            DoumError::LLM(msg) => {
                if msg.contains("401") || msg.contains("unauthorized") {
                    format!(
                        "Authentication error (LLM API): {}. Check your API key.",
                        msg
                    )
                } else if msg.contains("timeout") || msg.contains("timed out") {
                    "LLM request timed out. Please try again or increase the timeout.".to_string()
                } else if msg.contains("rate limit") || msg.contains("429") {
                    "Rate limit exceeded. Please wait a moment and try again.".to_string()
                } else {
                    format!("LLM API error: {}", msg)
                }
            }
            DoumError::Parse(msg) => {
                format!("Failed to parse LLM response: {}", msg)
            }
            DoumError::Io(err) => {
                format!("I/O error: {}", err)
            }
            DoumError::CommandExecution(msg) => {
                format!("Command execution failed: {}", msg)
            }
            DoumError::UserCancelled => "Operation cancelled by user.".to_string(),
            DoumError::InvalidConfig(msg) => {
                format!("Invalid configuration: {}", msg)
            }
            DoumError::Timeout => "Request timed out. Please try again.".to_string(),
            DoumError::Reqwest(err) => {
                if err.is_timeout() {
                    "Network timeout. Please check your connection and try again.".to_string()
                } else if err.is_connect() {
                    "Failed to connect to server. Please check your network connection.".to_string()
                } else {
                    format!("Network error: {}", err)
                }
            }
            DoumError::Json(err) => {
                format!("JSON error: {}", err)
            }
            DoumError::Toml(err) => {
                format!("TOML error: {}", err)
            }
            DoumError::Dialoguer(msg) => {
                format!("User interaction error: {}", msg)
            }
        }
    }
}

impl From<dialoguer::Error> for DoumError {
    fn from(err: dialoguer::Error) -> Self {
        DoumError::Dialoguer(err.to_string())
    }
}
