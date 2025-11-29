// 전체 시스템 관련 기능을 포함하는 모듈

pub mod config;
pub mod env;
pub mod logging;
pub mod paths;
pub mod secret;

pub use config::{
    Config, ContextConfig, LLMConfig, LoggingConfig, load_config, load_default_config, save_config,
};
pub use env::{OsType, ShellType, SystemInfo, detect_os, detect_shell, get_system_info};
pub use logging::init_logging;
pub use paths::{get_app_dir, get_config_path, get_log_dir};
pub use secret::{ProviderSecret, SecretManager};
