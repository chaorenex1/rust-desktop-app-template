//! Configuration loader module
//!
//! This module handles loading and managing application configuration.
use std::path::PathBuf;
use crate::utils::error::{AppError, AppResult};
use crate::config::schema::AppConfig;

/// Get user home directory across multiple operating systems
pub fn get_user_home() -> AppResult<String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| AppError::ConfigError("Failed to get user home directory".to_string()))?;
    Ok(home_dir.to_string_lossy().to_string())
}

pub fn get_default_data_dir() -> AppResult<String> {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| AppError::ConfigError("Failed to get config directory".to_string()))?
        .join("code-ai-assistant");
    Ok(config_dir.to_string_lossy().to_string())
}

use config::{Config, Environment, File as ConfigFile};

pub fn load_settings() -> AppResult<AppConfig> {
    // Step 1: 构建默认 Map（从 AppConfig::default() 序列化键值对）
    let defaults =  serde_json::to_string(&AppConfig::default())?;
    // Step 2: 构建 Config 对象，添加多个配置源
    let cfg: Config = Config::builder()
        .add_source(ConfigFile::from_str(&defaults, config::FileFormat::Json))
        .add_source(ConfigFile::with_name("config.toml").required(false))
        .add_source(Environment::with_prefix(".env"))
        .build()?;    // Step 2: 反序列化为 AppConfig（自动合并覆盖）
    let mut config: AppConfig = cfg.try_deserialize()
        .map_err(|e| AppError::ConfigError(format!("Failed to deserialize config: {}", e)))?;
    // Step 3: 环境变量
    config.env_vars = load_env_from_file().unwrap_or_default();
    // Step 4: 返回最终配置
    Ok(config)
}


/// Load application configuration
pub fn load_config() -> AppResult<AppConfig> {
    let config = load_settings()?;
    Ok(config)
}

/// Load configuration from file
fn load_config_from_file() -> AppResult<AppConfig> {
    let config_dir = PathBuf::from(get_default_data_dir()?);
    let config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        return Err(AppError::ConfigError("Configuration file not found".to_string()));
    }

    let config_str = std::fs::read_to_string(&config_file)
        .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;

    let config: AppConfig = toml::from_str(&config_str)
        .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &AppConfig) -> AppResult<()> {
    let config_dir = PathBuf::from(get_default_data_dir()?);
    // Create config directory if it doesn't exist
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| AppError::ConfigError(format!("Failed to create config directory: {}", e)))?;

    let config_file = config_dir.join("config.toml");
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;

    std::fs::write(&config_file, config_str)
        .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;

    tracing::info!("Configuration saved successfully to: {:?}", config_file);
    Ok(())
}

/// Load Environment Variables from file
pub fn load_env_from_file() -> AppResult<Vec<(String, String)>> {
    let config_dir = PathBuf::from(get_default_data_dir()?);

    let env_file = config_dir.join(".env");

    if !env_file.exists() {
        return Err(AppError::ConfigError("Environment file not found".to_string()));
    }

    let env_str = std::fs::read_to_string(&env_file)
        .map_err(|e| AppError::ConfigError(format!("Failed to read environment file: {}", e)))?;

    /// 转成 key=value 格式
    let env_vars: Vec<(String, String)> = env_str.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut parts = line.split('=');
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim().to_string();
            (key, value)
        })
        .collect();

    tracing::info!("Environment variables loaded successfully from: {:?}", env_file);
    Ok(env_vars)
}
