use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create settings table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("settings"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("theme")).string().not_null().default("light"))
                    .col(ColumnDef::new(Alias::new("currency")).string().not_null().default("USD ($)"))
                    .col(ColumnDef::new(Alias::new("default_loan_term")).integer().not_null().default(30))
                    .col(ColumnDef::new(Alias::new("dti_threshold")).double().not_null().default(43.0))
                    .col(ColumnDef::new(Alias::new("auto_backup")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop settings table
        manager
            .drop_table(Table::drop().table(Alias::new("settings")).to_owned())
            .await?;

        Ok(())
    }
}