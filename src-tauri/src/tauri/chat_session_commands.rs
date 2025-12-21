//! Tauri commands for chat session management

use crate::services::chat_session::{self, ChatMessage, ChatSession};
use tracing::{info, error};

/// Save a chat session
#[tauri::command]
pub async fn save_chat_session(
    session_id: Option<String>,
    name: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<ChatSession, String> {
    info!(
        "Command: save_chat_session - session_id: {:?}, name: {:?}, message_count: {}",
        session_id,
        name,
        messages.len()
    );

    match chat_session::save_session(session_id, name, messages) {
        Ok(session) => {
            info!("Successfully saved chat session: {}", session.id);
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
pub async fn load_chat_sessions(limit: Option<usize>) -> Result<Vec<ChatSession>, String> {
    info!("Command: load_chat_sessions - limit: {:?}", limit);

    match chat_session::load_all_sessions(limit) {
        Ok(sessions) => {
            info!("Successfully loaded {} chat sessions", sessions.len());
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
    info!("Command: delete_chat_session - session_id: {}", session_id);

    match chat_session::delete_session(&session_id) {
        Ok(()) => {
            info!("Successfully deleted chat session: {}", session_id);
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
    info!(
        "Command: update_chat_session_name - session_id: {}, name: {}",
        session_id, name
    );

    match chat_session::update_session_name(&session_id, name) {
        Ok(session) => {
            info!("Successfully updated chat session name: {}", session_id);
            Ok(session)
        }
        Err(e) => {
            error!("Failed to update chat session name: {}", e);
            Err(e)
        }
    }
}
