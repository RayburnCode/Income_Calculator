use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add borrower_monthly_income column to income_information table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("income_information"))
                    .add_column(ColumnDef::new(Alias::new("borrower_monthly_income")).decimal_len(15, 2).null())
                    .to_owned(),
            )
            .await?;

        // Add coborrower_monthly_income column to income_information table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("income_information"))
                    .add_column(ColumnDef::new(Alias::new("coborrower_monthly_income")).decimal_len(15, 2).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove coborrower_monthly_income column from income_information table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("income_information"))
                    .drop_column(Alias::new("coborrower_monthly_income"))
                    .to_owned(),
            )
            .await?;

        // Remove borrower_monthly_income column from income_information table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("income_information"))
                    .drop_column(Alias::new("borrower_monthly_income"))
                    .to_owned(),
            )
            .await
    }
}
