use tauri::State;

use crate::core::notification_manager::NotificationManager;

/// Show a system notification.
///
/// If `title` is None, uses the app package name.
#[tauri::command]
pub fn show_system_notification(
    manager: State<'_, NotificationManager>,
    title: Option<String>,
    body: String,
) -> Result<(), String> {
    manager
        .notify(title.as_deref(), &body)
        .map_err(|e| e.to_string())
}
