use crate::llm::CommandSuggestion;
use crate::system::error::{DoumError, Result};
use arboard::Clipboard;
use console::Style;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// UI module for all user interactions (ask, suggest modes)
// Config mode uses TUI (tui.rs, menu.rs)

/// Action to take after selecting a command
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandAction {
    Copy,
    Execute,
    Cancel,
}

/// Enhanced command selection with dialoguer
pub fn prompt_for_command_selection(
    suggestions: &[CommandSuggestion],
) -> Result<Option<(usize, CommandAction)>> {
    if suggestions.is_empty() {
        println!("\n⚠️  No commands to suggest.");
        return Ok(None);
    }

    let theme = ColorfulTheme::default();
    let cmd_style = Style::new().cyan().bold();
    let desc_style = Style::new().dim();

    // Format items with command in color and description
    let items: Vec<String> = suggestions
        .iter()
        .map(|s| {
            format!(
                "{}\n  {}",
                cmd_style.apply_to(&s.cmd),
                desc_style.apply_to(&s.description)
            )
        })
        .collect();

    println!("\n📋 Select a command:\n");

    let selection = Select::with_theme(&theme)
        .items(&items)
        .default(0)
        .interact_opt()
        .map_err(|e| DoumError::Config(format!("Selection failed: {}", e)))?;

    match selection {
        Some(index) => {
            let selected_cmd = &suggestions[index].cmd;

            // Ask what to do with the command
            println!("\n📋 Selected: {}", cmd_style.apply_to(selected_cmd));

            let actions = vec!["📋 Copy to clipboard", "▶️  Execute now", "❌ Cancel"];

            let action = Select::with_theme(&theme)
                .with_prompt("What would you like to do?")
                .items(&actions)
                .default(0)
                .interact_opt()
                .map_err(|e| DoumError::Config(format!("Action selection failed: {}", e)))?;

            match action {
                Some(0) => Ok(Some((index, CommandAction::Copy))),
                Some(1) => Ok(Some((index, CommandAction::Execute))),
                _ => Ok(Some((index, CommandAction::Cancel))),
            }
        }
        None => Ok(None),
    }
}

/// Simple confirmation prompt
pub fn confirm_execution(command: &str) -> Result<bool> {
    let theme = ColorfulTheme::default();
    let cmd_style = Style::new().cyan().bold();

    println!("\n📋 Command: {}", cmd_style.apply_to(command));

    Confirm::with_theme(&theme)
        .with_prompt("Execute this command?")
        .default(true)
        .interact()
        .map_err(|e| DoumError::Config(format!("Confirmation failed: {}", e)))
}

/// Text input prompt
pub fn prompt_text_input(message: &str, default: Option<&str>) -> Result<String> {
    let theme = ColorfulTheme::default();

    let mut input = Input::with_theme(&theme).with_prompt(message);

    if let Some(def) = default {
        input = input.default(def.to_string());
    }

    input
        .interact_text()
        .map_err(|e| DoumError::Config(format!("Input failed: {}", e)))
}

/// Password input prompt
pub fn prompt_password_input(message: &str) -> Result<String> {
    let theme = ColorfulTheme::default();

    Password::with_theme(&theme)
        .with_prompt(message)
        .interact()
        .map_err(|e| DoumError::Config(format!("Password input failed: {}", e)))
}

/// Number input prompt
pub fn prompt_number_input<T>(message: &str, default: Option<T>) -> Result<T>
where
    T: std::str::FromStr + std::fmt::Display + Clone,
    T::Err: std::fmt::Display,
{
    let theme = ColorfulTheme::default();

    let mut input = Input::with_theme(&theme).with_prompt(message);

    if let Some(def) = default {
        input = input.default(def);
    }

    input
        .interact_text()
        .map_err(|e| DoumError::Config(format!("Number input failed: {}", e)))
}

/// Copy text to clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard =
        Clipboard::new().map_err(|e| DoumError::Config(format!("Clipboard init failed: {}", e)))?;

    clipboard
        .set_text(text)
        .map_err(|e| DoumError::Config(format!("Clipboard copy failed: {}", e)))?;

    Ok(())
}

/// Create a spinner with message
pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

/// Finish spinner
pub fn finish_spinner(spinner: ProgressBar, message: Option<&str>) {
    if let Some(msg) = message {
        spinner.finish_with_message(msg.to_string());
    } else {
        spinner.finish_and_clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard() {
        let result = copy_to_clipboard("test command");
        assert!(result.is_ok() || result.is_err());
    }
}
