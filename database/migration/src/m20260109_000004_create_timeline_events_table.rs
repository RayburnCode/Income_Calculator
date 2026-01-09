use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create timeline_events table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("timeline_events"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("event_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("metadata")).json().null())
                    .col(ColumnDef::new(Alias::new("user_id")).string().null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign key first
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_timeline_events_borrower_id")
                    .table(Alias::new("timeline_events"))
                    .to_owned(),
            )
            .await?;

        // Drop timeline_events table
        manager
            .drop_table(Table::drop().table(Alias::new("timeline_events")).to_owned())
            .await?;

        Ok(())
    }
}