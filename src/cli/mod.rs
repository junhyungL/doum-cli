// CLI 모듈

pub mod args;
pub mod commands;
pub mod ui;
pub mod tui;
pub mod menu;
pub mod config;

pub use args::{Cli, Commands, ConfigAction};
pub use commands::{handle_config_command, handle_ask_command, handle_suggest_command, handle_auto_command};
pub use ui::{confirm_execution, copy_to_clipboard};
