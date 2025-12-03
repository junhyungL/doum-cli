use crate::llm::load_presets;
use crate::system::{load_config, save_config};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderModelOption {
    pub provider: String,
    pub model_id: String,
    pub display_name: String,
    pub description: String,
}

pub struct SwitchService;

impl SwitchService {
    /// Get all available provider/model combinations
    pub fn get_all_options() -> Vec<ProviderModelOption> {
        let providers = vec!["openai", "anthropic"];
        let mut options = Vec::new();

        for provider in providers {
            let models = load_presets(provider);
            for model in models {
                options.push(ProviderModelOption {
                    provider: provider.to_string(),
                    model_id: model.id.clone(),
                    display_name: format!("{} - {}", provider.to_uppercase(), model.name),
                    description: model.description.clone(),
                });
            }

            // Add custom option
            options.push(ProviderModelOption {
                provider: provider.to_string(),
                model_id: "custom".to_string(),
                display_name: format!("{} - Custom", provider.to_uppercase()),
                description: "Enter model name manually".to_string(),
            });
        }

        options
    }

    /// Switch to a specific provider and model
    pub fn switch_to(provider: &str, model: &str) -> Result<()> {
        // Validate provider
        if provider != "openai" && provider != "anthropic" {
            anyhow::bail!(
                "Unknown provider: {}. Available: openai, anthropic",
                provider
            );
        }

        let mut config = load_config()?;
        config.llm.provider = provider.to_string();
        config.llm.model = model.to_string();
        save_config(&config)?;

        Ok(())
    }

    /// List all available providers
    pub fn list_providers() -> Vec<String> {
        vec!["openai".to_string(), "anthropic".to_string()]
    }
}
