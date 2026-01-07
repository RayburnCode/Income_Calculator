use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add status column to the existing borrowers table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(
                        ColumnDef::new(Alias::new("status"))
                            .string()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add email column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(
                        ColumnDef::new(Alias::new("email"))
                            .string()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add phone_number column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(
                        ColumnDef::new(Alias::new("phone_number"))
                            .string()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the added columns
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("status"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("email"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("phone_number"))
                    .to_owned(),
            )
            .await
    }
}