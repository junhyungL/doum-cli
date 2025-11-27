use crate::cli::ui::prompt_text_input;
use crate::llm::load_presets;
use crate::system::DoumError::Config;
use crate::system::error::DoumResult;
use crate::system::{load_config, save_config};
use dialoguer::Select;

/// Interactive provider/model selection
pub fn select_provider_and_model() -> DoumResult<()> {
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
pub fn switch_provider_and_model(provider: &str, model: &str) -> DoumResult<()> {
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
