use crate::system::env::{OsType, ShellType, SystemInfo};
use crate::system::error::{DoumError, DoumResult};
use std::process::{Command, Output};
use std::time::Duration;

/// Result of command execution
#[derive(Debug)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CommandOutput {
    /// convert stdout to UTF-8 string
    pub fn stdout_string(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    /// convert stderr to UTF-8 string
    pub fn stderr_string(&self) -> String {
        String::from_utf8_lossy(&self.stderr).to_string()
    }

    /// formatted display of command output
    pub fn display(&self) -> String {
        let mut result = String::new();

        if !self.stdout.is_empty() {
            result.push_str("=== stdout ===\n");
            result.push_str(&self.stdout_string());
            result.push('\n');
        }

        if !self.stderr.is_empty() {
            result.push_str("=== stderr ===\n");
            result.push_str(&self.stderr_string());
            result.push('\n');
        }

        if !self.success {
            result.push_str("=== status ===\n");
            result.push_str(&format!("Exit code: {}", self.exit_code));
            result.push('\n');
        }

        result
    }
}

/// Execute a command based on the system information
pub fn execute_command(
    command: &str,
    system_info: &SystemInfo,
    timeout: Option<Duration>,
) -> DoumResult<CommandOutput> {
    let output = match system_info.os {
        OsType::Windows => execute_command_windows(command, &system_info.shell, timeout)?,
        OsType::Linux | OsType::MacOS => {
            execute_command_unix(command, &system_info.shell, timeout)?
        }
    };

    let success = output.status.success();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok(CommandOutput {
        success,
        exit_code,
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

/// Execute command on Windows
fn execute_command_windows(
    command: &str,
    shell: &ShellType,
    _timeout: Option<Duration>,
) -> DoumResult<Output> {
    let mut cmd = match shell {
        ShellType::PowerShell => {
            let mut c = Command::new("powershell.exe");
            c.arg("-NoProfile");
            c.arg("-Command");
            // Set output encoding to UTF-8
            c.arg(format!(
                "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; {}",
                command
            ));
            c
        }
        ShellType::Cmd => {
            // Default to cmd.exe
            let mut c = Command::new("cmd.exe");
            c.arg("/C");
            // Set code page to UTF-8 before executing command
            c.arg(format!("chcp 65001 >nul && {}", command));
            c
        }
        _ => {
            return Err(DoumError::CommandExecution(
                "Unsupported shell on Windows".to_string(),
            ));
        }
    };

    // 타임아웃 구현은 향후 개선 가능 (현재는 기본 동작)
    let output = cmd
        .output()
        .map_err(|e| DoumError::CommandExecution(format!("Failed to execute command: {}", e)))?;

    Ok(output)
}

/// Execute command on Unix-like systems
fn execute_command_unix(
    command: &str,
    shell: &ShellType,
    _timeout: Option<Duration>,
) -> DoumResult<Output> {
    let shell_path = match shell {
        ShellType::Bash => "/bin/bash",
        ShellType::Zsh => "/bin/zsh",
        ShellType::Fish => "/usr/bin/fish",
        _ => "/bin/sh", // default to sh
    };

    let mut cmd = Command::new(shell_path);
    cmd.arg("-c");
    cmd.arg(command);

    let output = cmd
        .output()
        .map_err(|e| DoumError::CommandExecution(format!("Failed to execute command: {}", e)))?;

    Ok(output)
}
