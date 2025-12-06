use crate::llm::{AnthropicSecret, OpenAISecret, Provider, load_presets, verify_client};
use crate::system::{LLMConfig, SecretManager};
use anyhow::{Context, Result};
use cliclack::{input, password, select, spinner};

pub async fn handle_secret_command() -> Result<()> {
    cliclack::intro("🔐 Configure LLM Provider Secret")?;

    // Step 1: Select provider
    let providers = Provider::all();
    let provider_items: Vec<_> = providers
        .iter()
        .map(|p| (p.as_str(), p.as_display(), ""))
        .collect();

    let provider_str = select("Select provider to configure")
        .items(&provider_items)
        .interact()
        .context("Selection failed")?;

    let provider: Provider = provider_str.parse()?;

    // Step 2: Input secrets based on provider
    match provider {
        Provider::OpenAI => {
            let api_key = password("OpenAI API Key (required)")
                .interact()
                .context("Password input failed")?;

            let organization: String = input("Organization ID (optional, press Enter to skip)")
                .placeholder("Press Enter to skip")
                .required(false)
                .interact()
                .context("Input failed")?;

            let project: String = input("Project ID (optional, press Enter to skip)")
                .placeholder("Press Enter to skip")
                .required(false)
                .interact()
                .context("Input failed")?;

            let secret = OpenAISecret {
                api_key: api_key.trim().to_string(),
                organization: if organization.trim().is_empty() {
                    None
                } else {
                    Some(organization.trim().to_string())
                },
                project: if project.trim().is_empty() {
                    None
                } else {
                    Some(project.trim().to_string())
                },
            };
            SecretManager::save(&provider, &secret)?;
        }
        Provider::Anthropic => {
            let api_key = password("Anthropic API Key (required)")
                .interact()
                .context("Password input failed")?;

            let secret = AnthropicSecret {
                api_key: api_key.trim().to_string(),
            };
            SecretManager::save(&provider, &secret)?;
        }
    }

    // Step 3: Get first model and verification
    let first_model = load_presets(&provider)
        .first()
        .map(|m| m.id.clone())
        .context("No preset models available for this provider")?;

    let llm_config = LLMConfig {
        provider,
        model: first_model.clone(),
        timeout: 30,
        use_thinking: false,
        use_web_search: false,
    };

    let sp = spinner();
    sp.start("Verifying API key...");

    match verify_client(&llm_config).await {
        Ok(true) => {
            sp.stop(format!(
                "{} secrets saved and verified successfully",
                provider_str.to_uppercase()
            ));
            cliclack::outro("✅ Configuration complete!")?;
            Ok(())
        }
        Ok(false) => {
            sp.stop("API key verification failed");
            cliclack::outro(
                "⚠️ Secrets saved but verification failed. Please check your API key.",
            )?;
            Ok(())
        }
        Err(e) => {
            sp.stop("Verification error");
            cliclack::outro(format!("❌ Error: {}", e))?;
            Err(e)
        }
    }
}