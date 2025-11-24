use crate::system::error::{DoumError, Result};
use crate::system::env::{SystemInfo, OsType, ShellType};
use std::process::{Command, Output};
use std::time::Duration;

/// 명령 실행 결과
#[derive(Debug)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// OS와 쉘에 맞게 명령 실행
pub fn execute(command: &str, system_info: &SystemInfo) -> Result<CommandOutput> {
    execute_with_timeout(command, system_info, None)
}

/// 타임아웃을 지정하여 명령 실행
pub fn execute_with_timeout(
    command: &str,
    system_info: &SystemInfo,
    timeout: Option<Duration>,
) -> Result<CommandOutput> {
    let output = match system_info.os {
        OsType::Windows => execute_windows(command, &system_info.shell, timeout)?,
        OsType::Linux | OsType::MacOS => execute_unix(command, &system_info.shell, timeout)?,
    };

    let exit_code = output.status.code().unwrap_or(-1);
    let success = output.status.success();

    Ok(CommandOutput {
        stdout: output.stdout,
        stderr: output.stderr,
        exit_code,
        success,
    })
}

/// Windows에서 명령 실행
fn execute_windows(command: &str, shell: &ShellType, _timeout: Option<Duration>) -> Result<Output> {
    let mut cmd = match shell {
        ShellType::PowerShell => {
            let mut c = Command::new("powershell.exe");
            c.arg("-NoProfile");
            c.arg("-Command");
            // PowerShell은 기본적으로 UTF-8 처리
            c.arg(format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; {}", command));
            c
        }
        ShellType::Cmd | _ => {
            // 기본값은 cmd.exe
            let mut c = Command::new("cmd.exe");
            c.arg("/C");
            // cmd는 chcp 65001로 UTF-8 설정
            c.arg(format!("chcp 65001 >nul && {}", command));
            c
        }
    };

    // 타임아웃 구현은 향후 개선 가능 (현재는 기본 동작)
    let output = cmd
        .output()
        .map_err(|e| DoumError::CommandExecution(format!("명령 실행 실패: {}", e)))?;

    Ok(output)
}

/// Unix 계열에서 명령 실행
fn execute_unix(command: &str, shell: &ShellType, _timeout: Option<Duration>) -> Result<Output> {
    let shell_path = match shell {
        ShellType::Bash => "/bin/bash",
        ShellType::Zsh => "/bin/zsh",
        ShellType::Fish => "/usr/bin/fish",
        ShellType::Unknown | _ => "/bin/sh", // 기본값
    };

    let mut cmd = Command::new(shell_path);
    cmd.arg("-c");
    cmd.arg(command);

    let output = cmd
        .output()
        .map_err(|e| DoumError::CommandExecution(format!("명령 실행 실패: {}", e)))?;

    Ok(output)
}

impl CommandOutput {
    /// stdout를 UTF-8 문자열로 변환
    pub fn stdout_string(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    /// stderr를 UTF-8 문자열로 변환
    pub fn stderr_string(&self) -> String {
        String::from_utf8_lossy(&self.stderr).to_string()
    }

    /// 명령 실행 결과를 사람이 읽기 쉬운 형태로 출력
    pub fn display(&self) -> String {
        let mut result = String::new();

        if !self.stdout.is_empty() {
            result.push_str(&self.stdout_string());
        }

        if !self.stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("=== stderr ===\n");
            result.push_str(&self.stderr_string());
        }

        if !self.success {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&format!("Exit code: {}", self.exit_code));
        }

        result
    }
}