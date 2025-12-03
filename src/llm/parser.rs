use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Select Mode Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSelectResponse {
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

/// parse Mode Select response
pub fn parse_mode_select(json_str: &str) -> Result<ModeSelectResponse> {
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned).context("Failed to parse Mode Select response")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode_select() {
        let json = r#"{"mode":"ask","reason":"사용자가 질문을 했습니다"}"#;
        let result = parse_mode_select(json).unwrap();
        assert_eq!(result.mode, "ask");
        assert_eq!(result.reason, "사용자가 질문을 했습니다");
    }

    #[test]
    fn test_parse_mode_select_with_markdown() {
        let json = r#"
```json
{"mode":"execute","reason":"실행 요청"}
```
        "#;
        let result = parse_mode_select(json).unwrap();
        assert_eq!(result.mode, "execute");
    }

    #[test]
    fn test_parse_suggest() {
        let json = r#"
{
  "suggestions": [
    {"cmd": "ls -la", "description": "모든 파일 나열"},
    {"cmd": "dir", "description": "디렉터리 내용 보기"}
  ]
}
        "#;
        let result = parse_suggest(json).unwrap();
        assert_eq!(result.suggestions.len(), 2);
        assert_eq!(result.suggestions[0].cmd, "ls -la");
    }

    #[test]
    fn test_extract_json() {
        let text = "Some text before\n{\"key\":\"value\"}\nSome text after";
        let result = extract_json(text);
        assert_eq!(result, r#"{"key":"value"}"#);
    }

    #[test]
    fn test_extract_json_with_code_block() {
        let text = "```json\n{\"key\":\"value\"}\n```";
        let result = extract_json(text);
        assert_eq!(result, r#"{"key":"value"}"#);
    }
}
