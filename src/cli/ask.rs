use crate::core::handle_ask;
use crate::llm::create_client;
use crate::system::{get_system_info, load_config};
use anyhow::Result;
use cliclack::spinner;

pub async fn handle_ask_command(question: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    let sp = spinner();
    sp.start("[ASK MODE] Waiting for answer...");

    let response = handle_ask(question, client.as_ref(), &system_info, &config).await?;

    sp.stop("");

    println!("\n{}\n", response);

    Ok(())
}
