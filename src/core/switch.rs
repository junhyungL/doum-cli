use crate::llm::{Provider, load_presets};
use crate::system::{load_config, save_config};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderModelOption {
    pub provider: String,
    pub model_id: String,
    pub display_name: String,
    pub description: String,
}

/// Get all available provider/model combinations
pub fn get_all_options() -> Vec<ProviderModelOption> {
    let providers = Provider::all();
    let mut options = Vec::new();

    for provider in providers {
        let models = load_presets(&provider);
        for model in models {
            options.push(ProviderModelOption {
                provider: provider.as_str().to_string(),
                model_id: model.id.clone(),
                display_name: format!("{} - {}", provider.as_str().to_uppercase(), model.name),
                description: model.description.clone(),
            });
        }

        // Add custom option
        options.push(ProviderModelOption {
            provider: provider.as_str().to_string(),
            model_id: "custom".to_string(),
            display_name: format!("{} - Custom", provider.as_str().to_uppercase()),
            description: "Enter model name manually".to_string(),
        });
    }

    options
}

/// Switch to a specific provider and model
pub fn switch_to(provider: &Provider, model: &str) -> Result<()> {
    let mut config = load_config()?;
    config.llm.provider = provider.as_str().to_string();
    config.llm.model = model.to_string();
    save_config(&config)?;

    Ok(())
}
