//! Settings repository for database operations

use sea_orm::*;
use crate::database::models::settings::{self, Entity as Settings, Model as SettingsModel};
use crate::utils::error::{AppError, AppResult};

/// Settings repository
pub struct SettingsRepository;

impl SettingsRepository {
    /// Get a setting by key
    pub async fn get_by_key(db: &DatabaseConnection, key: &str) -> AppResult<Option<SettingsModel>> {
        let setting = Settings::find()
            .filter(settings::Column::Key.eq(key))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(setting)
    }

    /// Get all settings by category
    pub async fn get_by_category(db: &DatabaseConnection, category: &str) -> AppResult<Vec<SettingsModel>> {
        let settings = Settings::find()
            .filter(settings::Column::Category.eq(category))
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(settings)
    }

    /// Get all settings
    pub async fn get_all(db: &DatabaseConnection) -> AppResult<Vec<SettingsModel>> {
        let settings = Settings::find()
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(settings)
    }

    /// Save or update a setting
    pub async fn upsert(
        db: &DatabaseConnection,
        key: &str,
        value: &str,
        category: &str,
        description: Option<&str>,
    ) -> AppResult<SettingsModel> {
        // Try to find existing setting
        let existing = Self::get_by_key(db, key).await?;

        let model = if let Some(existing_model) = existing {
            // Update existing setting
            let mut active_model: settings::ActiveModel = existing_model.into();
            active_model.value = Set(value.to_string());
            active_model.category = Set(category.to_string());
            if let Some(desc) = description {
                active_model.description = Set(Some(desc.to_string()));
            }
            
            active_model
                .update(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        } else {
            // Create new setting
            let new_setting = settings::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                category: Set(category.to_string()),
                description: Set(description.map(|s| s.to_string())),
                ..Default::default()
            };

            new_setting
                .insert(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        };

        Ok(model)
    }

    /// Delete a setting by key
    pub async fn delete_by_key(db: &DatabaseConnection, key: &str) -> AppResult<bool> {
        let result = Settings::delete_many()
            .filter(settings::Column::Key.eq(key))
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected > 0)
    }

    /// Delete all settings in a category
    pub async fn delete_by_category(db: &DatabaseConnection, category: &str) -> AppResult<u64> {
        let result = Settings::delete_many()
            .filter(settings::Column::Category.eq(category))
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected)
    }
}
