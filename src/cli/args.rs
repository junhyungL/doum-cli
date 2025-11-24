use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "doum")]
#[command(author = "doum-cli contributors")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered terminal assistant - Convert natural language to OS commands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Auto mode: Automatically select mode based on input
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
    /// Ask questions (Ask mode)
    Ask {
        /// Question to ask
        question: String,
    },
    /// Suggest and execute commands (Suggest mode)
    Suggest {
        /// Request description
        request: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Interactive configuration mode
    Interactive,
    /// Show all configuration
    Show,
    /// Reset all configuration to default
    Reset,
}
