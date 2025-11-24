// Logging Configuration Menu

use crate::system::error::Result;
use crate::system::config::{load_config, save_config};
use crate::cli::menu::{Menu, MenuItem};
use crate::cli::tui::ratatui_select;

/// Logging configuration menu
pub fn run_logging_menu() -> Result<()> {
    loop {
        let config = load_config()?;
        
        // Create menu with current values in descriptions
        let menu = Menu::builder("Logging Settings")
            .with_back()
            .add_item(
                "enabled",
                &format!("Enabled [current: {}]", config.logging.enabled),
                &format!("Enable/disable logging (currently: {})", config.logging.enabled)
            )
            .add_item(
                "level",
                &format!("Level [current: {}]", config.logging.level),
                &format!("Log level (currently: {})", config.logging.level)
            )
            .build();
        
        let selected = ratatui_select(
            "📝 Logging Configuration",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None
        )?;
        
        match selected {
            Some(item) => {
                match item.id.as_str() {
                    "back" => break,
                    "exit" => {
                        println!("\n👋 Goodbye!");
                        std::process::exit(0);
                    }
                    "enabled" => {
                        edit_logging_enabled()?;
                    }
                    "level" => {
                        edit_logging_level()?;
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

/// Edit logging enabled setting
fn edit_logging_enabled() -> Result<()> {
    let mut config = load_config()?;
    let current = config.logging.enabled;
    
    let menu_items = vec![
        MenuItem::new("true", "Enabled", "Enable logging"),
        MenuItem::new("false", "Disabled", "Disable logging"),
    ];
    
    let current_str = if current { "true" } else { "false" };
    
    let selected = ratatui_select(
        "Enable Logging",
        &menu_items,
        Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
        Some(current_str)
    )?;
    
    if let Some(item) = selected {
        config.logging.enabled = item.id == "true";
        save_config(&config)?;
        println!("✅ Setting saved: enabled = {}", config.logging.enabled);
    }
    
    Ok(())
}

/// Edit log level
fn edit_logging_level() -> Result<()> {
    let mut config = load_config()?;
    
    let menu_items = vec![
        MenuItem::new("debug", "Debug", "Debug level - Most verbose"),
        MenuItem::new("info", "Info", "Info level - General information"),
        MenuItem::new("warn", "Warn", "Warn level - Warnings only"),
        MenuItem::new("error", "Error", "Error level - Errors only"),
    ];
    
    let selected = ratatui_select(
        "Select Log Level",
        &menu_items,
        Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
        Some(&config.logging.level)
    )?;
    
    if let Some(item) = selected {
        config.logging.level = item.id.clone();
        save_config(&config)?;
        println!("✅ Setting saved: level = {}", config.logging.level);
    }
    
    Ok(())
}
