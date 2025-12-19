use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle};
use tracing::{error, info, debug};
use tauri::async_runtime;

use crate::config::AppConfig;
use crate::core::AppState;

/// Get application settings
#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<Option<serde_json::Value>, String> {
    debug!("Getting application settings");
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    let setting = crate::database::repositories::settings_repository::SettingsRepository::get_by_key(&db, "user_config")
        .await
        .map_err(|e| e.to_string())?;

    Ok(setting.map(|s| {
        serde_json::from_str(&s.value)
            .unwrap_or(serde_json::Value::String(s.value))
    }))
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    settings: String,
) -> Result<(), String> {
    debug!("Saving application settings");

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    crate::database::repositories::settings_repository::SettingsRepository::upsert(
        &db,
        "user_config",
        &settings,
        "user_config",
        Some("User configuration settings"),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Reset settings to defaults
#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<AppConfig, String> {
    info!("Resetting settings to defaults");

    let default_config = AppConfig::default();
    let config_clone = default_config.clone();
    
    async_runtime::spawn_blocking(move || {
        crate::config::save_config(&config_clone).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("重置设置任务失败: {}", e))??;

    // Update state after async operation
    {
        let mut state_config = state.config.lock().map_err(|e| e.to_string())?;
        *state_config = default_config.clone();
    }

    Ok(default_config)
}


    /// Get a single setting by key
#[tauri::command]
pub async fn get_setting(app: AppHandle, key: String) -> Result<Option<serde_json::Value>, String> {
    info!("Getting setting: {}", key);

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    let setting = crate::database::repositories::settings_repository::SettingsRepository::get_by_key(&db, &key)
        .await
        .map_err(|e| e.to_string())?;

    Ok(setting.map(|s| {
        serde_json::from_str(&s.value)
            .unwrap_or(serde_json::Value::String(s.value))
    }))
}

/// Save a single setting
#[tauri::command]
pub async fn save_setting(
    app: AppHandle,
    key: String,
    value: serde_json::Value,
    category: Option<String>,
) -> Result<(), String> {
    info!("Saving setting: {}", key);

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    let value_str = serde_json::to_string(&value).map_err(|e| e.to_string())?;

    // Determine category from key prefix if not provided
    let cat = category.unwrap_or_else(|| {
        if key.starts_with("app.") {
            "app"
        } else if key.starts_with("user.") {
            "user"
        } else if key.starts_with("workspace.") {
            "workspace"
        } else if key.starts_with("ai.") {
            "ai"
        } else {
            "general"
        }.to_string()
    });

    crate::database::repositories::settings_repository::SettingsRepository::upsert(
        &db,
        &key,
        &value_str,
        &cat,
        None,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get settings by category
#[tauri::command]
pub async fn get_settings_by_category(
    app: AppHandle,
    category: String,
) -> Result<serde_json::Value, String> {
    info!("Getting settings for category: {}", category);

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;

    let settings = crate::database::repositories::settings_repository::SettingsRepository::get_by_category(&db, &category)
        .await
        .map_err(|e| e.to_string())?;

    let mut settings_map = serde_json::Map::new();
    for setting in settings {
        let value: serde_json::Value = serde_json::from_str(&setting.value)
            .unwrap_or(serde_json::Value::String(setting.value.clone()));
        settings_map.insert(setting.key, value);
    }

    Ok(serde_json::Value::Object(settings_map))
}