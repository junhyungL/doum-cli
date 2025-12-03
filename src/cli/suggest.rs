use crate::core::handle_suggest;
use crate::llm::create_client;
use crate::system::{get_system_info, load_config};
use anyhow::Result;

pub async fn handle_suggest_command(request: &str) -> Result<()> {
    use cliclack::{select, spinner};

    let config = load_config()?;
    let client = create_client(&config.llm)?;
    let system_info = get_system_info();

    let sp = spinner();
    sp.start("AI is generating commands...");

    let response = handle_suggest(request, client.as_ref(), &system_info, &config).await?;

    sp.stop("✅ Commands ready");

    // Handle empty suggestions
    if response.suggestions.is_empty() {
        println!("\n⚠️  No commands to suggest.\n");
        return Ok(());
    }

    // Format command items for selection
    let items: Vec<_> = response
        .suggestions
        .iter()
        .map(|s| (s, s.cmd.as_str(), s.description.as_str()))
        .collect();

    let selected = select("Select a command")
        .items(&items)
        .interact()?;

    // Copy to clipboard
    copy_to_clipboard(&selected.cmd)?;

    cliclack::outro("✅ Command copied to clipboard!")?;

    Ok(())
}

/// Copy text to clipboard using arboard
fn copy_to_clipboard(text: &str) -> Result<()> {
    use arboard::Clipboard;

    let mut clipboard = Clipboard::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard: {}", e))?;

    clipboard
        .set_text(text)
        .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}
