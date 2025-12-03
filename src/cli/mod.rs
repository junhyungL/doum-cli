// CLI 모듈

pub mod args;
pub mod ask;
pub mod auto_mode;
pub mod config;
pub mod secret;
pub mod suggest;
pub mod switch;

pub use args::{Cli, Commands, ConfigAction};
pub use ask::handle_ask_command;
pub use auto_mode::handle_auto_command;
pub use config::handle_config_command;
pub use secret::handle_secret_command;
pub use suggest::handle_suggest_command;
pub use switch::handle_switch_command;
