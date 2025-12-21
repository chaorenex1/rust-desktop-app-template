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
            // To avoid a reference cycle, capture only the AppHandle and window label
            // instead of cloning the window. This way, the window doesn't hold a
            // reference to itself through the event handler closure.
            let app_handle = window.app_handle().clone();
            let window_label = window.label().to_string();
            let is_quitting_for_event = self.is_quitting.clone();

            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if !is_quitting_for_event.load(Ordering::SeqCst) {
                        api.prevent_close();
                        
                        // Retrieve the window from AppHandle when needed
                        if let Some(window) = app_handle.get_webview_window(&window_label) {
                            let _ = window.hide();
                        }

                        // When the window is hidden to tray, enter lightweight mode.
                        let _ = app_handle.emit(
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
