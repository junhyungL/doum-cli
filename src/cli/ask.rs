use crate::core::handle_ask;
use crate::llm::create_client;
use crate::system::{get_system_info, load_config};
use anyhow::Result;

pub async fn handle_ask_command(question: &str) -> Result<()> {
    use cliclack::spinner;

    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    let sp = spinner();
    sp.start("AI is generating an answer...");

    let response = handle_ask(question, client.as_ref(), &system_info, &config).await?;

    sp.stop("✅ Answer ready");

    println!("\n{}\n", response);

    Ok(())
}
