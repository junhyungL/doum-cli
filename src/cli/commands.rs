use super::args::ConfigAction;
use crate::cli::config::{get_config, reset_config, set_config, show_config, unset_config};
use crate::cli::secret::{config_provider_secret, select_provider};
use crate::cli::switch::{select_provider_and_model, switch_provider_and_model};
use crate::core::{handle_ask, handle_suggest, select_mode};
use crate::llm::create_client;
use crate::system::error::{DoumError, DoumResult};
use crate::system::{get_system_info, load_config};

pub fn handle_config_command(action: Option<ConfigAction>) -> DoumResult<()> {
    // if no action is provided, default to Show
    let action = action.unwrap_or(ConfigAction::Show);

    match action {
        ConfigAction::Show => show_config(),
        ConfigAction::Reset => reset_config(),
        ConfigAction::Set { key, value } => set_config(&key, &value),
        ConfigAction::Get { key } => get_config(&key),
        ConfigAction::Unset { key } => unset_config(&key),
    }
}

pub fn handle_secret_command(provider: Option<String>) -> DoumResult<()> {
    let provider = match provider {
        Some(p) => p,
        None => select_provider()?,
    };

    config_provider_secret(&provider)
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
        // doum switch error
        _ => {
            return Err(DoumError::Config(
                "Usage: doum switch [provider] [model] or just doum switch".to_string(),
            ));
        }
    }

    Ok(())
}

pub async fn handle_ask_command(question: &str) -> DoumResult<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    handle_ask(question, client.as_ref(), &system_info, &config).await
}

pub async fn handle_suggest_command(request: &str) -> DoumResult<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    handle_suggest(request, client.as_ref(), &system_info, &config).await?;
    Ok(())
}

pub async fn handle_auto_command(input: &str) -> DoumResult<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    select_mode(input, client.as_ref(), &system_info, &config).await
}
