// Config Main Menu

use crate::system::error::Result;
use crate::cli::menu::Menu;
use crate::cli::tui::ratatui_select;
use crate::cli::config::{logging, llm};

/// Run config interactive mode
pub fn run_config_interactive() -> Result<()> {
    loop {
        let menu = Menu::builder("Configuration Menu")
            .with_back()
            .add_item("llm", "LLM", "LLM related settings")
            .add_item("logging", "Logging", "Logging configuration")
            .build();
        
        let selected = ratatui_select(
            "⚙️  Configuration Menu",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None
        )?;
        
        match selected {
            Some(item) => {
                match item.id.as_str() {
                    "back" => break,
                    "llm" => {
                        if let Err(e) = llm::run_llm_menu() {
                            eprintln!("❌ LLM configuration failed: {}", e);
                        }
                    }
                    "logging" => {
                        if let Err(e) = logging::run_logging_menu() {
                            eprintln!("❌ Logging configuration failed: {}", e);
                        }
                    }
                    "exit" => {
                        println!("\n👋 Goodbye!");
                        std::process::exit(0);
                    }
                    _ => {
                        println!("\n⚠️  Unknown option");
                    }
                }
            }
            None => break,
        }
    }
    
    Ok(())
}
