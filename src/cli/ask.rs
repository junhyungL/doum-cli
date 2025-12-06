use crate::llm::client::LLMRequest;
use crate::llm::{LLMMessage, PromptBuilder, create_client};
use crate::system::{get_system_info, load_config};
use anyhow::Result;
use cliclack::spinner;

pub async fn handle_ask_command(question: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();
    let builder = PromptBuilder::new(system_info.clone());

    let sp = spinner();
    sp.start("[ASK MODE] Waiting for answer...");

    let request = LLMRequest {
        system: builder.build_ask(),
        messages: vec![LLMMessage::user(question)],
    };

    let response = client.generate(request).await?;

    sp.stop("");

    println!("\n{}\n", response);

    Ok(())
}
