use super::ask::handle_ask_command;
use super::suggest::handle_suggest_command;
use crate::core::select_mode;
use crate::llm::create_client;
use crate::system::{get_system_info, load_config};
use anyhow::Result;
use cliclack::spinner;

pub async fn handle_auto_command(input: &str) -> Result<()> {
    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    let sp = spinner();
    sp.start("[AUTO MODE] Selecting mode...");

    let mode_response = select_mode(input, client.as_ref(), &system_info, &config).await?;

    sp.stop("");

    // Execute based on selected mode
    match mode_response.mode.as_str() {
        "ask" => handle_ask_command(input).await,
        "suggest" => handle_suggest_command(input).await,
        unknown => {
            println!("⚠️  Unknown mode: {}", unknown);
            println!("💡 Falling back to Ask mode.\n");
            handle_ask_command(input).await
        }
    }
}
