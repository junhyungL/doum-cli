use crate::llm::client::LLMRequest;
use crate::llm::{LLMClient, LLMMessage, PromptBuilder};
use crate::system::{Config, SystemInfo};
use anyhow::Result;

/// Provide answer of the question using Ask mode
pub async fn handle_ask(
    question: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    _config: &Config,
) -> Result<String> {
    let builder = PromptBuilder::new(system_info.clone());

    let request = LLMRequest {
        system: builder.build_ask(),
        messages: vec![LLMMessage::user(question)],
    };

    // Generate response
    let response = client.generate(request).await?;

    Ok(response)
}
