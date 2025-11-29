use crate::system::{get_config_path, load_config, load_default_config, save_config};
use anyhow::Result;

pub struct ConfigManager;

impl ConfigManager {
    /// Get config value by key
    pub fn get_value(key: &str) -> Result<String> {
        let config = load_config()?;

        let value = match key {
            "llm.provider" => config.llm.provider,
            "llm.model" => config.llm.model,
            "llm.timeout" => config.llm.timeout.to_string(),
            "llm.max_retries" => config.llm.max_retries.to_string(),
            "llm.use_thinking" => config.llm.use_thinking.to_string(),
            "llm.use_web_search" => config.llm.use_web_search.to_string(),
            "context.max_lines" => config.context.max_lines.to_string(),
            "context.max_size_kb" => config.context.max_size_kb.to_string(),
            "logging.enabled" => config.logging.enabled.to_string(),
            "logging.level" => config.logging.level,
            _ => {
                anyhow::bail!("Unknown config key: {}", key);
            }
        };

        Ok(value)
    }

    /// Set config value by key
    pub fn set_value(key: &str, value: &str) -> Result<()> {
        let mut config = load_config()?;

        match key {
            "llm.provider" => {
                Self::validate_provider(value)?;
                config.llm.provider = value.to_string();
            }
            "llm.model" => {
                config.llm.model = value.to_string();
            }
            "llm.timeout" => {
                config.llm.timeout = Self::parse_value(value, "timeout")?;
            }
            "llm.max_retries" => {
                config.llm.max_retries = Self::parse_value(value, "max_retries")?;
            }
            "llm.use_thinking" => {
                config.llm.use_thinking = Self::parse_value(value, "use_thinking")?;
            }
            "llm.use_web_search" => {
                config.llm.use_web_search = Self::parse_value(value, "use_web_search")?;
            }
            "context.max_lines" => {
                config.context.max_lines = Self::parse_value(value, "max_lines")?;
            }
            "context.max_size_kb" => {
                config.context.max_size_kb = Self::parse_value(value, "max_size_kb")?;
            }
            "logging.enabled" => {
                config.logging.enabled = Self::parse_value(value, "logging.enabled")?;
            }
            "logging.level" => {
                config.logging.level = value.to_string();
            }
            _ => {
                anyhow::bail!("Unknown config key: {}", key);
            }
        }

        save_config(&config)?;
        Ok(())
    }

    /// Reset config to defaults
    pub fn reset() -> Result<()> {
        let default_config = load_default_config()?;
        save_config(&default_config)
    }

    /// Unset a key (restore to default)
    pub fn unset_value(key: &str) -> Result<()> {
        let default_config = load_default_config()?;
        let mut config = load_config()?;

        match key {
            "llm.provider" => config.llm.provider = default_config.llm.provider,
            "llm.model" => config.llm.model = default_config.llm.model,
            "llm.timeout" => config.llm.timeout = default_config.llm.timeout,
            "llm.max_retries" => config.llm.max_retries = default_config.llm.max_retries,
            "llm.use_thinking" => config.llm.use_thinking = default_config.llm.use_thinking,
            "llm.use_web_search" => config.llm.use_web_search = default_config.llm.use_web_search,
            "context.max_lines" => config.context.max_lines = default_config.context.max_lines,
            "context.max_size_kb" => {
                config.context.max_size_kb = default_config.context.max_size_kb
            }
            "logging.enabled" => config.logging.enabled = default_config.logging.enabled,
            "logging.level" => config.logging.level = default_config.logging.level,
            _ => {
                anyhow::bail!("Unknown config key: {}", key);
            }
        }

        save_config(&config)?;
        Ok(())
    }

    /// Get all config as TOML string
    pub fn get_all_as_toml() -> Result<String> {
        let config = load_config()?;
        toml::to_string_pretty(&config)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config to TOML: {}", e))
    }

    /// Get config file path
    pub fn get_config_path() -> Result<std::path::PathBuf> {
        get_config_path()
    }

    // Private helpers
    fn validate_provider(provider: &str) -> Result<()> {
        if provider != "openai" && provider != "anthropic" {
            anyhow::bail!(
                "Invalid provider: {}. Available: openai, anthropic",
                provider
            );
        }
        Ok(())
    }

    fn parse_value<T: std::str::FromStr>(value: &str, field_name: &str) -> Result<T>
    where
        T::Err: std::fmt::Display,
    {
        value
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid {} value: {} - {}", field_name, value, e))
    }
}
