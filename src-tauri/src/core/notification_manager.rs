//! System notification manager
//!
//! Centralizes system notification creation and delivery.

use tauri::{App, AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::utils::error::{generic_error, AppResult};

#[derive(Clone)]
pub struct NotificationManager {
    app_handle: AppHandle,
    default_title: String,
}

impl NotificationManager {
    pub fn new(app_handle: AppHandle) -> Self {
        let default_title = app_handle.package_info().name.clone();
        Self {
            app_handle,
            default_title,
        }
    }

    pub fn notify(&self, title: Option<&str>, body: &str) -> AppResult<()> {
        let title = title.unwrap_or(self.default_title.as_str());

        self.app_handle
            .notification()
            .builder()
            .title(title)
            .body(body)
            .show()
            .map_err(|e| generic_error(&format!("发送系统通知失败: {e}")))?;

        Ok(())
    }
}

/// Initialize and register the notification manager into Tauri state.
pub fn init(app: &mut App) -> AppResult<()> {
    let manager = NotificationManager::new(app.handle().clone());
    app.manage(manager);
    Ok(())
}
