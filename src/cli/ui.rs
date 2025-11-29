use crate::llm::CommandSuggestion;
use anyhow::{Context, Result};
use arboard::Clipboard;
use console::Style;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

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
        .context("Selection failed")?;

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
                .context("Action selection failed")?;

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
        .context("Confirmation failed")
}

/// Text input prompt
pub fn prompt_text_input(message: &str, default: Option<&str>) -> Result<String> {
    let theme = ColorfulTheme::default();

    let mut input = Input::with_theme(&theme)
        .with_prompt(message)
        .allow_empty(true);

    if let Some(def) = default {
        input = input.default(def.to_string());
    }

    input.interact_text().context("Input failed")
}

/// Password input prompt
pub fn prompt_password_input(message: &str) -> Result<String> {
    let theme = ColorfulTheme::default();

    Password::with_theme(&theme)
        .with_prompt(message)
        .interact()
        .context("Password input failed")
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

    input.interact_text().context("Number input failed")
}

/// Copy text to clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("Clipboard init failed")?;

    clipboard.set_text(text).context("Clipboard copy failed")?;

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

/// Display Ask mode response with formatting
pub fn display_ask_response(response: &str) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}\n", response);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Display selected mode in Auto mode
pub fn display_selected_mode(mode: &str) {
    println!("📌 Selected mode: {} mode\n", mode);
}

/// Handle suggest command response (selection, copy, execution)
/// Returns Ok(()) if completed successfully
pub fn handle_suggest_response(
    suggestions: &[crate::llm::CommandSuggestion],
    system_info: &crate::system::SystemInfo,
) -> Result<()> {
    use crate::tools::execute_command;

    // Handle empty suggestions
    if suggestions.is_empty() {
        println!("⚠️  No commands to suggest.\n");
        return Ok(());
    }

    // Prompt user to select action
    match prompt_for_command_selection(suggestions)? {
        Some((index, action)) => {
            let selected = &suggestions[index];

            match action {
                CommandAction::Copy => match copy_to_clipboard(&selected.cmd) {
                    Ok(_) => {
                        println!("\n✅ Command copied to clipboard!");
                        println!("📋 {}", selected.cmd);
                        println!("\n💡 Press Ctrl+V to paste in terminal.\n");
                    }
                    Err(e) => {
                        println!("\n⚠️  Failed to copy to clipboard: {}", e);
                        println!("📋 Command: {}\n", selected.cmd);
                    }
                },
                CommandAction::Execute => {
                    if confirm_execution(&selected.cmd)? {
                        println!("\n▶️  Executing command...\n");

                        match execute_command(&selected.cmd, system_info, None) {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let stderr = String::from_utf8_lossy(&output.stderr);

                                println!("{}", stdout);
                                if !stderr.is_empty() {
                                    eprintln!("\nStderr:\n{}", stderr);
                                }
                                println!("\n✅ Command executed successfully.\n");
                            }
                            Err(e) => {
                                eprintln!("\n❌ Execution failed: {}\n", e);
                            }
                        }
                    } else {
                        println!("\n❌ Execution cancelled.\n");
                    }
                }
                CommandAction::Cancel => {
                    println!("\n❌ Cancelled.\n");
                }
            }
        }
        None => {
            println!("\n❌ Cancelled.\n");
        }
    }

    Ok(())
}

/// Run async operation with spinner
/// Generic wrapper for common pattern: create spinner -> run async task -> finish spinner
pub async fn with_spinner<F, T>(message: &str, future: F) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    let spinner = create_spinner(message);
    let result = future.await;
    finish_spinner(spinner, None);
    result
}
