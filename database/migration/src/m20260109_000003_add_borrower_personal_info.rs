use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new columns to borrowers table one by one for SQLite compatibility
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("date_of_birth")).date().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("social_security_number")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("address")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("city")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("state")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("zip_code")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .add_column(ColumnDef::new(Alias::new("mailing_address_different")).boolean().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the added columns one by one
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("date_of_birth"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("social_security_number"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("address"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("city"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("state"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("zip_code"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("borrowers"))
                    .drop_column(Alias::new("mailing_address_different"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}