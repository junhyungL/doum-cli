use crate::core::switch_to;
use crate::llm::{Provider, load_presets};
use anyhow::{Context, Result};
use cliclack::{input, select};

pub async fn handle_switch_command() -> Result<()> {
    cliclack::intro("🔄 Switch Provider & Model")?;

    // Step 1: Select provider
    let providers = Provider::all();
    let provider_items: Vec<_> = providers
        .iter()
        .map(|p| (p.as_str(), p.as_str(), ""))
        .collect();

    let provider_str = select("Select provider")
        .items(&provider_items)
        .interact()
        .context("Provider selection failed")?;

    let provider: Provider = provider_str.parse()?;

    // Step 2: Select model for the chosen provider
    let models = load_presets(&provider);
    let mut model_items: Vec<_> = models
        .iter()
        .map(|m| (m.id.as_str(), m.name.as_str(), m.description.as_str()))
        .collect();

    // Add custom option
    model_items.push(("custom", "Custom", "Enter model name manually"));

    let model_id = select("Select model")
        .items(&model_items)
        .interact()
        .context("Model selection failed")?;

    let model = if model_id == "custom" {
        input("Enter custom model name")
            .placeholder("e.g., gpt-4-turbo")
            .interact()
            .context("Input failed")?
    } else {
        model_id.to_string()
    };

    switch_to(&provider, &model)?;
    cliclack::outro(format!("Switched to {} - {}", provider_str, model))?;
    Ok(())
}
