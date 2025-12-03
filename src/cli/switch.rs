use crate::core::SwitchService;
use anyhow::{Context, Result};

pub async fn handle_switch_command(provider: Option<String>, model: Option<String>) -> Result<()> {
    use cliclack::{input, select};

    match (provider, model) {
        // Specific provider and model provided
        (Some(prov), Some(mdl)) => {
            SwitchService::switch_to(&prov, &mdl)?;
            cliclack::outro(format!("Switched to {} - {}", prov, mdl))?;
            Ok(())
        }
        // Interactive selection
        (None, None) => {
            cliclack::intro("🔄 Switch Provider & Model")?;

            // Step 1: Select provider
            let providers = SwitchService::list_providers();
            let provider_items: Vec<_> = providers
                .iter()
                .map(|p| (p.as_str(), p.as_str(), ""))
                .collect();

            let selected_provider = select("Select provider")
                .items(&provider_items)
                .interact()
                .context("Provider selection failed")?;

            // Step 2: Select model for the chosen provider
            let models = crate::llm::load_presets(selected_provider);
            let mut model_items: Vec<_> = models
                .iter()
                .map(|m| (m.id.as_str(), m.name.as_str(), m.description.as_str()))
                .collect();

            // Add custom option
            model_items.push(("custom", "Custom", "Enter model name manually"));

            let selected_model_id = select("Select model")
                .items(&model_items)
                .interact()
                .context("Model selection failed")?;

            let model = if selected_model_id == "custom" {
                input("Enter custom model name")
                    .placeholder("e.g., gpt-4-turbo")
                    .interact()
                    .context("Input failed")?
            } else {
                selected_model_id.to_string()
            };

            SwitchService::switch_to(selected_provider, &model)?;
            cliclack::outro(format!("Switched to {} - {}", selected_provider, model))?;
            Ok(())
        }
        _ => {
            anyhow::bail!("Usage: doum switch [provider] [model] or just doum switch")
        }
    }
}
