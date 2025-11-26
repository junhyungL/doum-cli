use crate::cli::ui::{
    CommandAction, confirm_execution, copy_to_clipboard, create_spinner, finish_spinner,
    prompt_for_command_selection,
};
use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::llm::retry_with_parse;
use crate::llm::{LLMClient, PromptBuilder, parse_suggest};
use crate::system::Config;
use crate::system::SystemInfo;
use crate::system::error::DoumResult;
use crate::tools::execute;

/// Provide command suggestions using Suggest mode
/// Returns the selected command if copied or executed
pub async fn handle_suggest(
    request: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> DoumResult<Option<String>> {
    let builder = PromptBuilder::new(system_info.clone());

    // Start spinner
    let spinner = create_spinner("AI is generating commands...");

    // Call LLM to get suggestions
    let response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_suggest(),
                messages: vec![Message::user(request)],
                use_websearch: config.llm.use_web_search,
            };
            Box::pin(client.generate(request))
        },
        parse_suggest,
        config.llm.max_retries,
    )
    .await?;

    // End spinner
    finish_spinner(spinner, None);

    if response.suggestions.is_empty() {
        println!("⚠️  No commands to suggest.\n");
        return Ok(None);
    }

    // Prompt user to select action for suggested commands
    match prompt_for_command_selection(&response.suggestions)? {
        Some((index, action)) => {
            let selected = &response.suggestions[index];

            match action {
                CommandAction::Copy => {
                    // Copy to clipboard
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
                    // Execute command
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
