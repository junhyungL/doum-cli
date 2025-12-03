use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "doum-cli")]
#[command(author = "junhyungL")]
#[command(version = "0.3.0")]
#[command(about = "AI-powered terminal assistant - Ask about OS commands", long_about = None)]
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
    /// Secret management (API keys, tokens)
    Secret {
        /// Provider name (openai, anthropic)
        provider: Option<String>,
    },
    /// Switch provider and model
    Switch {
        /// Provider name (optional)
        provider: Option<String>,
        /// Model name (optional)
        model: Option<String>,
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
    /// Show all configuration
    Show,
    /// Reset all configuration to default
    Reset,
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., llm.timeout, context.max_lines)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., llm.provider, llm.model)
        key: String,
    },
    /// Unset (reset to default) a configuration value
    Unset {
        /// Configuration key
        key: String,
    },
}
