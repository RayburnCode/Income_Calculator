use sea_orm_migration::prelude::*;
use chrono;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create campaigns table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("campaigns"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("campaign_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("template_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_campaigns_template_id")
                            .from(Alias::new("campaigns"), Alias::new("template_id"))
                            .to(Alias::new("outreach_templates"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(Alias::new("segment_criteria")).json().not_null())
                    .col(ColumnDef::new(Alias::new("status")).string().not_null())
                    .col(ColumnDef::new(Alias::new("scheduled_date")).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Alias::new("completed_date")).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Alias::new("target_audience_count")).integer().null())
                    .col(ColumnDef::new(Alias::new("sent_count")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("opened_count")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("clicked_count")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("converted_count")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("created_by")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create ab_tests table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("ab_tests"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("campaign_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ab_tests_campaign_id")
                            .from(Alias::new("ab_tests"), Alias::new("campaign_id"))
                            .to(Alias::new("campaigns"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Alias::new("test_name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("subject_a")).string().not_null())
                    .col(ColumnDef::new(Alias::new("subject_b")).string().not_null())
                    .col(ColumnDef::new(Alias::new("winner")).string().null())
                    .col(ColumnDef::new(Alias::new("sent_a")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("sent_b")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("opened_a")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("opened_b")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("clicked_a")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("clicked_b")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Add foreign key constraints
        // Note: Foreign keys are now defined inline with column definitions for SQLite compatibility

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables (foreign keys will be dropped automatically)
        manager
            .drop_table(Table::drop().table(Alias::new("ab_tests")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("campaigns")).to_owned())
            .await?;

        Ok(())
    }
}