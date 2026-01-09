use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create notes table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("notes"))
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
                            .name("fk_notes_client_id")
                            .from(Alias::new("notes"), Alias::new("client_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("content")).text().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop notes table
        manager
            .drop_table(Table::drop().table(Alias::new("notes")).to_owned())
            .await?;

        Ok(())
    }
}