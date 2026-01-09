use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create documents table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("documents"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("client_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_documents_client_id")
                            .from(Alias::new("documents"), Alias::new("client_id"))
                            .to(Alias::new("borrowers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Alias::new("filename")).string().not_null())
                    .col(ColumnDef::new(Alias::new("file_size")).big_integer().not_null())
                    .col(ColumnDef::new(Alias::new("file_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("file_path")).string().not_null())
                    .col(ColumnDef::new(Alias::new("mime_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("upload_date")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("checksum")).string().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop documents table
        manager
            .drop_table(Table::drop().table(Alias::new("documents")).to_owned())
            .await?;

        Ok(())
    }
}