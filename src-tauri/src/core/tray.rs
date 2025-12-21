//! System tray (tray icon) support

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
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

fn attach_close_to_tray(window: &WebviewWindow, is_quitting: Arc<AtomicBool>) {
    let window_for_event = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            if !is_quitting.load(Ordering::SeqCst) {
                api.prevent_close();
                let _ = window_for_event.hide();

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

fn get_or_create_main_window(app: &AppHandle, is_quitting: Arc<AtomicBool>) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window("main") {
        return Some(window);
    }

    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Code AI Assistant")
        .build()
        .ok()?;

    attach_close_to_tray(&window, is_quitting);
    Some(window)
}

pub fn init_tray(app: &mut App, is_quitting: Arc<AtomicBool>) -> AppResult<()> {
    let app_handle = app.handle().clone();
    let is_quitting_for_tray_icon = is_quitting.clone();

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
            match id {
                TRAY_MENU_SHOW => {
                    if let Some(window) = get_or_create_main_window(app, is_quitting.clone()) {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }

                    let _ = app.emit(
                        "app:lightweight-mode",
                        LightweightModePayload {
                            enabled: false,
                            reason: "tray_show".to_string(),
                        },
                    );
                }
                TRAY_MENU_HIDE => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                    }

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
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.close();
                    } else {
                        app.exit(0);
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(move |tray: &TrayIcon, event: TrayIconEvent| {
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
                } else {
                    // No window: rebuild it then show
                    if let Some(window) =
                        get_or_create_main_window(app, is_quitting_for_tray_icon.clone())
                    {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }

                    let _ = app.emit(
                        "app:lightweight-mode",
                        LightweightModePayload {
                            enabled: false,
                            reason: "tray_double_click_rebuild".to_string(),
                        },
                    );
                }
            }
        })
        .build(&app_handle)
        .map_err(|e| generic_error(&format!("创建系统托盘失败: {e}")))?;

    Ok(())
}
