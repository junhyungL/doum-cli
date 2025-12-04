use anyhow::{Context, Result};
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
    pub fn save<T: ProviderSecret>(service_name: &str, secret: &T) -> Result<()> {
        // Validate secret
        secret.validate()?;

        // Save to Keyring
        let entry =
            Entry::new(service_name, SECRET_SERVICE_NAME).context("Failed to access keyring")?;
        let value = serde_json::to_string(secret).context("Failed to serialize secret to JSON")?;
        entry
            .set_password(&value)
            .context("Failed to save to keyring")?;

        Ok(())
    }

    /// Load secret from Keyring
    pub fn load<T: ProviderSecret>(service_name: &str) -> Result<T> {
        let entry =
            Entry::new(service_name, SECRET_SERVICE_NAME).context("Failed to access keyring")?;

        let secret_json = entry
            .get_password()
            .context("Failed to retrieve from keyring")?;

        serde_json::from_str(&secret_json).context("Failed to parse secret")
    }

    /// Delete secret from Keyring
    pub fn delete(service_name: &str) -> Result<()> {
        let entry =
            Entry::new(service_name, SECRET_SERVICE_NAME).context("Failed to access keyring")?;
        entry
            .delete_credential()
            .context("Failed to delete from keyring")?;
        Ok(())
    }
}
