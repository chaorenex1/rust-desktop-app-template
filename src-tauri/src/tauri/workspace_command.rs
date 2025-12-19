
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tracing::{info, debug};
use tauri::async_runtime;
use serde::{Deserialize, Serialize};
use serde_json;
use anyhow;

use crate::core::AppState;
use crate::config::AppConfig;
use crate::utils::error::AppResult;

/// Workspace information returned to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Get workspace by ID
#[tauri::command]
pub async fn get_workspace(app: AppHandle, workspace_id: String) -> AppResult<WorkspaceInfo> {
    debug!("Getting workspace with id: {}", workspace_id);
    let db = crate::database::connection::get_db_connection(&app)
        .await?;
    
    let workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::get_by_id(&db, &workspace_id.parse::<i32>().unwrap())
        .await?
        .ok_or_else(|| anyhow::anyhow!("Workspace not found"))?;
    
    let workspace_info: WorkspaceInfo = WorkspaceInfo { 
        id: workspace.id.to_string(), 
        name: workspace.name, 
        path: workspace.path, 
        is_active: workspace.is_active, 
        created_at: workspace.created_at.to_rfc3339(), 
        updated_at: workspace.updated_at.to_rfc3339()
    };
    
    Ok(workspace_info)
}

/// get current workspace
#[tauri::command]
pub async fn get_current_workspace(app: AppHandle) -> AppResult<WorkspaceInfo> {
    debug!("Getting current workspace");
    let db = crate::database::connection::get_db_connection(&app)
        .await?;
    
    let workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::get_active(&db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;
    
    let workspace_info = WorkspaceInfo { 
        id: workspace.id.to_string(), 
        name: workspace.name, 
        path: workspace.path, 
        is_active: workspace.is_active, 
        created_at: workspace.created_at.to_rfc3339(), 
        updated_at: workspace.updated_at.to_rfc3339()
    };
    
    Ok(workspace_info)
}

/// Get workspaces
#[tauri::command]
pub async fn get_workspaces(app: AppHandle) -> AppResult<Vec<WorkspaceInfo>> {
    debug!("Getting workspaces");
    let db = crate::database::connection::get_db_connection(&app)
        .await?;
    
    let workspaces = crate::database::repositories::workspace_repository::WorkspaceRepository::get_all(&db)
        .await?;
    
    let workspace_infos: Vec<WorkspaceInfo> = workspaces.into_iter().map(|w| WorkspaceInfo {
        id: w.id.to_string(),
        name: w.name,
        path: w.path,
        is_active: w.is_active,
        created_at: w.created_at.to_rfc3339(),
        updated_at: w.updated_at.to_rfc3339(),
    }).collect();
    
    Ok(workspace_infos)
}

/// Create workspace and persist to workspaces.json
#[tauri::command]
pub async fn create_workspace(app: AppHandle, name: String, path: String, is_active: bool) -> AppResult<WorkspaceInfo> {
    debug!("Creating workspace: {}", &name);

    let db = crate::database::connection::get_db_connection(&app)
        .await?;

    let workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::upsert(&db, &name, &path, is_active)
        .await?;

    let workspace_info = WorkspaceInfo { 
        id: workspace.id.to_string(), 
        name: workspace.name, 
        path: workspace.path, 
        is_active: workspace.is_active, 
        created_at: workspace.created_at.to_rfc3339(), 
        updated_at: workspace.updated_at.to_rfc3339()
    };

    // Note: The file-based workspace storage seems to be legacy code, but we'll keep it for now
    // In a real application, we should probably remove this duplication
    Ok(workspace_info)
}

/// Switch workspace: only update default in config for now
#[tauri::command]
pub async fn switch_workspace(app: AppHandle, workspace_id: String) -> AppResult<WorkspaceInfo> {
    debug!("Switching to workspace: {}", &workspace_id);
    let db = crate::database::connection::get_db_connection(&app)
        .await?;
        
    let new_workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::get_by_id(&db, &workspace_id.parse::<i32>().unwrap())
        .await?
        .ok_or_else(|| anyhow::anyhow!("Workspace not found"))?;
    
    // Set the new workspace as active
    let updated_workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::upsert(&db, &new_workspace.name, &new_workspace.path, true)
        .await?;

    let new_workspace_info = WorkspaceInfo { 
        id: updated_workspace.id.to_string(), 
        name: updated_workspace.name, 
        path: updated_workspace.path, 
        is_active: updated_workspace.is_active, 
        created_at: updated_workspace.created_at.to_rfc3339(), 
        updated_at: updated_workspace.updated_at.to_rfc3339()
    };

    Ok(new_workspace_info)
}

fn workspaces_file_path(config: &AppConfig) -> PathBuf {
    let mut path = PathBuf::from(&config.app.data_dir);
    path.push("workspaces.json");
    path
}

fn load_workspaces(config: &AppConfig, _workspaces: &[WorkspaceInfo]) -> Result<Vec<WorkspaceInfo>, String> {
    let path = workspaces_file_path(config);

    if !path.exists() {
        // Create an empty workspaces file if it doesn't exist
        save_workspaces(config, &[])?;
        return Ok(vec![]);
    }

    let data = fs::read(&path).map_err(|e| e.to_string())?;
    if data.is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_slice(&data).map_err(|e| e.to_string())
}

fn save_workspaces(config: &AppConfig, workspaces: &[WorkspaceInfo]) -> Result<(), String> {
    let path = workspaces_file_path(config);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let data = serde_json::to_vec_pretty(workspaces).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

/// Delete workspace from workspaces.json (does not delete files on disk)
#[tauri::command]
pub async fn delete_workspace(app: AppHandle, workspace_id: i32) -> AppResult<()> {
    info!("Deleting workspace: {}", &workspace_id);
    let db = crate::database::connection::get_db_connection(&app)
        .await?;
    crate::database::repositories::workspace_repository::WorkspaceRepository::delete(&db, &workspace_id)
        .await?;

    // Note: The file-based workspace storage seems to be legacy code, but we'll keep it for now
    // In a real application, we should probably remove this duplication
    Ok(())
}