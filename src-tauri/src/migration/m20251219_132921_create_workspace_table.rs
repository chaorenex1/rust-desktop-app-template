//! Migration: Create workspace table

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create workspace table
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workspace::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Workspace::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Workspace::Path)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Workspace::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Workspace::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Workspace::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Workspace::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on path for quick lookup
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_workspace_path")
                    .table(Workspace::Table)
                    .col(Workspace::Path)
                    .to_owned(),
            )
            .await?;

        // Create index on is_active for filtering active workspaces
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_workspace_is_active")
                    .table(Workspace::Table)
                    .col(Workspace::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(Index::drop().name("idx_workspace_is_active").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_workspace_path").to_owned())
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Define the table and column identifiers
#[derive(DeriveIden)]
enum Workspace {
    Table,
    Id,
    Name,
    Path,
    IsActive,
    Description,
    CreatedAt,
    UpdatedAt,
}
