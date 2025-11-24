use crate::system::error::{DoumError, Result};
use std::path::PathBuf;

/// doum-cli의 기본 디렉터리 경로를 반환
/// - Windows: C:\Users\{user}\AppData\Roaming\doum-cli
/// - macOS: ~/Library/Application Support/doum-cli
/// - Linux: ~/.config/doum-cli
pub fn get_app_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .map_err(|_| DoumError::Config("APPDATA 환경 변수를 찾을 수 없습니다".to_string()))?;
        Ok(PathBuf::from(appdata).join("doum-cli"))
    }
    
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| DoumError::Config("HOME 환경 변수를 찾을 수 없습니다".to_string()))?;
        Ok(PathBuf::from(home).join("Library/Application Support/doum-cli"))
    }
    
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| DoumError::Config("HOME 환경 변수를 찾을 수 없습니다".to_string()))?;
        let config_home = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        Ok(PathBuf::from(config_home).join("doum-cli"))
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(DoumError::Config("지원하지 않는 운영체제입니다".to_string()))
    }
}

/// 로그 디렉터리 경로를 반환
pub fn get_log_dir() -> Result<PathBuf> {
    Ok(get_app_dir()?.join("logs"))
}

/// 설정 파일 경로를 반환
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_app_dir()?.join("config.toml"))
}
