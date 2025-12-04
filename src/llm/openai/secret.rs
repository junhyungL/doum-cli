use crate::system::ProviderSecret;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// OpenAI Secret information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAISecret {
    pub api_key: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ProviderSecret for OpenAISecret {
    fn validate(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("API key cannot be empty");
        }
        Ok(())
    }

    fn masked(&self) -> String {
        if self.api_key.len() > 10 {
            format!(
                "{}...{}",
                &self.api_key[..7],
                &self.api_key[self.api_key.len() - 4..]
            )
        } else {
            "***".to_string()
        }
    }
}
