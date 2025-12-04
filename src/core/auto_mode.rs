use crate::llm::client::LLMRequest;
use crate::llm::retry_with_parse;
use crate::llm::{AutoResponse, LLMClient, LLMMessage, PromptBuilder, parse_auto_mode};
use crate::system::{Config, SystemInfo};
use anyhow::Result;

/// Select mode automatically based on input
/// Returns the selected mode and original input
pub async fn select_mode(
    input: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> Result<AutoResponse> {
    let builder = PromptBuilder::new(system_info.clone());

    // Request mode selection
    let response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_auto_mode(),
                messages: vec![LLMMessage::user(input)],
            };
            Box::pin(client.generate(request))
        },
        parse_auto_mode,
        config.llm.max_retries,
    )
    .await?;

    Ok(response)
}
