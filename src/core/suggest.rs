use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::system::error::Result;
use crate::system::Config;
use crate::system::SystemInfo;
use crate::llm::{LLMClient, PromptBuilder, parse_suggest};
use crate::cli::ui::{prompt_for_command_selection, copy_to_clipboard, confirm_execution, CommandAction, create_spinner, finish_spinner};
use crate::llm::retry_with_parse;
use crate::tools::execute;

/// Suggest 모드 핵심 로직
/// 
/// 사용자의 요청에 대해 여러 명령 후보를 제안하고 선택받습니다.
/// 선택된 명령은 클립보드에 복사하거나 즉시 실행할 수 있습니다.
pub async fn handle_suggest(
    request: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> Result<Option<String>> {
    // 프롬프트 빌더 생성
    let builder = PromptBuilder::new(system_info.clone());
    
    // 스피너 시작
    let spinner = create_spinner("AI is generating commands...");
    
    // LLM 호출 및 재시도 파싱
    let response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_suggest(),
                messages: vec![Message::user(request)],
                use_websearch: config.llm.use_web_search,
            };
            Box::pin(client.generate(request))
        },
        |content| parse_suggest(content),
        config.llm.max_retries,
    )
    .await?;
    
    // 스피너 완료
    finish_spinner(spinner, None);
    
    if response.suggestions.is_empty() {
        println!("⚠️  No commands to suggest.\n");
        return Ok(None);
    }
    
    // 사용자 선택 받기 (dialoguer 사용)
    match prompt_for_command_selection(&response.suggestions)? {
        Some((index, action)) => {
            let selected = &response.suggestions[index];
            
            match action {
                CommandAction::Copy => {
                    // 클립보드에 복사
                    match copy_to_clipboard(&selected.cmd) {
                        Ok(_) => {
                            println!("\n✅ Command copied to clipboard!");
                            println!("📋 {}", selected.cmd);
                            println!("\n💡 Press Ctrl+V to paste in terminal.\n");
                        }
                        Err(e) => {
                            println!("\n⚠️  Failed to copy to clipboard: {}", e);
                            println!("📋 Command: {}\n", selected.cmd);
                        }
                    }
                    Ok(Some(selected.cmd.clone()))
                }
                CommandAction::Execute => {
                    // 실행 확인
                    if confirm_execution(&selected.cmd)? {
                        println!("\n▶️  Executing command...\n");
                        
                        match execute(&selected.cmd, system_info) {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                
                                println!("{}", stdout);
                                if !stderr.is_empty() {
                                    eprintln!("\nStderr:\n{}", stderr);
                                }
                                println!("\n✅ Command executed successfully.\n");
                            }
                            Err(e) => {
                                eprintln!("\n❌ Execution failed: {}\n", e);
                            }
                        }
                    } else {
                        println!("\n❌ Execution cancelled.\n");
                    }
                    Ok(Some(selected.cmd.clone()))
                }
                CommandAction::Cancel => {
                    println!("\n❌ Cancelled.\n");
                    Ok(None)
                }
            }
        }
        None => {
            println!("\n❌ Cancelled.\n");
            Ok(None)
        }
    }
}