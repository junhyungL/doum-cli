use crate::cli::ui::{prompt_password_input, prompt_text_input};
use crate::llm::AnthropicSecret;
use crate::llm::OpenAISecret;
use crate::system::DoumError::Config;
use crate::system::error::DoumResult;
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

pub fn handle_secret_command(provider: Option<String>) -> DoumResult<()> {
    let provider = match provider {
        Some(p) => p,
        None => select_provider()?,
    };

    configure_provider_secret(&provider)
}

fn select_provider() -> DoumResult<String> {
    let providers = vec!["openai", "anthropic"];
    let selection = Select::new()
        .with_prompt("Select provider to configure")
        .items(&providers)
        .default(0)
        .interact()?;

    Ok(providers[selection].to_string())
}

fn configure_provider_secret(provider: &str) -> DoumResult<()> {
    match provider {
        "openai" => configure_openai_secret(),
        "anthropic" => configure_anthropic_secret(),
        _ => Err(Config(format!("Unknown provider: {}", provider))),
    }
}

fn configure_openai_secret() -> DoumResult<()> {
    println!("\n🔐 Configure OpenAI Secrets\n");

    let api_key = prompt_password_input("OpenAI API Key (required)")?;

    let organization = prompt_text_input("Organization ID (optional, press Enter to skip)", None)?;

    let project = prompt_text_input("Project ID (optional, press Enter to skip)", None)?;

    let secret = OpenAISecret {
        api_key,
        organization: if organization.is_empty() {
            None
        } else {
            Some(organization)
        },
        project: if project.is_empty() {
            None
        } else {
            Some(project)
        },
    };

    secret.save()?;
    println!("\n✅ OpenAI secrets saved successfully");
    Ok(())
}

fn configure_anthropic_secret() -> DoumResult<()> {
    println!("\n🔐 Configure Anthropic Secrets\n");

    let api_key = prompt_password_input("Anthropic API Key (required)")?;

    let secret = AnthropicSecret { api_key };

    secret.save()?;
    println!("\n✅ Anthropic secrets saved successfully");
    Ok(())
}
