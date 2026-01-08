use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create existing_loans table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("existing_loans"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("lender_name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("loan_number")).string().not_null())
                    .col(ColumnDef::new(Alias::new("loan_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("original_balance")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("current_balance")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("interest_rate")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("remaining_term_months")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_existing_loans_borrower_id")
                            .from(Alias::new("existing_loans"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create new_loan_details table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("new_loan_details"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("loan_amount")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("interest_rate")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("term_years")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_interest")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_cost")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_new_loan_details_borrower_id")
                            .from(Alias::new("new_loan_details"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create loan_information table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("loan_information"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("property_value")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("loan_to_value_ratio")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("credit_score")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("debt_to_income_ratio")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("employment_status")).string().not_null())
                    .col(ColumnDef::new(Alias::new("years_employed")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_loan_information_borrower_id")
                            .from(Alias::new("loan_information"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

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

        manager
            .drop_table(Table::drop().table(Alias::new("loan_information")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("new_loan_details")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("existing_loans")).to_owned())
            .await?;

        Ok(())
    }
}