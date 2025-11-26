use crate::system::error::{DoumError, DoumResult};
use serde::{Deserialize, Serialize};

/// 모드 선택 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSelectResponse {
    pub mode: String,
    pub reason: String,
}

/// Ask 모드 응답 (단순 문자열)
pub type AskResponse = String;

/// 명령 제안
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub cmd: String,
    pub description: String,
}

/// Suggest 모드 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestResponse {
    pub suggestions: Vec<CommandSuggestion>,
}

/// Execute 모드 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub command: String,
    pub description: String,
    pub is_dangerous: bool,
}

/// 모드 선택 응답 파싱
pub fn parse_mode_select(json_str: &str) -> DoumResult<ModeSelectResponse> {
    // JSON 추출 시도 (markdown 코드 블록 제거)
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned)
        .map_err(|e| DoumError::Parse(format!("모드 선택 응답 파싱 실패: {}", e)))
}

/// Suggest 응답 파싱
pub fn parse_suggest(json_str: &str) -> DoumResult<SuggestResponse> {
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned)
        .map_err(|e| DoumError::Parse(format!("Suggest 응답 파싱 실패: {}", e)))
}

/// Execute 응답 파싱
pub fn parse_execute(json_str: &str) -> DoumResult<ExecuteResponse> {
    let cleaned = extract_json(json_str);

    serde_json::from_str(&cleaned)
        .map_err(|e| DoumError::Parse(format!("Execute 응답 파싱 실패: {}", e)))
}

/// JSON 추출 (markdown 코드 블록이나 불필요한 텍스트 제거)
fn extract_json(text: &str) -> String {
    let text = text.trim();

    // ```json ... ``` 또는 ``` ... ``` 형식 처리
    if let Some(start) = text.find("```")
        && let Some(end) = text[start + 3..].find("```")
    {
        let json_block = &text[start + 3..start + 3 + end];
        // ```json 같은 언어 태그 제거
        let json_content = if let Some(newline) = json_block.find('\n') {
            &json_block[newline + 1..]
        } else {
            json_block
        };
        return json_content.trim().to_string();
    }

    // { 로 시작하는 JSON 찾기
    if let Some(start) = text.find('{')
        && let Some(end) = text.rfind('}')
        && end > start
    {
        return text[start..=end].to_string();
    }

    // 그대로 반환
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
    fn test_parse_execute() {
        let json = r#"
{
  "command": "echo hello",
  "description": "hello 출력",
  "is_dangerous": false
}
        "#;
        let result = parse_execute(json).unwrap();
        assert_eq!(result.command, "echo hello");
        assert!(!result.is_dangerous);
    }

    #[test]
    fn test_parse_execute_dangerous() {
        let json = r#"{"command":"rm -rf /","description":"위험","is_dangerous":true}"#;
        let result = parse_execute(json).unwrap();
        assert!(result.is_dangerous);
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
