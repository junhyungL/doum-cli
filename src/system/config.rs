use crate::system::error::{DoumError, Result};
use crate::system::paths::get_config_path;
use crate::llm::{OpenAIConfig, AnthropicConfig};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// 정적 파일 임베딩
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

/// 전체 설정 구조
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LLMConfig,
    pub logging: LoggingConfig,
}

/// LLM 관련 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String,
    pub providers: HashMap<String, ProviderConfig>,
    pub context: ContextConfig,
    pub timeout: u64,
    pub max_retries: u32,
    pub use_thinking: bool,
    pub use_web_search: bool,
}

/// 프로바이더별 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
    Openai(OpenAIConfig),
    Anthropic(AnthropicConfig),
}

/// 컨텍스트 수집 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub max_lines: usize,
    pub max_size_kb: usize,
}

impl LLMConfig {
    /// 현재 선택된 프로바이더 설정 가져오기
    pub fn get_current_provider(&self) -> Result<&ProviderConfig> {
        self.providers
            .get(&self.provider)
            .ok_or_else(|| DoumError::Config(
                format!("프로바이더 '{}'를 찾을 수 없습니다", self.provider)
            ))
    }

    /// 특정 프로바이더 설정 가져오기
    pub fn get_provider(&self, name: &str) -> Result<&ProviderConfig> {
        self.providers
            .get(name)
            .ok_or_else(|| DoumError::Config(
                format!("프로바이더 '{}'를 찾을 수 없습니다", name)
            ))
    }
    
    /// 특정 프로바이더 설정 mutable 참조 가져오기
    pub fn get_provider_mut(&mut self, name: &str) -> Result<&mut ProviderConfig> {
        self.providers
            .get_mut(name)
            .ok_or_else(|| DoumError::Config(
                format!("프로바이더 '{}'를 찾을 수 없습니다", name)
            ))
    }
}

/// 로깅 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
}

/// 설정 디렉터리 생성 및 권한 설정
fn ensure_config() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    
    if let Some(parent) = config_path.parent()
        && !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| DoumError::Config(format!("설정 디렉터리 생성 실패: {}", e)))?;
            
            // Unix 시스템에서 디렉터리 권한 설정 (700)
            #[cfg(unix)]
            {
                let metadata = fs::metadata(parent)
                    .map_err(|e| DoumError::Config(format!("디렉터리 메타데이터 읽기 실패: {}", e)))?;
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                fs::set_permissions(parent, permissions)
                    .map_err(|e| DoumError::Config(format!("디렉터리 권한 설정 실패: {}", e)))?;
            }
        }
    
    Ok(config_path)
}

/// 설정 파일 로드 (없으면 기본값으로 생성)
pub fn load_config() -> Result<Config> {
    let config_path = ensure_config()?;
    
    if config_path.exists() {
        // 권한 검증
        validate_config(&config_path)?;
        
        // 설정 파일 읽기
        let content = fs::read_to_string(&config_path)
            .map_err(|e| DoumError::Config(format!("설정 파일 읽기 실패: {}", e)))?;
        
        // TOML 파싱
        let config: Config = toml::from_str(&content)
            .map_err(|e| DoumError::Config(format!("설정 파일 파싱 실패: {}", e)))?;
        
        Ok(config)
    } else {
        // 임베드된 기본 config.toml을 로드하여 저장
        let config = load_default_config()?;
        save_config(&config)?;
        Ok(config)
    }
}

/// 임베드된 기본 config.toml 로드
pub fn load_default_config() -> Result<Config> {
    let config_content = StaticAssets::get("config.toml")
        .ok_or_else(|| DoumError::Config("기본 설정 파일을 찾을 수 없습니다".to_string()))?;
    
    let config_str = std::str::from_utf8(config_content.data.as_ref())
        .map_err(|e| DoumError::Config(format!("기본 설정 파일 인코딩 실패: {}", e)))?;
    
    let config: Config = toml::from_str(config_str)
        .map_err(|e| DoumError::Config(format!("기본 설정 파싱 실패: {}", e)))?;
    
    Ok(config)
}

/// 설정 파일 저장
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = ensure_config()?;
    
    // TOML로 직렬화
    let content = toml::to_string_pretty(config)
        .map_err(|e| DoumError::Config(format!("설정 직렬화 실패: {}", e)))?;
    
    // 파일 쓰기
    fs::write(&config_path, content)
        .map_err(|e| DoumError::Config(format!("설정 파일 쓰기 실패: {}", e)))?;
    
    // Windows에서는 기본 ACL 사용
    #[cfg(windows)]
    {
        // Windows의 경우 기본 ACL이 이미 적절하게 설정되어 있음
        // 추가 보안이 필요한 경우 winapi를 사용하여 ACL 설정 가능
    }

    // Unix에서 파일 권한 설정 (600)
    #[cfg(unix)]
    {
        let metadata = fs::metadata(&config_path)
            .map_err(|e| DoumError::Config(format!("파일 메타데이터 읽기 실패: {}", e)))?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&config_path, permissions)
            .map_err(|e| DoumError::Config(format!("파일 권한 설정 실패: {}", e)))?;
    }
    
    Ok(())
}

/// 설정 파일 권한 검증
fn validate_config(path: &PathBuf) -> Result<()> {
    #[cfg(windows)]
    {
        // Windows에서는 기본적으로 안전하다고 가정
        // 추가 검증이 필요한 경우 구현 가능
        let _ = path; // unused warning 방지
    }

    #[cfg(unix)]
    {
        let metadata = fs::metadata(path)
            .map_err(|e| DoumError::Config(format!("파일 메타데이터 읽기 실패: {}", e)))?;
        let permissions = metadata.permissions();
        let mode = permissions.mode() & 0o777;
        
        // 600 또는 400 권한만 허용
        if mode != 0o600 && mode != 0o400 {
            return Err(DoumError::InvalidConfig(
                format!(
                    "설정 파일 권한이 안전하지 않습니다 (현재: {:o}, 필요: 600 또는 400). \
                    다음 명령으로 수정하세요: chmod 600 {}",
                    mode,
                    path.display()
                )
            ));
        }
    }
    
    Ok(())
}
