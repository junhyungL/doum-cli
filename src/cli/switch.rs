use crate::cli::ui::prompt_text_input;
use crate::system::DoumError::Config;
use crate::system::error::DoumResult;
use crate::system::{load_config, save_config};
use dialoguer::Select;
use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "static/presets/"]
struct ModelPresets;

#[derive(Debug, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct ModelList {
    models: Vec<ModelInfo>,
}

/// Load model presets for a provider
pub fn load_presets(provider: &str) -> Vec<ModelInfo> {
    let filename = format!("{}.toml", provider);

    if let Some(content) = ModelPresets::get(&filename)
        && let Ok(data) = std::str::from_utf8(content.data.as_ref())
        && let Ok(list) = toml::from_str::<ModelList>(data)
    {
        return list.models;
    }

    vec![]
}

pub fn handle_switch_command(provider: Option<String>, model: Option<String>) -> DoumResult<()> {
    match (provider, model) {
        // doum switch openai gpt-5
        (Some(prov), Some(mdl)) => {
            switch_provider_and_model(&prov, &mdl)?;
        }
        // doum interactive selection
        (None, None) => {
            select_provider_and_model()?;
        }
        // doum switch openai (invalid)
        _ => {
            return Err(Config(
                "Usage: doum switch [provider] [model] or just doum switch".to_string(),
            ));
        }
    }

    Ok(())
}

/// Interactive provider/model selection
fn select_provider_and_model() -> DoumResult<()> {
    let providers = vec!["openai", "anthropic"];

    // Build menu items
    let mut menu_items = Vec::new();
    let mut mapping = Vec::new(); // (provider, model_id)

    for provider in &providers {
        let models = load_presets(provider);
        for model in models {
            menu_items.push(format!(
                "{} - {} | {}",
                provider.to_uppercase(),
                model.name,
                model.description
            ));
            mapping.push((provider.to_string(), model.id.clone()));
        }

        // Add custom option
        menu_items.push(format!(
            "{} - Custom (enter manually)",
            provider.to_uppercase()
        ));
        mapping.push((provider.to_string(), "custom".to_string()));
    }

    let selection = Select::new()
        .with_prompt("Select provider and model")
        .items(&menu_items)
        .default(0)
        .interact()?;

    let (provider, model_id) = &mapping[selection];

    let model = if model_id == "custom" {
        prompt_text_input("Enter custom model name", None)?
    } else {
        model_id.clone()
    };

    switch_provider_and_model(provider, &model)?;

    Ok(())
}

/// Switch provider and model
fn switch_provider_and_model(provider: &str, model: &str) -> DoumResult<()> {
    let mut config = load_config()?;

    // Validate provider
    if provider != "openai" && provider != "anthropic" {
        return Err(Config(format!(
            "Unknown provider: {}. Available: openai, anthropic",
            provider
        )));
    }

    config.llm.provider = provider.to_string();
    config.llm.model = model.to_string();

    save_config(&config)?;

    println!("✅ Switched to {} - {}", provider, model);

    Ok(())
}
