use crate::cli::ui::{create_spinner, finish_spinner};
use crate::llm::Message;
use crate::llm::client::LLMRequest;
use crate::llm::{LLMClient, PromptBuilder};
use crate::system::Config;
use crate::system::SystemInfo;
use crate::system::error::DoumResult;

/// Provide answer of the question using Ask mode
pub async fn handle_ask(
    question: &str,
    client: &dyn LLMClient,
    system_info: &SystemInfo,
    _config: &Config,
) -> DoumResult<()> {
    let builder = PromptBuilder::new(system_info.clone());

    let request = LLMRequest {
        system: builder.build_ask(),
        messages: vec![Message::user(question)],
        use_websearch: _config.llm.use_web_search,
    };

    // Start spinner
    let spinner = create_spinner("AI is generating an answer...");

    // Generate response
    let response = client.generate(request).await?;

    // End spinner
    finish_spinner(spinner, None);

    // 응답 출력
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}\n", response);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok(())
}
