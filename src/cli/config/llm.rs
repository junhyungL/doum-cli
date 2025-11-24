// LLM Configuration Menu

use crate::cli::config::{context, provider};
use crate::cli::menu::Menu;
use crate::cli::tui::ratatui_select;
use crate::cli::ui::prompt_number_input;
use crate::system::config::{load_config, save_config};
use crate::system::error::Result;

/// LLM configuration menu
pub fn run_llm_menu() -> Result<()> {
    loop {
        let config = load_config()?;

        let menu = Menu::builder("LLM Settings")
            .with_back()
            .add_item(
                "provider",
                format!("Provider [current: {}]", config.llm.provider),
                "Select and configure the default LLM provider",
            )
            .add_item(
                "providers",
                "Providers Configuration",
                "Configure individual provider settings (API keys, models, etc.)",
            )
            .add_item(
                "context",
                "Context Settings",
                "Configure context collection settings",
            )
            .add_item(
                "timeout",
                format!("Timeout [current: {}s]", config.llm.timeout),
                format!(
                    "Request timeout in seconds (current: {}s)",
                    config.llm.timeout
                ),
            )
            .add_item(
                "retry",
                format!("Max Retry [current: {}]", config.llm.max_retries),
                format!(
                    "Maximum retry attempts (current: {})",
                    config.llm.max_retries
                ),
            )
            .add_item(
                "thinking",
                format!(
                    "Use Thinking [current: {}]",
                    if config.llm.use_thinking {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ),
                "Enable/disable extended thinking for complex tasks",
            )
            .add_item(
                "web_search",
                format!(
                    "Use Web Search [current: {}]",
                    if config.llm.use_web_search {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ),
                "Enable/disable web search capability",
            )
            .build();

        let selected = ratatui_select(
            "🤖 LLM Configuration",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None,
        )?;

        match selected {
            Some(item) => match item.id.as_str() {
                "back" => break,
                "exit" => {
                    println!("\n👋 Goodbye!");
                    std::process::exit(0);
                }
                "provider" => {
                    if let Err(e) = provider::run_provider_selection() {
                        eprintln!("❌ Provider configuration failed: {}", e);
                    }
                }
                "providers" => {
                    if let Err(e) = provider::run_providers_menu() {
                        eprintln!("❌ Providers configuration failed: {}", e);
                    }
                }
                "context" => {
                    if let Err(e) = context::run_context_menu() {
                        eprintln!("❌ Context configuration failed: {}", e);
                    }
                }
                "timeout" => {
                    edit_timeout()?;
                }
                "retry" => {
                    edit_retry()?;
                }
                "thinking" => {
                    toggle_thinking()?;
                }
                "web_search" => {
                    toggle_web_search()?;
                }
                _ => {
                    println!("\n⚠️  Unknown option");
                }
            },
            None => break,
        }
    }

    Ok(())
}

/// Edit timeout setting
fn edit_timeout() -> Result<()> {
    let mut config = load_config()?;

    let new_timeout = prompt_number_input::<u64>("Timeout (seconds)", Some(config.llm.timeout))?;

    config.llm.timeout = new_timeout;
    save_config(&config)?;

    println!("✅ Setting saved: timeout = {}s", new_timeout);

    Ok(())
}

/// Edit retry setting
fn edit_retry() -> Result<()> {
    let mut config = load_config()?;

    let new_retries =
        prompt_number_input::<u32>("Maximum retry attempts", Some(config.llm.max_retries))?;

    config.llm.max_retries = new_retries;
    save_config(&config)?;

    println!("✅ Setting saved: max_retries = {}", new_retries);

    Ok(())
}

/// Toggle use_thinking setting
fn toggle_thinking() -> Result<()> {
    let mut config = load_config()?;

    config.llm.use_thinking = !config.llm.use_thinking;
    save_config(&config)?;

    println!(
        "✅ Use thinking: {}",
        if config.llm.use_thinking {
            "enabled"
        } else {
            "disabled"
        }
    );

    Ok(())
}

/// Toggle use_web_search setting
fn toggle_web_search() -> Result<()> {
    let mut config = load_config()?;

    config.llm.use_web_search = !config.llm.use_web_search;
    save_config(&config)?;

    println!(
        "✅ Web search: {}",
        if config.llm.use_web_search {
            "enabled"
        } else {
            "disabled"
        }
    );

    Ok(())
}
