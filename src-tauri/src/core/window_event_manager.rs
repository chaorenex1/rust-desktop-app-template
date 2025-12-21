//! Window event manager
//!
//! Centralizes all window-related event wiring.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tauri::{App, Emitter, Manager, WindowEvent};

#[derive(Clone, serde::Serialize)]
struct LightweightModePayload {
    enabled: bool,
    reason: String,
}

use crate::utils::error::AppResult;

/// Central manager for window events.
pub struct WindowEventManager {
    is_quitting: Arc<AtomicBool>,
}

impl WindowEventManager {
    pub fn new(is_quitting: Arc<AtomicBool>) -> Self {
        Self { is_quitting }
    }

    /// Register events for the main window.
    ///
    /// - Clicking the close button hides the window to tray.
    /// - When `is_quitting` is set (e.g. tray "退出"), closing is allowed.
    pub fn register_main_window(&self, app: &App) -> AppResult<()> {
        if let Some(window) = app.get_webview_window("main") {
            let window_for_event = window.clone();
            let is_quitting_for_event = self.is_quitting.clone();

            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if !is_quitting_for_event.load(Ordering::SeqCst) {
                        api.prevent_close();
                        let _ = window_for_event.hide();

                        // When the window is hidden to tray, enter lightweight mode.
                        let _ = window_for_event.app_handle().emit(
                            "app:lightweight-mode",
                            LightweightModePayload {
                                enabled: true,
                                reason: "hidden".to_string(),
                            },
                        );
                    }
                }
            });
        }

        Ok(())
    }

    /// Register all window events.
    pub fn register(&self, app: &App) -> AppResult<()> {
        self.register_main_window(app)
    }
}
