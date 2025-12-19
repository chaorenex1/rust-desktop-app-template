//! Terminal Service module
//!
//! This module handles terminal session management.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use uuid::Uuid;

use crate::utils::error::{AppError, AppResult};

/// Terminal session
#[derive(Debug)]
pub struct TerminalSession {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Working directory
    pub cwd: String,
    /// Process handle
    process: Option<Child>,
}

impl TerminalSession {
    /// Create a new terminal session
    pub fn new(name: String, cwd: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            cwd,
            process: None,
        }
    }

    /// Kill the terminal process
    pub fn kill(&mut self) -> AppResult<()> {
        if let Some(ref mut process) = self.process {
            process.kill().map_err(|e| AppError::ProcessError(e.to_string()))?;
        }
        self.process = None;
        Ok(())
    }
}

/// Terminal Service for managing terminal sessions
#[derive(Debug)]
pub struct TerminalService {
    /// Active terminal sessions
    sessions: Arc<Mutex<HashMap<String, TerminalSession>>>,
}

impl TerminalService {
    /// Create a new terminal service
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new terminal session
    pub fn create_session(&self, name: Option<String>, cwd: Option<String>) -> AppResult<String> {
        let session_name = name.unwrap_or_else(|| format!("Terminal {}", self.session_count() + 1));
        let working_dir = cwd.unwrap_or_else(|| ".".to_string());

        let session = TerminalSession::new(session_name, working_dir);
        let session_id = session.id.clone();

        let mut sessions = self.sessions.lock().map_err(|e| {
            AppError::ProcessError(format!("Failed to lock sessions: {}", e))
        })?;

        sessions.insert(session_id.clone(), session);
        info!("Created terminal session: {}", session_id);

        Ok(session_id)
    }

    /// Kill a terminal session
    pub fn kill_session(&self, session_id: &str) -> AppResult<()> {
        let mut sessions = self.sessions.lock().map_err(|e| {
            AppError::ProcessError(format!("Failed to lock sessions: {}", e))
        })?;

        if let Some(mut session) = sessions.remove(session_id) {
            session.kill()?;
            info!("Killed terminal session: {}", session_id);
        }

        Ok(())
    }

    /// Execute command in a session
    pub fn execute_command(
        &self,
        session_id: &str,
        shell: &str,
        command_line: &str,
    ) -> AppResult<String> {
        // 先在短时间内获取会话工作目录，然后释放锁，避免长时间持有锁阻塞并行执行
        let cwd = {
            let sessions = self.sessions.lock().map_err(|e| {
                AppError::ProcessError(format!("Failed to lock sessions: {}", e))
            })?;

            let session = sessions.get(session_id).ok_or_else(|| {
                AppError::ProcessError(format!("Session not found: {}", session_id))
            })?;

            session.cwd.clone()
        };

        // 根据前端选择的 shell 校验并构造具体命令
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let shell_norm = shell.trim().to_lowercase();

            if shell_norm.starts_with("powershell") || shell_norm == "pwsh" {
                let mut c = Command::new("powershell.exe");
                c.arg("-NoLogo")
                    .arg("-NoProfile")
                    .arg("-Command")
                    .arg(command_line);
                c
            } else if shell_norm == "cmd" || shell_norm == "cmd.exe" {
                let mut c = Command::new("cmd.exe");
                c.arg("/C").arg(command_line);
                c
            } else {
                return Err(AppError::ProcessError(format!(
                    "Unsupported shell on Windows: {}",
                    shell
                )));
            }
        };

        #[cfg(not(target_os = "windows"))]
        let mut cmd = {
            let shell_norm = shell.trim().to_lowercase();

            if shell_norm == "bash" {
                let mut c = Command::new("bash");
                c.arg("-lc").arg(command_line);
                c
            } else if shell_norm == "zsh" {
                let mut c = Command::new("zsh");
                c.arg("-lc").arg(command_line);
                c
            } else if shell_norm == "sh" {
                let mut c = Command::new("sh");
                c.arg("-lc").arg(command_line);
                c
            } else {
                return Err(AppError::ProcessError(format!(
                    "Unsupported shell on Unix-like system: {}",
                    shell
                )));
            }
        };

        info!(
            "Executing terminal command in session {} with shell '{}': {}",
            session_id, shell, command_line
        );

        let output = cmd
            .current_dir(&cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| AppError::ProcessError(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            error!("Command stderr: {}", stderr);
        }

        Ok(stdout.to_string())
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        self.sessions.lock().map(|s| s.len()).unwrap_or(0)
    }

    /// Get all session IDs
    pub fn get_session_ids(&self) -> Vec<String> {
        self.sessions
            .lock()
            .map(|s| s.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}
