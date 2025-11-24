// Context Configuration Menu

use crate::cli::menu::Menu;
use crate::cli::tui::ratatui_select;
use crate::cli::ui::prompt_number_input;
use crate::system::config::{load_config, save_config};
use crate::system::error::Result;

/// Context configuration menu
pub fn run_context_menu() -> Result<()> {
    loop {
        let config = load_config()?;

        let menu = Menu::builder("Context Settings")
            .with_back()
            .add_item(
                "max_lines",
                format!("Max Lines [current: {}]", config.llm.context.max_lines),
                format!(
                    "Maximum number of lines to include (current: {})",
                    config.llm.context.max_lines
                ),
            )
            .add_item(
                "max_size_kb",
                format!("Max Size [current: {} KB]", config.llm.context.max_size_kb),
                format!(
                    "Maximum context size in KB (current: {})",
                    config.llm.context.max_size_kb
                ),
            )
            .build();

        let selected = ratatui_select(
            "📄 Context Configuration",
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
                "max_lines" => {
                    edit_max_lines()?;
                }
                "max_size_kb" => {
                    edit_max_size_kb()?;
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

/// Edit max_lines setting
fn edit_max_lines() -> Result<()> {
    let mut config = load_config()?;

    let new_value =
        prompt_number_input::<usize>("Maximum lines", Some(config.llm.context.max_lines))?;

    config.llm.context.max_lines = new_value;
    save_config(&config)?;

    println!("✅ Setting saved: max_lines = {}", new_value);

    Ok(())
}

/// Edit max_size_kb setting
fn edit_max_size_kb() -> Result<()> {
    let mut config = load_config()?;

    let new_value =
        prompt_number_input::<usize>("Maximum size (KB)", Some(config.llm.context.max_size_kb))?;

    config.llm.context.max_size_kb = new_value;
    save_config(&config)?;

    println!("✅ Setting saved: max_size_kb = {}", new_value);

    Ok(())
}
