use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create junction tables for many-to-many relationships
        // mortgage_refinance_options_existing_loans junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_options_existing_loans"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("mortgage_refinance_option_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("existing_loan_id")).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(Alias::new("mortgage_refinance_option_id"))
                            .col(Alias::new("existing_loan_id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mro_el_mortgage_refinance_option_id")
                            .from(Alias::new("mortgage_refinance_options_existing_loans"), Alias::new("mortgage_refinance_option_id"))
                            .to(Alias::new("mortgage_refinance_options"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mro_el_existing_loan_id")
                            .from(Alias::new("mortgage_refinance_options_existing_loans"), Alias::new("existing_loan_id"))
                            .to(Alias::new("existing_loans"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // mortgage_refinance_options_consumer_debts junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_options_consumer_debts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("mortgage_refinance_option_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("consumer_debt_id")).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(Alias::new("mortgage_refinance_option_id"))
                            .col(Alias::new("consumer_debt_id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mro_cd_mortgage_refinance_option_id")
                            .from(Alias::new("mortgage_refinance_options_consumer_debts"), Alias::new("mortgage_refinance_option_id"))
                            .to(Alias::new("mortgage_refinance_options"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mro_cd_consumer_debt_id")
                            .from(Alias::new("mortgage_refinance_options_consumer_debts"), Alias::new("consumer_debt_id"))
                            .to(Alias::new("consumer_debts"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop junction tables
        manager
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_options_consumer_debts")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_options_existing_loans")).to_owned())
            .await?;

        Ok(())
    }
}