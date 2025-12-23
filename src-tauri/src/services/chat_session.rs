//! Chat session management service
//!
//! This module provides functions for managing chat sessions stored as JSON files.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::Local;
use uuid::Uuid;
use tracing::{info, warn, debug, error};

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub files: Option<Vec<String>>,
    pub model: Option<String>,
    pub session_id: Option<String>,
    pub workspace_id: Option<String>,
}

/// Chat session structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub name: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    pub workspace_id: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: usize,
    pub first_message_preview: String,
    #[serde(default)]
    pub code_cli_task_ids: HashMap<String, String>,
}

/// Get the chat sessions directory path
fn get_sessions_dir() -> Result<PathBuf, String> {
    let home = crate::config::get_default_data_dir().map_err(|e| format!("Failed to get home directory: {}", e))?;
    let sessions_dir = PathBuf::from(home).join("chat-sessions");
    Ok(sessions_dir)
}

/// Ensure the sessions directory exists
fn ensure_sessions_dir_exists() -> Result<PathBuf, String> {
    let dir = get_sessions_dir()?;

    if !dir.exists() {
        info!("Creating chat sessions directory: {:?}", dir);
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create sessions directory: {}", e))?;
    }

    Ok(dir)
}

/// Load a single session by ID
fn load_session_by_id(session_id: &str) -> Result<ChatSession, String> {
    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    if !file_path.exists() {
        return Err(format!("Session not found: {}", session_id));
    }

    debug!("Loading session from: {:?}", file_path);

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read session file: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse session JSON: {}", e))
}

/// Save a chat session to file
pub fn save_session(
    session_id: Option<String>,
    name: Option<String>,
    workspace_id: Option<String>,
    messages: Vec<ChatMessage>,
    code_cli_task_ids: Option<HashMap<String, String>>,
) -> Result<ChatSession, String> {
    let dir = ensure_sessions_dir_exists()?;

    let session_id = session_id.as_deref().ok_or("Session ID is required")?;
    let file_path = dir.join(format!("{}.json", session_id));

    info!("Saving chat session: {} (file: {:?})", session_id, file_path);

    // Build session object
    let now = Local::now().to_rfc3339();

    // Generate first message preview
    let first_message_preview = messages
        .first()
        .map(|m| {
            let content = &m.content;
            let mut chars = content.chars();
            let preview: String = chars.by_ref().take(100).collect();
            if chars.next().is_some() {
                format!("{}...", preview)
            } else {
                preview
            }
        })
        .unwrap_or_default();

    // Preserve created_at if updating existing session
    let (created_at, preserved_session_id) = if file_path.exists() {
        match load_session_by_id(&session_id) {
            Ok(existing) => {
                let preserved = existing
                    .session_id
                    .clone()
                    .or_else(|| Some(session_id.to_string()));
                (existing.created_at, preserved)
            }
            Err(_) => {
                warn!("Failed to load existing session, using current time as created_at");
                (now.clone(), Some(session_id.to_string()))
            }
        }
    } else {
        (now.clone(), Some(session_id.to_string()))
    };

    let message_count = messages.len();

    let session = ChatSession {
        id: session_id.to_string(),
        name,
        session_id: preserved_session_id,
        workspace_id,
        messages,
        created_at,
        updated_at: now,
        message_count,
        first_message_preview,
        code_cli_task_ids: code_cli_task_ids.unwrap_or_default(),
    };

    // Write to file
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write session file: {}", e))?;

    info!("Chat session saved successfully: {}", session_id);
    Ok(session)
}

/// append a message to a chat session
pub fn append_message_to_session(
    session_id: &str,
    messages: Vec<ChatMessage>,
    code_cli: Option<String>,
    code_cli_task_id: Option<String>,
) -> Result<(), String> {
    info!("Appending message to session: {}", session_id);

    let mut session = match load_session_by_id(session_id) {
        Ok(existing) => existing,
        Err(err) => {
            warn!(
                "Session {} not found when appending messages ({}), creating a new one",
                session_id, err
            );
            let now = Local::now().to_rfc3339();
            ChatSession {
                id: session_id.to_string(),
                name: None,
                session_id: Some(session_id.to_string()),
                workspace_id: messages
                    .first()
                    .and_then(|msg| msg.workspace_id.clone()),
                messages: Vec::new(),
                created_at: now.clone(),
                updated_at: now,
                message_count: 0,
                first_message_preview: String::new(),
                code_cli_task_ids: HashMap::new(),
            }
        }
    };
    session.session_id = Some(session_id.to_string());
    session.messages.extend(messages);
    session.message_count = session.messages.len();
    session.updated_at = Local::now().to_rfc3339();
    if let (Some(cli), Some(task_id)) = (code_cli, code_cli_task_id) {
        session
            .code_cli_task_ids
            .insert(cli, task_id);
    }

    info!("Session updated with {} total messages", session.message_count);

    save_session(
        Some(session_id.to_string()),
        session.name,
        session.workspace_id,
        session.messages,
        Some(session.code_cli_task_ids.clone()),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Load all chat sessions
pub fn load_all_sessions(workspace_id: String, limit: Option<usize>) -> Result<Vec<ChatSession>, String> {
    let dir = get_sessions_dir()?;

    if !dir.exists() {
        info!("Sessions directory does not exist, returning empty list");
        return Ok(Vec::new());
    }

    info!("Loading chat sessions from: {:?}", dir);

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read sessions directory: {}", e))?;

    let mut sessions: Vec<ChatSession> = Vec::new();
    let mut error_count = 0;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("Failed to read directory entry: {}", e);
                error_count += 1;
                continue;
            }
        };

        let path = entry.path();

        // Only process .json files
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<ChatSession>(&content) {
                Ok(session) => {
                    if session.workspace_id.as_deref() != Some(&workspace_id) {
                        continue;
                    }
                    // 自动迁移已禁用：session_id 字段已标准化，不再需要 codeagent_session_id 迁移
                    // 注：此注释保留以说明数据模型已演变

                    debug!("Loaded session: {} from {:?}", session.id, path);
                    sessions.push(session);
                }
                Err(e) => {
                    error!("Failed to parse session file {:?}: {}", path, e);
                    error_count += 1;
                }
            },
            Err(e) => {
                error!("Failed to read session file {:?}: {}", path, e);
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        warn!("Encountered {} errors while loading sessions", error_count);
    }

    // Sort by updated_at in descending order (newest first)
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    // Apply limit if specified
    let result_count = sessions.len();
    if let Some(limit) = limit {
        sessions.truncate(limit);
        debug!("Loaded {} sessions (limited to {} from {})", sessions.len(), limit, result_count);
    } else {
        debug!("Loaded {} sessions", sessions.len());
    }

    Ok(sessions)
}

/// Delete a chat session
pub fn delete_session(session_id: &str) -> Result<(), String> {
    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    if !file_path.exists() {
        return Err(format!("Session not found: {}", session_id));
    }

    debug!("Deleting chat session: {} (file: {:?})", session_id, file_path);

    fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete session file: {}", e))?;

    debug!("Chat session deleted successfully: {}", session_id);
    Ok(())
}

/// Update a chat session name
pub fn update_session_name(session_id: &str, name: String) -> Result<ChatSession, String> {
    debug!("Updating session name: {} -> {}", session_id, name);

    let mut session = load_session_by_id(session_id)?;
    session.name = Some(name);
    session.updated_at = Local::now().to_rfc3339();

    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    // Write updated session to file
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write session file: {}", e))?;

    debug!("Session name updated successfully: {}", session_id);
    Ok(session)
}
