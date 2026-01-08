use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create other_fees table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("other_fees"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("fee_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("fee_amount")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("paid_by")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_other_fees_borrower_id")
                            .from(Alias::new("other_fees"), Alias::new("borrower_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create mortgage_refinance_options table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_options"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("borrower_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("option_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("loan_amount")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("interest_rate")).decimal_len(5, 4).not_null())
                    .col(ColumnDef::new(Alias::new("term_years")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("monthly_payment")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_interest")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("total_cost")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("closing_costs")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("cash_to_close")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("cash_out")).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Alias::new("is_recommended")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_options_borrower_id")
                            .from(Alias::new("mortgage_refinance_options"), Alias::new("borrower_id"))
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
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_options")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("other_fees")).to_owned())
            .await?;

        Ok(())
    }
}