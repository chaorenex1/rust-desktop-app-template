use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Conversion::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Conversion::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Conversion::Name).string().not_null())
                    .col(ColumnDef::new(Conversion::ConversionId).string().not_null())
                    .col(ColumnDef::new(Conversion::Description).string().null().default(""))
                    .col(ColumnDef::new(Conversion::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Conversion::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        
        // Create index for conversion_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_conversion_id")
                    .table(Conversion::Table)
                    .col(Conversion::ConversionId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_conversion_id").table(Conversion::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Conversion::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Define the table and column identifiers
#[derive(DeriveIden)]
enum Conversion {
    Table,
    Id,
    Name,
    ConversionId,
    Description,
    CreatedAt,
    UpdatedAt,
}
