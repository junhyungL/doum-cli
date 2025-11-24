use doum_cli::llm::{parse_mode_select, parse_suggest, parse_execute};

#[test]
fn test_parse_mode_select() {
    let json = r#"{"mode": "ask", "reason": "This is a question"}"#;
    let result = parse_mode_select(json).unwrap();
    assert_eq!(result.mode, "ask");
}

#[test]
fn test_parse_suggest() {
    let json = r#"{
        "suggestions": [
            {"cmd": "ls -la", "description": "List all files"}
        ]
    }"#;
    let result = parse_suggest(json).unwrap();
    assert_eq!(result.suggestions.len(), 1);
    assert_eq!(result.suggestions[0].cmd, "ls -la");
}

#[test]
fn test_parse_execute() {
    let json = r#"{
        "command": "echo test",
        "description": "Print test",
        "is_dangerous": false
    }"#;
    let result = parse_execute(json).unwrap();
    assert_eq!(result.command, "echo test");
    assert_eq!(result.is_dangerous, false);
}
