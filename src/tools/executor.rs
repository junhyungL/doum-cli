use crate::system::env::{OsType, ShellType, SystemInfo};
use anyhow::{Context, Result};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
) -> Result<CommandOutput> {
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
    timeout: Option<Duration>,
) -> Result<Output> {
    let cmd = match shell {
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
            anyhow::bail!("Unsupported shell on Windows");
        }
    };

    run_with_timeout(cmd, timeout)
}

/// Execute command on Unix-like systems
fn execute_command_unix(
    command: &str,
    shell: &ShellType,
    timeout: Option<Duration>,
) -> Result<Output> {
    let shell_path = match shell {
        ShellType::Bash => "/bin/bash",
        ShellType::Zsh => "/bin/zsh",
        ShellType::Fish => "/usr/bin/fish",
        _ => "/bin/sh", // default to sh
    };

    let mut cmd = Command::new(shell_path);
    cmd.arg("-c");
    cmd.arg(command);

    run_with_timeout(cmd, timeout)
}

/// Run command with optional timeout
fn run_with_timeout(mut cmd: Command, timeout: Option<Duration>) -> Result<Output> {
    // setup to capture output
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().context("Failed to spawn command")?;

    match timeout {
        None => {
            // No timeout, wait normally
            child
                .wait_with_output()
                .context("Failed to wait for command")
        }
        Some(timeout) => {
            let start = Instant::now();

            loop {
                match child.try_wait() {
                    // Process finished
                    Ok(Some(_status)) => {
                        return child
                            .wait_with_output()
                            .context("Failed to wait for command output");
                    }
                    // Still running
                    Ok(None) => {
                        if start.elapsed() >= timeout {
                            // When timeout occurs, kill the process
                            let _ = child.kill();
                            let output = child
                                .wait_with_output()
                                .context("Failed to collect output after killing command")?;

                            // Return timeout error with partial output
                            anyhow::bail!(
                                "Command timed out after {:?}. Partial output:\n{}",
                                timeout,
                                String::from_utf8_lossy(&output.stdout)
                            );
                        }

                        // Delay before next poll
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => {
                        anyhow::bail!("Failed to poll command status: {}", e);
                    }
                }
            }
        }
    }
}
