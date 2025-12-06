use crate::llm::Provider;
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
    pub provider: Provider,
    pub model: String,
    pub timeout: u64,
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
        // Read and parse existing config file
        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
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
            provider: Provider::OpenAI,
            model: "gpt-5".to_string(),
            timeout: 30,
            use_thinking: false,
            use_web_search: true,
        },
        context: ContextConfig {
            max_lines: 100,
            max_size_kb: 50,
        },
        logging: LoggingConfig {
            enabled: true,
            level: "info".to_string(),
        },
    })
}

/// Save configuration to file with secure permissions
pub fn save_config(config: &Config) -> Result<()> {
    // Get config path and write file
    let config_path = ensure_config()?;
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&config_path, content).context("Failed to write config file")?;

    // if Windows, set ACLs for the user only
    #[cfg(windows)]
    {
        // In Windows, basic file permissions are usually sufficient
        // Additional ACL settings can be implemented if needed
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
