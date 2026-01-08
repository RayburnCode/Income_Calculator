use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create w2_jobs table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("w2_jobs"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("employer_name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("job_title")).string().not_null())
                    .col(ColumnDef::new(Alias::new("years_employed")).integer().null())
                    .col(ColumnDef::new(Alias::new("months_employed")).integer().null())
                    .col(ColumnDef::new(Alias::new("annual_salary")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("hourly_rate")).decimal_len(10, 2).null())
                    .col(ColumnDef::new(Alias::new("hours_per_week")).integer().null())
                    .col(ColumnDef::new(Alias::new("commission_monthly")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("bonus_monthly")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("overtime_monthly")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_w2_jobs_borrower_id")
                            .from(Alias::new("w2_jobs"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop w2_jobs table
        manager
            .drop_table(Table::drop().table(Alias::new("w2_jobs")).to_owned())
            .await?;

        Ok(())
    }
}