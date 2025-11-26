// CLI 모듈

pub mod args;
pub mod commands;
pub mod config;
pub mod secret;
pub mod switch;
pub mod ui;

pub use args::{Cli, Commands, ConfigAction};
pub use commands::{
    handle_ask_command, handle_auto_command, handle_config_command, handle_suggest_command,
};
pub use secret::handle_secret_command;
pub use switch::handle_switch_command;
pub use ui::{confirm_execution, copy_to_clipboard};
