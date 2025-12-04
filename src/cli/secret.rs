use crate::core::{get_provider_config, save_secrets};
use crate::llm::{Provider, load_presets, verify_config};
use anyhow::{Context, Result};
use cliclack::{input, password, select, spinner};
use std::collections::HashMap;

pub async fn handle_secret_command() -> Result<()> {
    cliclack::intro("🔐 Configure LLM Provider Secret")?;

    // Step 1: Select provider
    let providers = Provider::all();
    let provider_items: Vec<_> = providers
        .iter()
        .map(|p| (p.as_str(), p.as_str(), ""))
        .collect();

    let provider_str = select("Select provider to configure")
        .items(&provider_items)
        .interact()
        .context("Selection failed")?;

    let provider: Provider = provider_str.parse()?;

    // Step 2: Prompt for secret fields
    let config = get_provider_config(&provider)?;
    let mut values = HashMap::new();
    for field in &config.fields {
        let value = if field.is_password {
            password(&field.label)
                .interact()
                .context("Password input failed")?
        } else {
            // Optional field: allow empty input
            let input_value: String = input(&field.label)
                .placeholder("Press Enter to skip")
                .required(false)
                .interact()
                .context("Input failed")?;
            input_value
        };
        values.insert(field.name.clone(), value.trim().to_string());
    }

    // Save secrets
    save_secrets(&provider, values)?;

    // Get first model for verification
    let first_model = load_presets(&provider)
        .first()
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "gpt-4".to_string());

    // Verify with spinner
    let sp = spinner();
    sp.start("Verifying API key...");

    match verify_config(provider.as_str(), &first_model).await {
        Ok(true) => {
            sp.stop(format!(
                "✅ {} secrets saved and verified successfully",
                provider_str.to_uppercase()
            ));
            cliclack::outro("Configuration complete!")?;
            Ok(())
        }
        Ok(false) => {
            sp.stop("⚠️  API key verification failed");
            cliclack::outro("Secrets saved but verification failed. Please check your API key.")?;
            Ok(())
        }
        Err(e) => {
            sp.stop("❌ Verification error");
            cliclack::outro(format!("Error: {}", e))?;
            Err(e)
        }
    }
}
