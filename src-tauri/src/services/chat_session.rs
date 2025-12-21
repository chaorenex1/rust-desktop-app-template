//! Chat session management service
//!
//! This module provides functions for managing chat sessions stored as JSON files.

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::Utc;
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
}

/// Chat session structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub name: Option<String>,
    #[serde(default)]
    pub codeagent_session_id: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: usize,
    pub first_message_preview: String,
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
    codeagent_session_id: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<ChatSession, String> {
    let dir = ensure_sessions_dir_exists()?;

    let session_id = session_id.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });
    let codeagent_session_id = codeagent_session_id.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });

    // 统一 id：优先使用 codeagent_session_id（用于 resume），否则沿用已有 id，再否则生成。
    let id = codeagent_session_id
        .clone()
        .or(session_id.clone())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let file_path = dir.join(format!("{}.json", id));

    info!("Saving chat session: {} (file: {:?})", id, file_path);

    // Build session object
    let now = Utc::now().to_rfc3339();

    // Generate first message preview
    let first_message_preview = messages
        .first()
        .map(|m| {
            let content = &m.content;
            if content.len() > 100 {
                format!("{}...", &content[..100])
            } else {
                content.clone()
            }
        })
        .unwrap_or_default();

    // Preserve created_at if updating existing session
    let (created_at, preserved_codeagent_session_id) = if file_path.exists() {
        match load_session_by_id(&id) {
            Ok(existing) => (existing.created_at, existing.codeagent_session_id),
            Err(_) => {
                warn!("Failed to load existing session, using current time as created_at");
                (now.clone(), None)
            }
        }
    } else {
        (now.clone(), None)
    };

    let message_count = messages.len();

    let session = ChatSession {
        id: id.clone(),
        name,
        // 兼容字段：保持与 id 一致，避免“双 id”
        codeagent_session_id: Some(id.clone()).or(preserved_codeagent_session_id),
        messages,
        created_at,
        updated_at: now,
        message_count,
        first_message_preview,
    };

    // Write to file
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write session file: {}", e))?;

    info!("Chat session saved successfully: {}", id);
    Ok(session)
}

/// Load all chat sessions
pub fn load_all_sessions(limit: Option<usize>) -> Result<Vec<ChatSession>, String> {
    let dir = get_sessions_dir()?;

    if !dir.exists() {
        info!("Sessions directory does not exist, returning empty list");
        return Ok(Vec::new());
    }

    info!("Loading chat sessions from: {:?}", dir);

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read sessions directory: {}", e))?;

    let mut sessions = Vec::new();
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
                Ok(mut session) => {
                    // 自动迁移：如果历史里存在 codeagent_session_id 且与 id 不同，尝试将文件重命名为 codeagent_session_id。
                    if let Some(new_id) = session.codeagent_session_id.clone() {
                        let new_id = new_id.trim().to_string();
                        if !new_id.is_empty() && new_id != session.id {
                            let new_path = dir.join(format!("{}.json", new_id));
                            if !new_path.exists() {
                                match fs::rename(&path, &new_path) {
                                    Ok(()) => {
                                        session.id = new_id.clone();
                                        session.codeagent_session_id = Some(new_id);
                                        if let Ok(json) = serde_json::to_string_pretty(&session) {
                                            let _ = fs::write(&new_path, json);
                                        }
                                        debug!("Migrated session file {:?} -> {:?}", path, new_path);
                                    }
                                    Err(e) => {
                                        warn!("Failed to migrate session file {:?} -> {:?}: {}", path, new_path, e);
                                    }
                                }
                            }
                        }
                    }

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
        info!("Loaded {} sessions (limited to {} from {})", sessions.len(), limit, result_count);
    } else {
        info!("Loaded {} sessions", sessions.len());
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

    info!("Deleting chat session: {} (file: {:?})", session_id, file_path);

    fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete session file: {}", e))?;

    info!("Chat session deleted successfully: {}", session_id);
    Ok(())
}

/// Update a chat session name
pub fn update_session_name(session_id: &str, name: String) -> Result<ChatSession, String> {
    info!("Updating session name: {} -> {}", session_id, name);

    let mut session = load_session_by_id(session_id)?;
    session.name = Some(name);
    session.updated_at = Utc::now().to_rfc3339();

    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    // Write updated session to file
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write session file: {}", e))?;

    info!("Session name updated successfully: {}", session_id);
    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_message_preview_truncation() {
        let long_content = "a".repeat(150);
        let messages = vec![ChatMessage {
            id: "test".to_string(),
            role: "user".to_string(),
            content: long_content,
            timestamp: Utc::now().to_rfc3339(),
            files: None,
            model: None,
        }];

        let preview = messages
            .first()
            .map(|m| {
                let content = &m.content;
                if content.len() > 100 {
                    format!("{}...", &content[..100])
                } else {
                    content.clone()
                }
            })
            .unwrap_or_default();

        assert_eq!(preview.len(), 103); // 100 chars + "..."
        assert!(preview.ends_with("..."));
    }
}
