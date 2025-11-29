use crate::system::ProviderSecret;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Anthropic Secret information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnthropicSecret {
    pub api_key: String,
}

impl ProviderSecret for AnthropicSecret {
    fn validate(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("API key cannot be empty");
        }
        Ok(())
    }

    fn masked(&self) -> String {
        if self.api_key.len() > 14 {
            format!(
                "{}...{}",
                &self.api_key[..10],
                &self.api_key[self.api_key.len() - 4..]
            )
        } else {
            "***".to_string()
        }
    }
}
