//! Tauri commands for chat session management

use crate::services::chat_session::{self, ChatMessage, ChatSession};
use tracing::{debug, error};

/// Save a chat session
#[tauri::command]
pub async fn save_chat_session(
    session_id: Option<String>,
    name: Option<String>,
    codeagent_session_id: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<ChatSession, String> {
    debug!(
        "Command: save_chat_session - session_id: {:?}, name: {:?}, message_count: {}",
        session_id,
        name,
        messages.len()
    );

    match chat_session::save_session(
        session_id,
        name,
        codeagent_session_id,
        messages,
        None,
    ) {
        Ok(session) => {
            debug!("Successfully saved chat session: {}", session.id);
            Ok(session)
        }
        Err(e) => {
            error!("Failed to save chat session: {}", e);
            Err(e)
        }
    }
}

/// Load all chat sessions
#[tauri::command]
pub async fn load_chat_sessions(workspace_id: String, limit: Option<usize>) -> Result<Vec<ChatSession>, String> {
    debug!("Command: load_chat_sessions - workspace_id: {}, limit: {:?}", workspace_id, limit);

    match chat_session::load_all_sessions(workspace_id, limit) {
        Ok(sessions) => {
            debug!("Successfully loaded {} chat sessions", sessions.len());
            Ok(sessions)
        }
        Err(e) => {
            error!("Failed to load chat sessions: {}", e);
            Err(e)
        }
    }
}

/// Delete a chat session
#[tauri::command]
pub async fn delete_chat_session(session_id: String) -> Result<(), String> {
    debug!("Command: delete_chat_session - session_id: {}", session_id);

    match chat_session::delete_session(&session_id) {
        Ok(()) => {
            debug!("Successfully deleted chat session: {}", session_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete chat session: {}", e);
            Err(e)
        }
    }
}

/// Update a chat session name
#[tauri::command]
pub async fn update_chat_session_name(
    session_id: String,
    name: String,
) -> Result<ChatSession, String> {
    debug!(
        "Command: update_chat_session_name - session_id: {}, name: {}",
        session_id, name
    );

    match chat_session::update_session_name(&session_id, name) {
        Ok(session) => {
            debug!("Successfully updated chat session name: {}", session_id);
            Ok(session)
        }
        Err(e) => {
            error!("Failed to update chat session name: {}", e);
            Err(e)
        }
    }
}
