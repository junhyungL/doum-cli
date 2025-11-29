// CLI 모듈

pub mod args;
pub mod commands;
pub mod ui;

pub use args::{Cli, Commands, ConfigAction};
pub use commands::{
    handle_ask_command, handle_auto_command, handle_config_command, handle_secret_command,
    handle_suggest_command, handle_switch_command,
};
pub use ui::{CommandAction, confirm_execution, copy_to_clipboard};
