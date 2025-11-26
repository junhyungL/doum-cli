use crate::system::error::{DoumError, DoumResult};
use crate::system::{get_config_path, load_config, load_default_config, save_config};

pub fn show_config() -> DoumResult<()> {
    let config = load_config()?;
    let config_path = get_config_path()?;
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| DoumError::Config(format!("Failed to serialize config to TOML: {}", e)))?;

    println!("Config file location: {}\n", config_path.display());
    println!("{}", toml_str);

    Ok(())
}

pub fn reset_config() -> DoumResult<()> {
    let default_config = load_default_config()?;
    save_config(&default_config)?;

    println!("✅ Configuration reset to default");

    Ok(())
}

pub fn set_config(key: &str, value: &str) -> DoumResult<()> {
    let mut config = load_config()?;

    match key {
        "llm.provider" => {
            if value != "openai" && value != "anthropic" {
                return Err(DoumError::Config(format!(
                    "Invalid provider: {}. Available: openai, anthropic",
                    value
                )));
            }
            config.llm.provider = value.to_string();
        }
        "llm.model" => {
            config.llm.model = value.to_string();
        }
        "llm.timeout" => {
            config.llm.timeout = value
                .parse()
                .map_err(|_| DoumError::Config(format!("Invalid timeout value: {}", value)))?;
        }
        "llm.max_retries" => {
            config.llm.max_retries = value
                .parse()
                .map_err(|_| DoumError::Config(format!("Invalid max_retries value: {}", value)))?;
        }
        "llm.use_thinking" => {
            config.llm.use_thinking = value
                .parse()
                .map_err(|_| DoumError::Config(format!("Invalid use_thinking value: {}", value)))?;
        }
        "llm.use_web_search" => {
            config.llm.use_web_search = value.parse().map_err(|_| {
                DoumError::Config(format!("Invalid use_web_search value: {}", value))
            })?;
        }
        "context.max_lines" => {
            config.context.max_lines = value
                .parse()
                .map_err(|_| DoumError::Config(format!("Invalid max_lines value: {}", value)))?;
        }
        "context.max_size_kb" => {
            config.context.max_size_kb = value
                .parse()
                .map_err(|_| DoumError::Config(format!("Invalid max_size_kb value: {}", value)))?;
        }
        "logging.enabled" => {
            config.logging.enabled = value.parse().map_err(|_| {
                DoumError::Config(format!("Invalid logging.enabled value: {}", value))
            })?;
        }
        "logging.level" => {
            config.logging.level = value.to_string();
        }
        _ => {
            return Err(DoumError::Config(format!("Unknown config key: {}", key)));
        }
    }

    save_config(&config)?;
    println!("✅ Config {} = {}", key, value);

    Ok(())
}

pub fn get_config(key: &str) -> DoumResult<()> {
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
            return Err(DoumError::Config(format!("Unknown config key: {}", key)));
        }
    };

    println!("{}", value);

    Ok(())
}

pub fn unset_config(key: &str) -> DoumResult<()> {
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
        "context.max_size_kb" => config.context.max_size_kb = default_config.context.max_size_kb,
        "logging.enabled" => config.logging.enabled = default_config.logging.enabled,
        "logging.level" => config.logging.level = default_config.logging.level,
        _ => {
            return Err(DoumError::Config(format!("Unknown config key: {}", key)));
        }
    }

    save_config(&config)?;
    println!("✅ Config {} reset to default", key);

    Ok(())
}
