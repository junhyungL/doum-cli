use crate::llm::client::LLMRequest;
use crate::llm::retry_with_parse;
use crate::llm::{LLMClient, LLMMessage, PromptBuilder, SuggestResponse, parse_suggest};
use crate::system::{Config, SystemInfo};
use anyhow::Result;

/// Provide command suggestions using Suggest mode
/// Returns the suggestion response for CLI to handle
pub async fn handle_suggest(
    request: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    config: &Config,
) -> Result<SuggestResponse> {
    let builder = PromptBuilder::new(system_info.clone());

    // Call LLM to get suggestions
    let response = retry_with_parse(
        || {
            let request = LLMRequest {
                system: builder.build_suggest(),
                messages: vec![LLMMessage::user(request)],
            };
            Box::pin(client.generate(request))
        },
        parse_suggest,
        config.llm.max_retries,
    )
    .await?;

    Ok(response)
}
