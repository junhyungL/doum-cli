use thiserror::Error;

/// doum-cli에서 사용하는 결과 타입
pub type Result<T> = std::result::Result<T, DoumError>;

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

    /// TOML 직렬화 에러
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

impl DoumError {
    /// 사용자 친화적인 에러 메시지 반환
    pub fn user_message(&self) -> String {
        match self {
            DoumError::Config(msg) => {
                format!(
                    "⚙️  Configuration Error\n\n\
                     Problem: {}\n\n\
                     💡 Solution:\n\
                     1. Check your config file location:\n\
                        • Windows: %APPDATA%\\doum-cli\\config.toml\n\
                        • macOS: ~/Library/Application Support/doum-cli/config.toml\n\
                        • Linux: ~/.config/doum-cli/config.toml\n\
                     2. Reset to default: doum config unset <key>\n\
                     3. View all settings: doum config show",
                    msg
                )
            }
            DoumError::LLM(msg) => {
                if msg.contains("401") || msg.contains("unauthorized") {
                    "🔑 API Key Error\n\n\
                     Problem: Invalid or missing API key\n\n\
                     💡 Solution:\n\
                     1. Set your API key: doum config set llm.api_key sk-...\n\
                     2. Get a key from: https://platform.openai.com/api-keys\n\
                     3. Verify key format (starts with 'sk-')".to_string()
                } else if msg.contains("timeout") || msg.contains("timed out") {
                    "⏱️  Request Timeout\n\n\
                     Problem: LLM request took too long\n\n\
                     💡 Solution:\n\
                     1. Increase timeout: doum config set llm.timeout 60\n\
                     2. Check your internet connection\n\
                     3. Try again in a few moments".to_string()
                } else if msg.contains("rate limit") || msg.contains("429") {
                    "🚦 Rate Limit Exceeded\n\n\
                     Problem: Too many requests to the API\n\n\
                     💡 Solution:\n\
                     1. Wait a moment and try again\n\
                     2. Check your API quota at: https://platform.openai.com/usage\n\
                     3. Consider upgrading your plan".to_string()
                } else {
                    format!(
                        "🤖 LLM API Error\n\n\
                         Problem: {}\n\n\
                         💡 Solution:\n\
                         1. Check your internet connection\n\
                         2. Verify API key: doum config get llm.api_key\n\
                         3. Check OpenAI status: https://status.openai.com",
                        msg
                    )
                }
            }
            DoumError::Parse(msg) => {
                format!(
                    "📝 Parse Error\n\n\
                     Problem: Failed to parse LLM response\n\
                     Details: {}\n\n\
                     💡 Solution:\n\
                     1. This usually resolves automatically (retry logic active)\n\
                     2. If it persists, try a different model: doum config set llm.model gpt-4\n\
                     3. Increase retry limit: doum config set llm.max_retries 5",
                    msg
                )
            }
            DoumError::CommandExecution(msg) => {
                format!(
                    "⚠️  Command Execution Failed\n\n\
                     Problem: {}\n\n\
                     💡 Solution:\n\
                     1. Check if you have necessary permissions\n\
                     2. Verify the command is valid for your OS/shell\n\
                     3. Try running the command manually first\n\
                     4. Use suggest mode to explore alternatives: doum suggest \"<request>\"",
                    msg
                )
            }
            DoumError::UserCancelled => {
                "❌ Operation Cancelled\n\n\
                 You cancelled the operation. No changes were made.".to_string()
            }
            DoumError::InvalidConfig(msg) => {
                format!(
                    "🔧 Invalid Configuration\n\n\
                     Problem: {}\n\n\
                     💡 Solution:\n\
                     1. View current config: doum config show\n\
                     2. Reset to default: doum config unset <key>\n\
                     3. Check valid values in documentation",
                    msg
                )
            }
            DoumError::Timeout => {
                "⏱️  Request Timeout\n\n\
                 Problem: The request took too long\n\n\
                 💡 Solution:\n\
                 1. Increase timeout: doum config set llm.timeout 60\n\
                 2. Check your internet connection\n\
                 3. Try with a simpler request".to_string()
            }
            DoumError::Io(err) => {
                format!(
                    "💾 File System Error\n\n\
                     Problem: {}\n\n\
                     💡 Solution:\n\
                     1. Check file permissions\n\
                     2. Verify the path exists\n\
                     3. Make sure you have sufficient disk space",
                    err
                )
            }
            DoumError::Reqwest(err) => {
                if err.is_timeout() {
                    "⏱️  Network Timeout\n\n\
                     Problem: Network request timed out\n\n\
                     💡 Solution:\n\
                     1. Check your internet connection\n\
                     2. Increase timeout: doum config set llm.timeout 60\n\
                     3. Try again in a few moments".to_string()
                } else if err.is_connect() {
                    "🌐 Connection Error\n\n\
                     Problem: Failed to connect to the API\n\n\
                     💡 Solution:\n\
                     1. Check your internet connection\n\
                     2. Verify firewall settings\n\
                     3. Check if you need a proxy".to_string()
                } else {
                    format!(
                        "🌐 Network Error\n\n\
                         Problem: {}\n\n\
                         💡 Solution:\n\
                         1. Check your internet connection\n\
                         2. Try again in a few moments",
                        err
                    )
                }
            }
            DoumError::Json(err) => {
                format!(
                    "📄 JSON Error\n\n\
                     Problem: Failed to parse JSON data\n\
                     Details: {}\n\n\
                     💡 This is likely a temporary issue. Please try again.",
                    err
                )
            }
            DoumError::Toml(err) => {
                format!(
                    "📝 Configuration File Error\n\n\
                     Problem: Failed to read config file\n\
                     Details: {}\n\n\
                     💡 Solution:\n\
                     1. Check if config file is corrupted\n\
                     2. Backup and delete config file to reset\n\
                     3. Config will be recreated with defaults",
                    err
                )
            }
            DoumError::TomlSer(err) => {
                format!(
                    "📝 Configuration Save Error\n\n\
                     Problem: Failed to save config file\n\
                     Details: {}\n\n\
                     💡 Solution:\n\
                     1. Check file permissions\n\
                     2. Verify disk space is available",
                    err
                )
            }
        }
    }
}
