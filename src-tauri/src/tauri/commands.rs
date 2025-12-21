//! Tauri commands module
//!
//! This module defines Tauri IPC commands that can be called from the frontend.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, State};
use tracing::{error, info};
use tauri::async_runtime;
use crate::core::AppState;
use crate::services::ai::{AiChatOptions, AiService};
use super::event_handlers::emit_ai_response;

/// Send chat message to AI
#[tauri::command]
pub async fn send_chat_message(
    message: String,
    context_files: Option<Vec<String>>,
) -> Result<String, String> {
    info!("Sending chat message: {}", message);

    // Use AiService as the single entry; internally it calls codeagent-wrapper.
    let ai = AiService::new();
    ai.send_message(&message, context_files)
        .await
        .map_err(|e| e.to_string())
}

/// Send chat message to AI with simulated streaming response
#[tauri::command]
pub async fn send_chat_message_streaming(
    app_handle: AppHandle,
    message: String,
    context_files: Option<Vec<String>>,
    code_cli: Option<String>,
    resume_session_id: Option<String>,
    codex_model: Option<String>,
) -> Result<String, String> {
    info!("Sending chat message (streaming): {}", message);
    info!(
        code_cli = ?code_cli,
        resume_session_id = ?resume_session_id,
        codex_model = ?codex_model,
        "Streaming chat options"
    );

    // 为本次会话生成唯一 request_id，前端用它关联流式回复
    let request_id = uuid::Uuid::new_v4().to_string();
    let request_id_for_task = request_id.clone();
    let app_handle_clone = app_handle.clone();

    // 将实际消息处理与流式发送放到后台任务中，避免阻塞当前命令
    let msg = message.clone();
    let ctx_files = context_files.clone();

    async_runtime::spawn(async move {
        let ai = AiService::new();
        match ai
            .send_message_with_options(
                &msg,
                ctx_files,
                AiChatOptions {
                    code_cli,
                    resume_session_id,
                    parallel: false,
                    codex_model,
                },
            )
            .await
        {
            Ok(result) => {
                let chars: Vec<char> = result.message.chars().collect();
                let total = chars.len();
                let mut buffer = String::new();

                for (idx, ch) in chars.into_iter().enumerate() {
                    buffer.push(ch);

                    let is_last = idx + 1 == total;
                    // 每凑够一定长度，或者到达结尾，就发送一块增量
                    if buffer.len() >= 32 || is_last {
                        let delta = buffer.clone();
                        buffer.clear();

                        let codeagent_session_id = if is_last {
                            result.codeagent_session_id.as_deref()
                        } else {
                            None
                        };

                        if let Err(e) = emit_ai_response(
                            &app_handle_clone,
                            &request_id_for_task,
                            &delta,
                            is_last,
                            codeagent_session_id,
                        ) {
                            error!("Failed to emit AI response chunk: {:?}", e);
                            break;
                        }

                        // 模拟流式延迟效果（阻塞当前后台任务线程即可）
                        std::thread::sleep(Duration::from_millis(60));
                    }
                }
            }
            Err(e) => {
                error!("Failed to build AI response for streaming: {}", e);
                let _ = emit_ai_response(
                    &app_handle_clone,
                    &request_id_for_task,
                    &format!("[AI 错误] {}", e),
                    true,
                    None,
                );
            }
        }
    });

    // 立即把 request_id 返回给前端，前端可用它在 Chat Messages Area 中关联消息
    Ok(request_id)
}

