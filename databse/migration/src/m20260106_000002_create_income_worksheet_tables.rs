use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create consumer_debts table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("consumer_debts"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("debtor_name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("credit_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("balance")).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Alias::new("monthly_payment")).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Alias::new("term_months")).integer().null())
                    .col(ColumnDef::new(Alias::new("interest_rate")).decimal_len(5, 2).null())
                    .col(ColumnDef::new(Alias::new("omit_from_dti")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("pay_off_at_closing")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consumer_debts_borrower_id")
                            .from(Alias::new("consumer_debts"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create income_information table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("income_information"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("income_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("gross_income")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("net_income")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("frequency")).string().not_null())
                    .col(ColumnDef::new(Alias::new("hours_per_week")).integer().null())
                    .col(ColumnDef::new(Alias::new("weeks_per_year")).integer().null())
                    .col(ColumnDef::new(Alias::new("overtime")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("commissions")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("bonuses")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("other_income")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("borrower_monthly_income")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("coborrower_monthly_income")).decimal_len(15, 2).null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_income_information_borrower_id")
                            .from(Alias::new("income_information"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

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

        // Drop income_information table
        manager
            .drop_table(Table::drop().table(Alias::new("income_information")).to_owned())
            .await?;

        // Drop consumer_debts table
        manager
            .drop_table(Table::drop().table(Alias::new("consumer_debts")).to_owned())
            .await?;

        Ok(())
    }
}