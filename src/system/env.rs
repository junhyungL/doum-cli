use std::env;
use std::path::PathBuf;

/// Supported operating system
#[derive(Debug, Clone, PartialEq)]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
}

impl OsType {
    pub fn as_str(&self) -> &str {
        match self {
            OsType::Windows => "Windows",
            OsType::Linux => "Linux",
            OsType::MacOS => "macOS",
        }
    }
}

/// Supported shell types
#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    Cmd,
    PowerShell,
    Bash,
    Zsh,
    Fish,
    Unknown,
}

impl ShellType {
    pub fn as_str(&self) -> &str {
        match self {
            ShellType::Cmd => "cmd.exe",
            ShellType::PowerShell => "PowerShell",
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
            ShellType::Unknown => "unknown",
        }
    }
}

/// System information structure
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: OsType,
    pub shell: ShellType,
    pub current_dir: PathBuf,
    pub username: Option<String>,
    pub hostname: Option<String>,
}

impl SystemInfo {
    pub fn display(&self) -> String {
        format!(
            "OS: {}\nShell: {}\nCurrent Dir: {}\nUsername: {}\nHostname: {}",
            self.os.as_str(),
            self.shell.as_str(),
            self.current_dir.display(),
            self.username.as_deref().unwrap_or("(unknown)"),
            self.hostname.as_deref().unwrap_or("(unknown)")
        )
    }
}

/// Get current system information
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: detect_os(),
        shell: detect_shell(),
        current_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        username: env::var("USERNAME").or_else(|_| env::var("USER")).ok(),
        hostname: env::var("COMPUTERNAME")
            .or_else(|_| env::var("HOSTNAME"))
            .ok(),
    }
}

/// Detect operating system
pub fn detect_os() -> OsType {
    #[cfg(target_os = "windows")]
    return OsType::Windows;

    #[cfg(target_os = "linux")]
    return OsType::Linux;

    #[cfg(target_os = "macos")]
    return OsType::MacOS;

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return OsType::Linux; // 기본값
}

/// Detect shell type
pub fn detect_shell() -> ShellType {
    if cfg!(target_os = "windows") {
        // Check COMSPEC on Windows
        if let Ok(comspec) = env::var("COMSPEC")
            && comspec.to_lowercase().contains("cmd.exe")
        {
            return ShellType::Cmd;
        }

        // if PSModulePath exists, it's likely PowerShell
        if env::var("PSModulePath").is_ok() {
            return ShellType::PowerShell;
        }

        // Windows Default
        return ShellType::Cmd;
    }

    // Check SHELL variable on Unix-like systems
    if let Ok(shell) = env::var("SHELL") {
        let shell_lower = shell.to_lowercase();

        if shell_lower.contains("bash") {
            return ShellType::Bash;
        } else if shell_lower.contains("zsh") {
            return ShellType::Zsh;
        } else if shell_lower.contains("fish") {
            return ShellType::Fish;
        }
    }

    // 부모 프로세스 이름으로 추가 확인 시도 (간단한 방법)
    // 실제로는 더 정교한 감지가 필요할 수 있음

    ShellType::Unknown
}
