use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create conditions table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("conditions"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("client_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conditions_client_id")
                            .from(Alias::new("conditions"), Alias::new("client_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().not_null())
                    .col(ColumnDef::new(Alias::new("condition_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("severity")).string().not_null())
                    .col(ColumnDef::new(Alias::new("status")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop conditions table
        manager
            .drop_table(Table::drop().table(Alias::new("conditions")).to_owned())
            .await?;

        Ok(())
    }
}