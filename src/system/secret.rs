use crate::llm::Provider;
use anyhow::Result;
use keyring::Entry;
use serde::{Deserialize, Serialize};

/// Trait for provider-specific secret implementations
pub trait ProviderSecret: Serialize + for<'de> Deserialize<'de> {
    /// Validate the secret before saving
    fn validate(&self) -> Result<()>;

    /// Get masked version for display
    fn masked(&self) -> String;
}

/// Constant service name for keyring entries
const SECRET_SERVICE_NAME: &str = "doum-cli";

/// Secret manager for handling keyring operations
pub struct SecretManager;

impl SecretManager {
    /// Save a secret value to the system keyring
    pub fn save<T: ProviderSecret>(provider: &Provider, secret: &T) -> Result<()> {
        // Validate secret
        secret.validate()?;

        // Save to Keyring
        let entry = Entry::new(provider.as_str(), SECRET_SERVICE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to access keyring: {}", e))?;
        let value = serde_json::to_string(secret)
            .map_err(|e| anyhow::anyhow!("Failed to serialize secret: {}", e))?;
        entry
            .set_password(&value)
            .map_err(|e| anyhow::anyhow!("Failed to save to keyring: {}", e))?;

        Ok(())
    }

    /// Load secret from Keyring
    pub fn load<T: ProviderSecret>(provider: &Provider) -> Result<T> {
        let entry = Entry::new(provider.as_str(), SECRET_SERVICE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to access keyring: {}", e))?;

        let secret_json = entry
            .get_password()
            .map_err(|e| anyhow::anyhow!("Failed to retrieve from keyring: {}", e))?;

        serde_json::from_str(&secret_json)
            .map_err(|e| anyhow::anyhow!("Failed to parse secret: {}", e))
    }

    /// Delete secret from Keyring
    pub fn delete(provider: &Provider) -> Result<()> {
        let entry = Entry::new(provider.as_str(), SECRET_SERVICE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to access keyring: {}", e))?;
        entry
            .delete_credential()
            .map_err(|e| anyhow::anyhow!("Failed to delete from keyring: {}", e))?;
        Ok(())
    }
}
