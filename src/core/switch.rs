use crate::llm::Provider;
use crate::system::{load_config, save_config};
use anyhow::Result;

/// Switch to a specific provider and model
pub fn switch_to(provider: &Provider, model: &str) -> Result<()> {
    let mut config = load_config()?;
    config.llm.provider = *provider;
    config.llm.model = model.to_string();
    save_config(&config)?;

    Ok(())
}
