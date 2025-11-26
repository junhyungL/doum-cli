use crate::cli::ui::{create_spinner, finish_spinner};
use crate::core::{handle_ask, handle_suggest};
use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::llm::retry_with_parse;
use crate::llm::{LLMClient, PromptBuilder, parse_mode_select};
use crate::system::Config;
use crate::system::SystemInfo;
use crate::system::error::DoumResult;

/// Select mode automatically and execute
pub async fn select_mode(
    input: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> DoumResult<()> {
    let builder = PromptBuilder::new(system_info.clone());

    // Start spinner
    let spinner = create_spinner("Analyzing input...");

    // Request mode selection
    let response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_mode_select(),
                messages: vec![Message::user(input)],
                use_websearch: false,
            };
            Box::pin(client.generate(request))
        },
        parse_mode_select,
        config.llm.max_retries,
    )
    .await?;

    // End spinner
    finish_spinner(spinner, None);
    println!("📌 Selected mode: {} mode\n", response.mode);

    // Execute based on selected mode
    match response.mode.as_str() {
        "ask" => handle_ask(input, client, system_info, config).await,
        "suggest" => {
            handle_suggest(input, client, system_info, config).await?;
            Ok(())
        }
        "execute" => {
            // Execute 모드는 suggest로 통합됨 (suggest에서 실행 선택 가능)
            handle_suggest(input, client, system_info, config).await?;
            Ok(())
        }
        unknown => {
            println!("⚠️  Unknown mode: {}", unknown);
            println!("💡 Falling back to Ask mode.\n");
            handle_ask(input, client, system_info, config).await
        }
    }
}
