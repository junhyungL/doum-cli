use anyhow::Result;
use clap::Parser;
use doum_cli::cli::{
    Cli, Commands, handle_ask_command, handle_auto_command, handle_config_command,
    handle_secret_command, handle_suggest_command, handle_switch_command,
};
use doum_cli::system::{init_logging, load_config, load_default_config};

#[tokio::main]
async fn main() {
    let exit_code = match run().await {
        Ok(_) => 0,
        Err(e) => {
            // Internal logging
            tracing::error!("doum-cli terminated with an error: {}", e);

            // User-facing error message
            eprintln!("\n[Error] {}\n", e);
            1
        }
    };

    std::process::exit(exit_code);
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = load_config().unwrap_or_else(|e| {
        eprintln!(
            "⚠️  Failed to load configuration: {}. Falling back to default configuration.",
            e
        );
        load_default_config().expect("Failed to load default configuration")
    });

    // Initialize logging
    if let Err(e) = init_logging(&config) {
        eprintln!(
            "⚠️  Failed to initialize logging: {}. Continuing without logging.",
            e
        );
    }

    tracing::info!("Starting doum-cli");

    let result = match cli.command {
        Some(Commands::Config { action }) => {
            tracing::info!("Running 'config' command");
            handle_config_command(action)?;
            Ok(())
        }
        Some(Commands::Secret { provider }) => {
            tracing::info!("Running 'secret' command");
            handle_secret_command(provider)?;
            Ok(())
        }
        Some(Commands::Switch { provider, model }) => {
            tracing::info!("Running 'switch' command");
            handle_switch_command(provider, model)?;
            Ok(())
        }
        Some(Commands::Ask { question }) => {
            tracing::info!("Running 'ask' command with question: {}", question);
            handle_ask_command(&question).await
        }
        Some(Commands::Suggest { request }) => {
            tracing::info!("Running 'suggest' command with request: {}", request);
            handle_suggest_command(&request).await
        }
        None => {
            if let Some(input) = cli.input {
                tracing::info!("Running 'auto' mode with input: {}", input);
                handle_auto_command(&input).await
            } else {
                // No arguments: show help and exit
                tracing::info!("doum-cli invoked without arguments. Showing help and exiting.");
                Cli::parse_from(["doum", "--help"]);
                Ok(())
            }
        }
    };

    tracing::info!("Shutting down doum-cli");
    result
}
