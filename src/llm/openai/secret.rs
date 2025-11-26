use crate::system::{DoumError, DoumResult};
use keyring::Entry;
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

impl OpenAISecret {
    /// Parse Secret from JSON string
    pub fn from_string(s: &str) -> DoumResult<Self> {
        serde_json::from_str(s)
            .map_err(|e| DoumError::Parse(format!("Failed to parse OpenAI secret: {}", e)))
    }

    /// Save a secret value
    pub fn save(&self) -> DoumResult<()> {
        // validate API key
        if self.api_key.is_empty() {
            return Err(DoumError::Config("API key cannot be empty".to_string()));
        }

        // 1 rank: keyring
        let entry = Entry::new("openai", "doum-cli")
            .map_err(|e| DoumError::Config(format!("Failed to access keyring: {}", e)))?;
        let value = serde_json::to_string(self)
            .map_err(|e| DoumError::Config(format!("Failed to serialize secret to JSON: {}", e)))?;
        entry
            .set_password(&value)
            .map_err(|e| DoumError::Config(format!("Failed to save to keyring: {}", e)))?;

        // 2 rank: environment variable (cannot set programmatically, just warn)
        if std::env::var("OPENAI_SECRET").is_ok() {
            eprintln!(
                "Warning: OpenAI secret is also set in environment variable. Please ensure consistency."
            );
        }
        Ok(())
    }

    /// Load secret from Keyring or Environment variable
    pub fn load() -> DoumResult<Self> {
        // 1 rank: Keyring
        if let Ok(entry) = Entry::new("openai", "doum-cli")
            && let Ok(value) = entry.get_password()
        {
            return Self::from_string(&value);
        }

        // 2 rank: Environment variable
        if let Ok(value) = std::env::var("OPENAI_SECRET") {
            return Self::from_string(&value);
        }

        Err(DoumError::Config(
            "OpenAI secret not found in keyring or environment variable".to_string(),
        ))
    }

    /// Delete secret from Keyring and warn about Environment variable
    pub fn delete() -> DoumResult<()> {
        // 1 rank: Keyring
        if let Ok(entry) = Entry::new("openai", "doum-cli") {
            entry
                .delete_credential()
                .map_err(|e| DoumError::Config(format!("Failed to delete from keyring: {}", e)))?;
        }

        // 2 rank: Environment variable (cannot delete programmatically, just warn)
        if std::env::var("OPENAI_SECRET").is_ok() {
            eprintln!(
                "Warning: OpenAI secret is also set in environment variable. Please remove it manually if needed."
            );
        }

        Ok(())
    }

    /// Get masked API key for display
    pub fn masked(&self) -> String {
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
