//! Configuration schema module
//!
//! This module defines additional configuration schemas.

use serde::{Deserialize, Serialize};
use crate::config::loader::{get_default_data_dir, get_user_home};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application settings
    pub app: AppSettings,
    /// Database settings
    pub database: DatabaseSettings,
    //// Deployment settings
    pub deployment: DeploymentSettings,
    /// Logging settings
    pub logging: LoggingSettings,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Data directory
    pub data_dir: String,
    /// User Home directory
    pub user_home: String,
    /// Enable debug mode
    /// Auto update enabled
    pub auto_update: Option<bool>,
}
/// deployment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSettings {
    /// Deployment environment
    pub environment: String,
    /// DEBUG mode
    pub debug: bool,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
}

/// logging settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// Log level
    pub log_level: String,
    /// Log fmt pattern
    pub log_fmt_pattern: Option<String>,
    /// Log file path
    pub log_file_path: String,
    /// Log file name
    pub log_file_name: String,
    /// Enable console logging
    pub console: bool,
    /// file_rotation settings
    pub log_file_rotation: FileRotationSettings,
}

/// File rotation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRotationSettings {
    /// Maximum file size in MB
    pub log_file_max_size_mb: u64,
    /// Maximum number of backup files
    pub log_file_max_backups: u32,
    /// Maximum age of log files in days
    pub log_file_max_age_days: u32,
}

/// Database settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// Database URL
    pub url: String,
    /// Maximum connections
    pub max_connections: u32,
    /// Minimum connections
    pub min_connections: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = get_default_data_dir().unwrap();
        Self {
            app: AppSettings {
                name: "Code AI Assistant".to_string(),
                version: "0.1.0".to_string(),
                data_dir: data_dir.clone(),
                user_home: get_user_home().unwrap(),
                auto_update: Some(true),
            },
            database: DatabaseSettings {
                url: format!("sqlite://{}/app.db?mode=rwc", data_dir),
                max_connections: 10,
                min_connections: 1,
            },
            deployment: DeploymentSettings {
                environment: "development".to_string(),
                debug: true,
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            logging: LoggingSettings {
                log_level: "debug".to_string(),
                log_file_path: get_default_data_dir().unwrap() + "/logs",
                log_file_name: "app.log".to_string(),
                log_fmt_pattern: Some("%Y-%m-%d %H:%M:%S%.3f %l %T %n %f:%L".to_string()),
                console: true,
                log_file_rotation: FileRotationSettings {
                    log_file_max_size_mb: 10,
                    log_file_max_backups: 5,
                    log_file_max_age_days: 30,
                },
            },
        }
    }
}
