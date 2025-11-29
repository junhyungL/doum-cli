use crate::system::paths::get_config_path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Entire application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LLMConfig,
    pub context: ContextConfig,
    pub logging: LoggingConfig,
}

/// Configuration for LLM API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String,
    pub model: String,
    pub timeout: u64,
    pub max_retries: u32,
    pub use_thinking: bool,
    pub use_web_search: bool,
}

/// Context management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub max_lines: usize,
    pub max_size_kb: usize,
}

/// Configuration for logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
}

/// Ensure configuration directory and return config file path
fn ensure_config() -> Result<PathBuf> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).context("Failed to create config directory")?;

        // Set directory permissions to 700 on Unix
        #[cfg(unix)]
        {
            let metadata = fs::metadata(parent).context("Failed to read directory metadata")?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(parent, permissions)
                .context("Failed to set directory permissions")?;
        }
    }

    Ok(config_path)
}

/// load configuration from file or create default
pub fn load_config() -> Result<Config> {
    let config_path = ensure_config()?;

    if config_path.exists() {
        // Validate file permissions
        validate_config(&config_path)?;

        // Read file content
        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        // Parse TOML content
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    } else {
        // If config file doesn't exist, create default
        let config = load_default_config()?;
        save_config(&config)?;
        Ok(config)
    }
}

/// Load default configuration
pub fn load_default_config() -> Result<Config> {
    Ok(Config {
        llm: LLMConfig {
            provider: "openai".to_string(),
            model: "gpt-5".to_string(),
            timeout: 30,
            max_retries: 3,
            use_thinking: false,
            use_web_search: true,
        },
        context: ContextConfig {
            max_lines: 100,
            max_size_kb: 50,
        },
        logging: LoggingConfig {
            enabled: false,
            level: "info".to_string(),
        },
    })
}

/// Save configuration to file with secure permissions
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = ensure_config()?;

    // Serialize configuration to TOML
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;

    // Write to file
    fs::write(&config_path, content).context("Failed to write config file")?;

    // if Windows, set ACLs for the user only
    #[cfg(windows)]
    {
        // Windows의 경우 기본 ACL이 이미 적절하게 설정되어 있음
        // 추가 보안이 필요한 경우 winapi를 사용하여 ACL 설정 가능
    }

    // if Unix, set file permissions to 600
    #[cfg(unix)]
    {
        let metadata = fs::metadata(&config_path).context("File metadata read failed")?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&config_path, permissions).context("Failed to set file permissions")?;
    }

    Ok(())
}

/// Validate configuration file permissions
fn validate_config(path: &PathBuf) -> Result<()> {
    #[cfg(windows)]
    {
        // Windows에서는 기본적으로 안전하다고 가정
        // 추가 검증이 필요한 경우 구현 가능
        let _ = path; // unused warning 방지
    }

    #[cfg(unix)]
    {
        let metadata = fs::metadata(path).context("Failed to read file metadata")?;
        let permissions = metadata.permissions();
        let mode = permissions.mode() & 0o777;

        // Check if permissions are 600 or 400
        if mode != 0o600 && mode != 0o400 {
            anyhow::bail!(
                "Insecure config file permissions: {:o} on {}. Expected 600 or 400.",
                mode,
                path.display()
            );
        }
    }

    Ok(())
}
