
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle};
use tracing::{error, info, debug};
use tauri::async_runtime;
use tracing_subscriber::field::debug;

use crate::config::AppConfig;
use crate::core::AppState;
use super::event_handlers::emit_ai_response;

/// File entry for directory listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: Option<String>,
}

/// Read file content
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    info!("Reading file: {}", path);
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// Read file content
#[tauri::command]
pub async fn read_max_file(path: String) -> Result<String, String> {
    info!("Reading file: {}", path);
    // 先检查元数据，避免将目录或超大文件直接读入内存导致应用卡死
    let metadata = fs::metadata(&path).map_err(|e| {
        error!("Failed to stat file {}: {:?}", path, e);
        e.to_string()
    })?;

    if metadata.is_dir() {
        return Err("指定路径是目录，无法作为文件读取".to_string());
    }

    if metadata.len() <= 1024 * 1024 * 1 {
        return Ok(read_file(path.clone()).await?);
    }

    // 在阻塞线程池中读取文件，避免阻塞异步运行时
    let read_path = path.clone();
    let bytes = async_runtime::spawn_blocking(move || fs::read(&read_path))
        .await
        .map_err(|e| {
            let msg = format!("Failed to join blocking read task for {}: {:?}", path, e);
            error!("{}", msg);
            msg
        })?
        .map_err(|e| {
            // 额外输出错误日志以便调试
            error!("Failed to read file {}: {:?}", path, e);
            e.to_string()
        })?;

    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(content)
}


/// Write file content
#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    info!("Writing file: {}", path);
    
    async_runtime::spawn_blocking(move || {
        fs::write(&path, content).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("写入文件任务失败: {}", e))?
}

/// List files in directory
#[tauri::command]
pub async fn list_files(path: String) -> Result<Vec<FileEntry>, String> {
    info!("Listing files in: {}", path);

    async_runtime::spawn_blocking(move || {
        let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let path_buf = entry.path();

            files.push(FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path_buf.to_string_lossy().to_string(),
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: metadata.modified().ok().map(|t| {
                    let datetime: chrono::DateTime<chrono::Utc> = t.into();
                    datetime.to_rfc3339()
                }),
            });
        }

        // Sort: directories first, then by name
        files.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok::<Vec<FileEntry>, String>(files)
    })
    .await
    .map_err(|e| format!("列出文件任务失败: {}", e))?
}

/// Create file
#[tauri::command]
pub async fn create_file(path: String) -> Result<(), String> {
    info!("Creating file: {}", path);
    
    async_runtime::spawn_blocking(move || {
        fs::File::create(&path).map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("创建文件任务失败: {}", e))?
}

/// Delete file
#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    info!("Deleting file: {}", path);
    
    async_runtime::spawn_blocking(move || {
        let path_ref = Path::new(&path);
        if path_ref.is_dir() {
            fs::remove_dir_all(path_ref).map_err(|e| e.to_string())
        } else {
            fs::remove_file(path_ref).map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|e| format!("删除文件任务失败: {}", e))?
}

/// Rename file
#[tauri::command]
pub async fn rename_file(old_path: String, new_path: String) -> Result<(), String> {
    info!("Renaming file: {} -> {}", old_path, new_path);
    
    async_runtime::spawn_blocking(move || {
        fs::rename(&old_path, &new_path).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("重命名文件任务失败: {}", e))?
}

/// Create directory
#[tauri::command]
pub async fn create_directory(path: String) -> Result<(), String> {
    info!("Creating directory: {}", path);
    
    async_runtime::spawn_blocking(move || {
        fs::create_dir_all(&path).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("创建目录任务失败: {}", e))?
}

/// List directories
#[tauri::command]
pub async fn list_directories(path: String) -> Result<Vec<String>, String> {
    info!("Listing directories in: {}", path);

    async_runtime::spawn_blocking(move || {
        let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
        let mut dirs = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                dirs.push(entry.file_name().to_string_lossy().to_string());
            }
        }

        Ok::<Vec<String>, String>(dirs)
    })
    .await
    .map_err(|e| format!("列出目录任务失败: {}", e))?
}

/// Delete directory
#[tauri::command]
pub async fn delete_directory(path: String) -> Result<(), String> {
    info!("Deleting directory: {}", path);
    
    async_runtime::spawn_blocking(move || {
        fs::remove_dir_all(&path).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("删除目录任务失败: {}", e))?
}