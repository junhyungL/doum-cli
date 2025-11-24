use doum_cli::system::{Config, ProviderConfig, save_config, load_config, load_default_config};

#[test]
fn test_config_default() {
    let config = load_default_config().unwrap();
    assert_eq!(config.llm.provider, "openai");
    assert!(config.llm.providers.contains_key("openai"));
    assert!(config.llm.providers.contains_key("anthropic"));
}

#[test]
fn test_config_serialization() {
    let config = load_default_config().unwrap();
    let toml_str = toml::to_string_pretty(&config).unwrap();
    
    assert!(toml_str.contains("provider"));
    assert!(toml_str.contains("openai"));
    assert!(toml_str.contains("[llm.providers.openai]"));
    
    let deserialized: Config = toml::from_str(&toml_str).unwrap();
    assert_eq!(deserialized.llm.provider, config.llm.provider);
}

#[test]
fn test_config_save_and_load() {
    let mut config = load_default_config().unwrap();
    if let Some(ProviderConfig::Openai(openai)) = config.llm.providers.get_mut("openai") {
        openai.model = "test-model".to_string();
    }
    config.llm.timeout = 45;
    
    save_config(&config).unwrap();
    let loaded = load_config().unwrap();
    
    if let Some(ProviderConfig::Openai(openai)) = loaded.llm.providers.get("openai") {
        assert_eq!(openai.model, "test-model");
    } else {
        panic!("OpenAI provider not found");
    }
    assert_eq!(loaded.llm.timeout, 45);
}

#[test]
fn test_get_current_provider() {
    let config = load_default_config().unwrap();
    let current = config.llm.get_current_provider().unwrap();
    
    match current {
        ProviderConfig::Openai(cfg) => assert_eq!(cfg.model, "gpt-4-turbo"),
        _ => panic!("Expected OpenAI provider"),
    }
}

#[test]
fn test_get_provider() {
    let config = load_default_config().unwrap();
    
    let openai = config.llm.get_provider("openai").unwrap();
    match openai {
        ProviderConfig::Openai(cfg) => assert_eq!(cfg.model, "gpt-4-turbo"),
        _ => panic!("Expected OpenAI provider"),
    }
    
    let anthropic = config.llm.get_provider("anthropic").unwrap();
    match anthropic {
        ProviderConfig::Anthropic(cfg) => assert_eq!(cfg.model, "claude-3-5-sonnet-20241022"),
        _ => panic!("Expected Anthropic provider"),
    }
}
