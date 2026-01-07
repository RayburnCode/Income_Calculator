use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // This migration has been replaced by the separated migrations
        // This file exists only to satisfy SeaORM's migration tracking
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // This migration has been replaced by the separated migrations
        // This file exists only to satisfy SeaORM's migration tracking
        Ok(())
    }
}