/// Execute command in terminal
#[tauri::command]
pub async fn execute_command(
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
) -> Result<String, String> {
    info!("Executing command: {} {:?}", command, args);

    async_runtime::spawn_blocking(move || {
        let mut cmd = std::process::Command::new(&command);
        cmd.args(&args);

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output = cmd.output().map_err(|e| e.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        if !stderr.is_empty() {
            error!("Command stderr: {}", stderr);
        }

        Ok::<String, String>(stdout)
    })
    .await
    .map_err(|e| format!("执行命令任务失败: {}", e))?
}

/// Execute a command in an existing terminal session
#[tauri::command]
pub async fn execute_terminal_command(
    state: State<'_, AppState>,
    sessionId: String,
    shell: String,
    command: String,
) -> Result<String, String> {
    info!(
        "Executing terminal command in session {} with shell {}: {}",
        sessionId, shell, command
    );

    state
        .terminal
        .execute_command(&sessionId, &shell, &command)
        .map_err(|e| e.to_string())
}

/// Spawn new terminal session using TerminalService
#[tauri::command]
pub async fn spawn_terminal(state: State<'_, AppState>, cwd: Option<String>) -> Result<String, String> {
    info!("Spawning new terminal");

    state
        .terminal
        .create_session(None, cwd)
        .map_err(|e| e.to_string())
}

/// Kill terminal session via TerminalService
#[tauri::command]
pub async fn kill_terminal(state: State<'_, AppState>, terminal_id: String) -> Result<(), String> {
    info!("Killing terminal: {}", terminal_id);

    state
        .terminal
        .kill_session(&terminal_id)
        .map_err(|e| e.to_string())
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    info!("Getting system information");

    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
        "total_swap": sys.total_swap(),
        "used_swap": sys.used_swap(),
        "cpu_count": sys.cpus().len(),
        "host_name": System::host_name().unwrap_or_default(),
    });

    Ok(info)
}

/// Get application logs from the configured log file
#[tauri::command]
pub async fn get_logs(state: State<'_, AppState>, limit: Option<usize>) -> Result<Vec<String>, String> {
    let date = chrono::Local::now().format("%Y-%m-%d");
    let path = {
        let cfg = state.config.lock().map_err(|e| e.to_string())?;
        let mut p = PathBuf::from(&cfg.logging.log_file_path);
        let filename = format!("{}.{}", cfg.logging.log_file_name, date);
        p.push(&filename);
        p
    };
    async_runtime::spawn_blocking(move || {
        if !path.exists() {
            return Ok(Vec::new());
        }

        use std::io::{BufRead, BufReader};

        let file = fs::File::open(&path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader
            .lines()
            .filter_map(|l| l.ok())
            .collect();

        if let Some(limit) = limit {
            if lines.len() > limit {
                lines = lines.split_off(lines.len() - limit);
            }
        }

        Ok::<Vec<String>, String>(lines)
    })
    .await
    .map_err(|e| format!("读取日志任务失败: {}", e))?
}

/// Clear application logs by truncating the log file
#[tauri::command]
pub async fn clear_logs(state: State<'_, AppState>) -> Result<(), String> {
    info!("Clearing application logs");

    let path = {
        let cfg = state.config.lock().map_err(|e| e.to_string())?;
        let mut p = PathBuf::from(&cfg.logging.log_file_path);
        p.push(&cfg.logging.log_file_name);
        p
    };

    async_runtime::spawn_blocking(move || {
        if path.exists() {
            fs::write(&path, "").map_err(|e| e.to_string())?;
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("清除日志任务失败: {}", e))?
}

/// Add a recent directory
#[tauri::command]
pub async fn add_recent_directory(
    app: AppHandle,
    path: String,
) -> Result<(), String> {
    info!("Adding recent directory: {}", path);

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::add(&db, &path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get recent directories
#[tauri::command]
pub async fn get_recent_directories(
    app: AppHandle,
) -> Result<Vec<serde_json::Value>, String> {
    info!("Getting recent directories");

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    let directories = crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::get_recent(&db)
        .await
        .map_err(|e| e.to_string())?;

    let result = directories.into_iter().map(|dir| {
        serde_json::json!({
            "path": dir.path,
            "openedAt": dir.opened_at.to_rfc3339(),
        })
    }).collect();

    Ok(result)
}

/// Clear recent directories
#[tauri::command]
pub async fn clear_recent_directories(
    app: AppHandle,
) -> Result<(), String> {
    info!("Clearing recent directories");

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::clear(&db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
