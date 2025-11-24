// 전체 시스템 관련 기능을 포함하는 모듈

pub mod config;
pub mod env;
pub mod error;
pub mod logging;
pub mod paths;

pub use config::{Config, LLMConfig, ProviderConfig, ContextConfig, LoggingConfig, load_config, load_default_config, save_config};
pub use env::{OsType, ShellType, SystemInfo, get_system_info, detect_os, detect_shell};
pub use error::{DoumError, Result};
pub use logging::{init_logging};
pub use paths::{get_app_dir, get_log_dir, get_config_path};
