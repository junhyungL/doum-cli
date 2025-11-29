use super::args::ConfigAction;
use crate::cli::ui::{prompt_password_input, prompt_text_input};
use crate::core::{ConfigManager, SecretService, SwitchService};
use crate::core::{handle_ask, handle_suggest, select_mode};
use crate::llm::create_client;
use crate::system::{get_system_info, load_config};
use anyhow::{Context, Result};
use dialoguer::Select;
use std::collections::HashMap;

pub fn handle_config_command(action: Option<ConfigAction>) -> Result<()> {
    // if no action is provided, default to Show
    let action = action.unwrap_or(ConfigAction::Show);

    match action {
        ConfigAction::Show => {
            let config_path = ConfigManager::get_config_path()?;
            let toml_str = ConfigManager::get_all_as_toml()?;
            println!("Config file location: {}\n", config_path.display());
            println!("{}", toml_str);
            Ok(())
        }
        ConfigAction::Reset => {
            ConfigManager::reset()?;
            println!("✅ Configuration reset to default");
            Ok(())
        }
        ConfigAction::Set { key, value } => {
            ConfigManager::set_value(&key, &value)?;
            println!("✅ Config {} = {}", key, value);
            Ok(())
        }
        ConfigAction::Get { key } => {
            let value = ConfigManager::get_value(&key)?;
            println!("{}", value);
            Ok(())
        }
        ConfigAction::Unset { key } => {
            ConfigManager::unset_value(&key)?;
            println!("✅ Config {} reset to default", key);
            Ok(())
        }
    }
}

pub fn handle_secret_command(provider: Option<String>) -> Result<()> {
    // Select provider if not provided
    let provider = match provider {
        Some(p) => p,
        None => {
            let providers = SecretService::list_providers();
            let selection = Select::new()
                .with_prompt("Select provider to configure")
                .items(&providers)
                .default(0)
                .interact()
                .context("Selection failed")?;
            providers[selection].clone()
        }
    };

    // Get configuration for this provider
    let config = SecretService::get_provider_config(&provider)?;

    println!("\n🔐 Configure {} Secrets\n", provider.to_uppercase());

    // Collect values from user
    let mut values = HashMap::new();
    for field in &config.fields {
        let value = if field.is_password {
            prompt_password_input(&field.label)?
        } else {
            prompt_text_input(&field.label, None)?
        };
        values.insert(field.name.clone(), value);
    }

    // Save secrets
    SecretService::save_secrets(&provider, values)?;

    println!(
        "\n✅ {} secrets saved successfully",
        provider.to_uppercase()
    );
    Ok(())
}

pub fn handle_switch_command(provider: Option<String>, model: Option<String>) -> Result<()> {
    match (provider, model) {
        // Specific provider and model
        (Some(prov), Some(mdl)) => {
            SwitchService::switch_to(&prov, &mdl)?;
            println!("✅ Switched to {} - {}", prov, mdl);
            Ok(())
        }
        // Interactive selection
        (None, None) => {
            let options = SwitchService::get_all_options();

            let items: Vec<String> = options
                .iter()
                .map(|opt| format!("{} | {}", opt.display_name, opt.description))
                .collect();

            let selection = Select::new()
                .with_prompt("Select provider and model")
                .items(&items)
                .default(0)
                .interact()
                .context("Selection failed")?;

            let selected = &options[selection];
            let model = if selected.model_id == "custom" {
                prompt_text_input("Enter custom model name", None)?
            } else {
                selected.model_id.clone()
            };

            SwitchService::switch_to(&selected.provider, &model)?;
            println!("✅ Switched to {} - {}", selected.provider, model);
            Ok(())
        }
        _ => anyhow::bail!("Usage: doum switch [provider] [model] or just doum switch"),
    }
}

pub async fn handle_ask_command(question: &str) -> Result<()> {
    use crate::cli::ui::{display_ask_response, with_spinner};

    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    // Execute with spinner
    let response = with_spinner(
        "AI is generating an answer...",
        handle_ask(question, client.as_ref(), &system_info, &config),
    )
    .await?;

    // Display response
    display_ask_response(&response);

    Ok(())
}

pub async fn handle_suggest_command(request: &str) -> Result<()> {
    use crate::cli::ui::{handle_suggest_response, with_spinner};

    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    // Execute with spinner
    let response = with_spinner(
        "AI is generating commands...",
        handle_suggest(request, client.as_ref(), &system_info, &config),
    )
    .await?;

    // Handle suggestion response (selection, copy, execution)
    handle_suggest_response(&response.suggestions, &system_info)?;

    Ok(())
}

pub async fn handle_auto_command(input: &str) -> Result<()> {
    use crate::cli::ui::{display_selected_mode, with_spinner};

    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    // Select mode with spinner
    let mode_response = with_spinner(
        "Analyzing input...",
        select_mode(input, client.as_ref(), &system_info, &config),
    )
    .await?;

    // Display selected mode
    display_selected_mode(&mode_response.mode);

    // Execute based on selected mode
    match mode_response.mode.as_str() {
        "ask" => handle_ask_command(input).await,
        "suggest" | "execute" => handle_suggest_command(input).await,
        unknown => {
            println!("⚠️  Unknown mode: {}", unknown);
            println!("💡 Falling back to Ask mode.\n");
            handle_ask_command(input).await
        }
    }
}
