use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Select Mode Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoResponse {
    pub mode: String,
    pub reason: String,
}

/// Ask Mode Response
pub type AskResponse = String;

/// Command Suggestion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub cmd: String,
    pub description: String,
}

/// Suggest Mode Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestResponse {
    pub suggestions: Vec<CommandSuggestion>,
}

/// parse Auto Mode response
pub fn parse_auto_mode(json_str: &str) -> Result<AutoResponse> {
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned).context("Failed to parse Auto Mode response")
}

/// parse Suggest response
pub fn parse_suggest(json_str: &str) -> Result<SuggestResponse> {
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned).context("Failed to parse Suggest response")
}

/// Extract JSON content from text (handles code blocks and surrounding text)
fn extract_json(text: &str) -> String {
    let text = text.trim();

    // ```json ... ``` or ``` ... ```
    if let Some(start) = text.find("```")
        && let Some(end) = text[start + 3..].find("```")
    {
        let json_block = &text[start + 3..start + 3 + end];
        let json_content = if let Some(newline) = json_block.find('\n') {
            &json_block[newline + 1..]
        } else {
            json_block
        };
        return json_content.trim().to_string();
    }

    // Find first { ... } block
    if let Some(start) = text.find('{')
        && let Some(end) = text.rfind('}')
        && end > start
    {
        return text[start..=end].to_string();
    }

    // Return original text if no JSON found
    text.to_string()
}
