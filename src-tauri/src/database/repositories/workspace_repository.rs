//! workspace repository

use sea_orm::*;
use crate::database::models::workspace::{self, Entity as Workspace, Model as WorkspaceModel};
use crate::utils::error::{AppError, AppResult};

/// workspace repository
pub struct WorkspaceRepository;

impl WorkspaceRepository {

    // Get workspace by id
    pub async fn get_by_id(db: &DatabaseConnection, id: &i32) -> AppResult<Option<WorkspaceModel>> {
        let workspace = Workspace::find_by_id(*id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(workspace)
    }

    // get current active workspace\
    pub async fn get_active(db: &DatabaseConnection) -> AppResult<Option<WorkspaceModel>> {
        let workspace = Workspace::find()
            .filter(workspace::Column::IsActive.eq(true))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(workspace)
    }
    
    // query all workspaces limit to 5
    pub async fn get_all(db: &DatabaseConnection) -> AppResult<Vec<WorkspaceModel>> {
        let workspaces = Workspace::find()
            .order_by(workspace::Column::Id, Order::Desc)
            .limit(5)
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(workspaces)
    }

    /// Save a new workspace
    pub async fn upsert(
        db: &DatabaseConnection,
        name: &str,
        path: &str,
        is_active: bool,
    ) -> AppResult<WorkspaceModel> {
        // Try to find existing workspace by path
        let existing = Workspace::find()
            .filter(workspace::Column::Path.eq(path))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let model = if let Some(existing_model) = existing {
            // Update existing workspace
            let mut active_model: workspace::ActiveModel = existing_model.into();
            active_model.name = Set(name.to_string());
            active_model.is_active = Set(is_active);
            active_model.update(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        } else {
            // Create new workspace
            let new_active_model = workspace::ActiveModel {
                name: Set(name.to_string()),
                path: Set(path.to_string()),
                is_active: Set(is_active),
                ..Default::default()
            };
            new_active_model.insert(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        };
        Ok(model)
    }

    /// Delete a workspace by id
    pub async fn delete(db: &DatabaseConnection, id: &i32) -> AppResult<()> {
        let workspace = Workspace::find_by_id(*id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(workspace) = workspace {
            workspace.delete(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }
}
