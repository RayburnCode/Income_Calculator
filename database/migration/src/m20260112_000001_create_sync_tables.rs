use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create sync_log table to track all changes
        manager
            .create_table(
                Table::create()
                    .table(SyncLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SyncLog::TableName).string().not_null())
                    .col(ColumnDef::new(SyncLog::RecordId).string().not_null())
                    .col(
                        ColumnDef::new(SyncLog::Operation)
                            .string()
                            .not_null()
                            .comment("INSERT, UPDATE, DELETE"),
                    )
                    .col(ColumnDef::new(SyncLog::ChangeData).json().not_null())
                    .col(ColumnDef::new(SyncLog::DeviceId).string().not_null())
                    .col(ColumnDef::new(SyncLog::Version).big_integer().not_null())
                    .col(ColumnDef::new(SyncLog::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(SyncLog::Hash).string().not_null().comment("SHA256 hash of the change"))
                    .col(ColumnDef::new(SyncLog::IsSynced).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        // Create sync_devices table to track authorized devices
        manager
            .create_table(
                Table::create()
                    .table(SyncDevices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncDevices::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SyncDevices::DeviceId).string().not_null().unique_key())
                    .col(ColumnDef::new(SyncDevices::DeviceName).string().not_null())
                    .col(ColumnDef::new(SyncDevices::PublicKey).text().not_null())
                    .col(ColumnDef::new(SyncDevices::TailscaleIp).string())
                    .col(ColumnDef::new(SyncDevices::IsAuthorized).boolean().not_null().default(false))
                    .col(ColumnDef::new(SyncDevices::LastSyncAt).timestamp())
                    .col(ColumnDef::new(SyncDevices::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create sync_conflicts table to handle merge conflicts
        manager
            .create_table(
                Table::create()
                    .table(SyncConflicts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncConflicts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SyncConflicts::TableName).string().not_null())
                    .col(ColumnDef::new(SyncConflicts::RecordId).string().not_null())
                    .col(ColumnDef::new(SyncConflicts::LocalData).json().not_null())
                    .col(ColumnDef::new(SyncConflicts::RemoteData).json().not_null())
                    .col(ColumnDef::new(SyncConflicts::LocalVersion).big_integer().not_null())
                    .col(ColumnDef::new(SyncConflicts::RemoteVersion).big_integer().not_null())
                    .col(ColumnDef::new(SyncConflicts::LocalDeviceId).string().not_null())
                    .col(ColumnDef::new(SyncConflicts::RemoteDeviceId).string().not_null())
                    .col(
                        ColumnDef::new(SyncConflicts::Status)
                            .string()
                            .not_null()
                            .default("pending")
                            .comment("pending, resolved, ignored"),
                    )
                    .col(ColumnDef::new(SyncConflicts::Resolution).json())
                    .col(ColumnDef::new(SyncConflicts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SyncConflicts::ResolvedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_sync_log_table_record")
                    .table(SyncLog::Table)
                    .col(SyncLog::TableName)
                    .col(SyncLog::RecordId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sync_log_synced")
                    .table(SyncLog::Table)
                    .col(SyncLog::IsSynced)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sync_log_timestamp")
                    .table(SyncLog::Table)
                    .col(SyncLog::Timestamp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SyncConflicts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SyncDevices::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SyncLog::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum SyncLog {
    Table,
    Id,
    TableName,
    RecordId,
    Operation,
    ChangeData,
    DeviceId,
    Version,
    Timestamp,
    Hash,
    IsSynced,
}

#[derive(Iden)]
enum SyncDevices {
    Table,
    Id,
    DeviceId,
    DeviceName,
    PublicKey,
    TailscaleIp,
    IsAuthorized,
    LastSyncAt,
    CreatedAt,
}

#[derive(Iden)]
enum SyncConflicts {
    Table,
    Id,
    TableName,
    RecordId,
    LocalData,
    RemoteData,
    LocalVersion,
    RemoteVersion,
    LocalDeviceId,
    RemoteDeviceId,
    Status,
    Resolution,
    CreatedAt,
    ResolvedAt,
}
