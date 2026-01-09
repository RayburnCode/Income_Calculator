use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add missing columns to loan_information table one by one (SQLite limitation)
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .add_column(ColumnDef::new(Alias::new("occupancy_type")).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .add_column(ColumnDef::new(Alias::new("loan_type")).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .add_column(ColumnDef::new(Alias::new("new_term_months")).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .add_column(ColumnDef::new(Alias::new("loan_purpose")).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .add_column(ColumnDef::new(Alias::new("appraisal_waiver")).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the added columns one by one (SQLite limitation)
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .drop_column(Alias::new("occupancy_type"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .drop_column(Alias::new("loan_type"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .drop_column(Alias::new("new_term_months"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .drop_column(Alias::new("loan_purpose"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("loan_information"))
                    .drop_column(Alias::new("appraisal_waiver"))
                    .to_owned(),
            )
            .await
    }
}
