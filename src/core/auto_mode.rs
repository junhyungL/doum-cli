use crate::llm::client::LLMRequest;
use crate::llm::retry_with_parse;
use crate::llm::{LLMClient, Message, ModeSelectResponse, PromptBuilder, parse_mode_select};
use crate::system::{Config, SystemInfo};
use anyhow::Result;

/// Select mode automatically based on input
/// Returns the selected mode and original input
pub async fn select_mode(
    input: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> Result<ModeSelectResponse> {
    let builder = PromptBuilder::new(system_info.clone());

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

    Ok(response)
}
