//! Recent directories repository

use sea_orm::*;
use crate::utils::error::AppResult;

/// Recent directory entry
#[derive(Debug, Clone)]
pub struct RecentDirectory {
    pub path: String,
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

/// Recent directories repository
pub struct RecentDirectoriesRepository;

impl RecentDirectoriesRepository {
    /// Add a recent directory
    pub async fn add(db: &DatabaseConnection, path: &str) -> AppResult<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        // Delete existing entry with the same path
        let _ = db
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "DELETE FROM settings WHERE key = ? AND category = 'recent_directory'",
                vec![path.into()],
            ))
            .await;
        
        // Insert new entry
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT INTO settings (key, value, category, created_at, updated_at) VALUES (?, ?, 'recent_directory', ?, ?)",
            vec![path.into(), now.clone().into(), now.clone().into(), now.into()],
        ))
        .await?;
        
        // Keep only the 5 most recent directories
        db.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            format!(
                "DELETE FROM settings WHERE category = 'recent_directory' AND id NOT IN \
                (SELECT id FROM settings WHERE category = 'recent_directory' ORDER BY updated_at DESC LIMIT 5)"
            ),
        ))
        .await?;
        
        Ok(())
    }
    
    /// Get recent directories (limited to 5)
    pub async fn get_recent(db: &DatabaseConnection) -> AppResult<Vec<RecentDirectory>> {
        let results = db
            .query_all(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT key, updated_at FROM settings WHERE category = 'recent_directory' ORDER BY updated_at DESC LIMIT 5".to_string(),
            ))
            .await?;
        
        let mut directories = Vec::new();
        for row in results {
            let path: String = row.try_get("", "key")?;
            let updated_at: String = row.try_get("", "updated_at")?;
            
            directories.push(RecentDirectory {
                path,
                opened_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            });
        }
        
        Ok(directories)
    }
    
    /// Clear all recent directories
    pub async fn clear(db: &DatabaseConnection) -> AppResult<()> {
        db.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "DELETE FROM settings WHERE category = 'recent_directory'".to_string(),
        ))
        .await?;
        
        Ok(())
    }
}
