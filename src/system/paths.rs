use crate::system::error::{DoumError, DoumResult};
use std::path::PathBuf;

/// Returns the application directory path based on the operating system:
/// - Windows: C:\Users\{user}\AppData\Roaming\doum-cli
/// - macOS: ~/Library/Application Support/doum-cli
/// - Linux: ~/.config/doum-cli
pub fn get_app_dir() -> DoumResult<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").map_err(|_| {
            DoumError::Config("Could not find APPDATA environment variable".to_string())
        })?;
        Ok(PathBuf::from(appdata).join("doum-cli"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| DoumError::Config("HOME environment variable not found".to_string()))?;
        Ok(PathBuf::from(home).join("Library/Application Support/doum-cli"))
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| DoumError::Config("HOME environment variable not found".to_string()))?;
        let config_home =
            std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
        Ok(PathBuf::from(config_home).join("doum-cli"))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(DoumError::Config(
            "Unsupported operating system for determining app directory".to_string(),
        ))
    }
}

/// Returns the log directory path
pub fn get_log_dir() -> DoumResult<PathBuf> {
    Ok(get_app_dir()?.join("logs"))
}

/// Returns the configuration file path
pub fn get_config_path() -> DoumResult<PathBuf> {
    Ok(get_app_dir()?.join("config.toml"))
}
