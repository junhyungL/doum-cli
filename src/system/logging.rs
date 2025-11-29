use crate::system::config::Config;
use crate::system::paths::get_log_dir;
use anyhow::{Context, Result};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logging system
pub fn init_logging(config: &Config) -> Result<()> {
    if !config.logging.enabled {
        return Ok(());
    }

    // Set log level
    let level = match config.logging.level.as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => "info",
    };

    let filter = EnvFilter::try_new(format!("doum_cli={}", level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Get log directory path
    let log_dir = get_log_dir()?;

    // Create log directory if it doesn't exist
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).context("Failed to create log directory")?;
    }

    // Set up rolling file appender (daily rotation)
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .build(&log_dir)
        .context("Failed to create log file appender")?;

    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_target(false),
        )
        .try_init()
        .context("Failed to initialize logging subscriber")?;

    tracing::info!("Configured logging with level: {}", level);
    Ok(())
}
