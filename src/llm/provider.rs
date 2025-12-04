use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// LLM Provider enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    OpenAI,
    Anthropic,
}

impl Provider {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::OpenAI => "openai",
            Provider::Anthropic => "anthropic",
        }
    }

    /// Get display representation
    pub fn as_display(&self) -> String {
        match self {
            Provider::OpenAI => "OpenAI".to_string(),
            Provider::Anthropic => "Anthropic".to_string(),
        }
    }

    /// Get all available providers
    pub fn all() -> Vec<Provider> {
        vec![Provider::OpenAI, Provider::Anthropic]
    }

    /// Get all provider names as strings
    pub fn all_as_str() -> Vec<String> {
        Self::all().iter().map(|p| p.as_str().to_string()).collect()
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Provider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(Provider::OpenAI),
            "anthropic" => Ok(Provider::Anthropic),
            _ => anyhow::bail!(
                "Unknown provider: {}. Available: {}",
                s,
                Self::all_as_str().join(", ")
            ),
        }
    }
}
