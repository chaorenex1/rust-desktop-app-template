//! System tray (tray icon) support

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Emitter, Manager,
};

#[derive(Clone, serde::Serialize)]
struct LightweightModePayload {
    enabled: bool,
    reason: String,
}

use crate::utils::error::{generic_error, AppResult};

const TRAY_MENU_SHOW: &str = "tray_show";
const TRAY_MENU_HIDE: &str = "tray_hide";
const TRAY_MENU_QUIT: &str = "tray_quit";

pub fn init_tray(app: &mut App, is_quitting: Arc<AtomicBool>) -> AppResult<()> {
    let app_handle = app.handle().clone();

    // Build tray context menu
    let show_item = MenuItem::with_id(&app_handle, TRAY_MENU_SHOW, "显示主窗口", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(&app_handle, TRAY_MENU_HIDE, "隐藏主窗口", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(&app_handle, TRAY_MENU_QUIT, "退出", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(&app_handle)?;

    let menu = Menu::with_items(&app_handle, &[&show_item, &hide_item, &separator, &quit_item])?;

    // Use the app's default icon (from tauri.conf.json bundle icons)
    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or_else(|| generic_error("找不到默认窗口图标，无法创建系统托盘图标"))?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app: &AppHandle, event: tauri::menu::MenuEvent| {
            let id = event.id().as_ref();
            if let Some(window) = app.get_webview_window("main") {
                match id {
                    TRAY_MENU_SHOW => {
                        let _ = window.show();
                        let _ = window.set_focus();

                        let _ = app.emit(
                            "app:lightweight-mode",
                            LightweightModePayload {
                                enabled: false,
                                reason: "tray_show".to_string(),
                            },
                        );
                    }
                    TRAY_MENU_HIDE => {
                        let _ = window.hide();

                        let _ = app.emit(
                            "app:lightweight-mode",
                            LightweightModePayload {
                                enabled: true,
                                reason: "tray_hide".to_string(),
                            },
                        );
                    }
                    TRAY_MENU_QUIT => {
                        // Mark quitting so CloseRequested handler allows the window to close.
                        is_quitting.store(true, Ordering::SeqCst);
                        let _ = window.close();
                    }
                    _ => {}
                }
            } else if id == TRAY_MENU_QUIT {
                is_quitting.store(true, Ordering::SeqCst);
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray: &TrayIcon, event: TrayIconEvent| {
            // Double-click to toggle main window
            if let TrayIconEvent::DoubleClick { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(true) {
                        let _ = window.hide();

                        let _ = app.emit(
                            "app:lightweight-mode",
                            LightweightModePayload {
                                enabled: true,
                                reason: "tray_double_click_hide".to_string(),
                            },
                        );
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();

                        let _ = app.emit(
                            "app:lightweight-mode",
                            LightweightModePayload {
                                enabled: false,
                                reason: "tray_double_click_show".to_string(),
                            },
                        );
                    }
                }
            }
        })
        .build(&app_handle)
        .map_err(|e| generic_error(&format!("创建系统托盘失败: {e}")))?;

    Ok(())
}
