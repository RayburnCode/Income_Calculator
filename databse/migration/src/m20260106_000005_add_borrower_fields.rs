use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Note: status, email, and phone_number columns are already added in the create table migration
        // This migration is a no-op to avoid duplicate column errors
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Note: status, email, and phone_number columns are part of the create table migration
        // This migration is a no-op
        Ok(())
    }
}