//! Code AI Assistant - Main entry point
//!
//! This is a desktop application with code editor, AI chat assistant,
//! CLI output and terminal functionality.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tracing::info;

mod config;
mod core;
mod database;
mod migration;
mod services;
#[path = "tauri/mod.rs"]
mod tauri_module;
mod utils;

/// Main entry point for the application
fn main() {
    tauri::Builder::default()
        // Register Tauri plugins
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // Focus the existing window (don't log here as tracing not yet initialized)
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())

        // Register Tauri commands
        .invoke_handler(tauri::generate_handler![
            tauri_module::fs_command::read_file,
            tauri_module::fs_command::read_max_file,
            tauri_module::fs_command::write_file,
            tauri_module::fs_command::list_files,
            tauri_module::fs_command::create_file,
            tauri_module::fs_command::delete_file,
            tauri_module::fs_command::rename_file,
            tauri_module::fs_command::create_directory,
            tauri_module::fs_command::list_directories,
            tauri_module::fs_command::delete_directory,
            tauri_module::commands::send_chat_message,
            tauri_module::commands::send_chat_message_streaming,
            tauri_module::commands::execute_command,
            tauri_module::commands::execute_terminal_command,
            tauri_module::commands::spawn_terminal,
            tauri_module::commands::kill_terminal,
            tauri_module::settings_commands::get_settings,
            tauri_module::settings_commands::save_settings,
            tauri_module::settings_commands::reset_settings,
            tauri_module::settings_commands::get_setting,
            tauri_module::settings_commands::save_setting,
            tauri_module::settings_commands::get_settings_by_category,
            tauri_module::commands::add_recent_directory,
            tauri_module::commands::get_recent_directories,
            tauri_module::commands::clear_recent_directories,
            tauri_module::workspace_command::get_workspaces,
            tauri_module::workspace_command::get_workspace,
            tauri_module::workspace_command::get_current_workspace,
            tauri_module::workspace_command::create_workspace,
            tauri_module::workspace_command::switch_workspace,
            tauri_module::workspace_command::delete_workspace,
            tauri_module::commands::get_system_info,
            tauri_module::commands::get_logs,
            tauri_module::commands::clear_logs,
            tauri_module::chat_session_commands::save_chat_session,
            tauri_module::chat_session_commands::load_chat_sessions,
            tauri_module::chat_session_commands::delete_chat_session,
            tauri_module::chat_session_commands::update_chat_session_name,
        ])

        // Setup application state
        .setup(|app| {
            // Initialize application core first (loads config)
            core::app::init(app)?;
            
            // Initialize logging (requires config to be loaded)
            utils::logging::init_tracing(app)?;
            
            // Initialize database connection
            database::connection::init(app)?;

            // Register event handlers
            tauri_module::event_handlers::register_event_handlers(app)?;

            info!("Application setup completed successfully");
            Ok(())
        })

        // Run the application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}