use crate::core::SecretService;
use crate::llm::verify_config;
use anyhow::{Context, Result};
use std::collections::HashMap;

pub async fn handle_secret_command(provider: Option<String>) -> Result<()> {
    use cliclack::{input, password, select, spinner};

    // Select provider if not provided
    let provider = match provider {
        Some(p) => p,
        None => {
            let providers = SecretService::list_providers();
            let items: Vec<_> = providers
                .iter()
                .map(|p| (p.as_str(), p.as_str(), ""))
                .collect();

            select("Select provider to configure")
                .items(&items)
                .interact()
                .context("Selection failed")?
                .to_string()
        }
    };

    // Get configuration for this provider
    let config = SecretService::get_provider_config(&provider)?;

    cliclack::intro(format!("🔐 Configure {} Secrets", provider.to_uppercase()))?;

    // Collect values from user
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
        values.insert(field.name.clone(), value);
    }

    // Save secrets
    SecretService::save_secrets(&provider, values)?;

    // Get first model for verification
    let first_model = crate::llm::load_presets(&provider)
        .first()
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "gpt-4".to_string());

    // Verify with spinner
    let sp = spinner();
    sp.start("Verifying API key...");

    match verify_config(&provider, &first_model).await {
        Ok(true) => {
            sp.stop(format!(
                "✅ {} secrets saved and verified successfully",
                provider.to_uppercase()
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
