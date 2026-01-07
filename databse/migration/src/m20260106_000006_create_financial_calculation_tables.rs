use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create benefit_to_borrower table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("benefit_to_borrower"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("monthly_savings")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("annual_savings")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_savings")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("break_even_point")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("cash_to_close")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("cash_out")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_benefit_to_borrower_borrower_id")
                            .from(Alias::new("benefit_to_borrower"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create pricing_options table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("pricing_options"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("option_name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("interest_rate")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_cost")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("closing_costs")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("is_recommended")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pricing_options_borrower_id")
                            .from(Alias::new("pricing_options"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create savings_calculations table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("savings_calculations"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("current_monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("new_monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("monthly_savings")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("annual_savings")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_savings_5_years")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_savings_10_years")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("break_even_months")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_savings_calculations_borrower_id")
                            .from(Alias::new("savings_calculations"), Alias::new("borrower_id"))
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
        manager
            .drop_table(Table::drop().table(Alias::new("savings_calculations")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("pricing_options")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("benefit_to_borrower")).to_owned())
            .await?;

        Ok(())
    }
}