use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use std::error::Error;
use std::fmt;

use crate::entities::{sync_log, sync_devices};
use crate::entities::prelude::{SyncLogEntity, SyncDevicesEntity};

#[derive(Debug, Clone)]
pub enum SyncError {
    DatabaseError(String),
    AuthenticationError(String),
    ConflictError(String),
    EncryptionError(String),
    NetworkError(String),
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            SyncError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            SyncError::ConflictError(msg) => write!(f, "Conflict error: {}", msg),
            SyncError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            SyncError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl Error for SyncError {}

impl From<sea_orm::DbErr> for SyncError {
    fn from(err: sea_orm::DbErr) -> Self {
        SyncError::DatabaseError(err.to_string())
    }
}

/// Represents a change to be synchronized
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncChange {
    pub table_name: String,
    pub record_id: String,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub version: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub device_id: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SyncOperation {
    Insert,
    Update,
    Delete,
}

impl fmt::Display for SyncOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncOperation::Insert => write!(f, "INSERT"),
            SyncOperation::Update => write!(f, "UPDATE"),
            SyncOperation::Delete => write!(f, "DELETE"),
        }
    }
}

impl SyncOperation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INSERT" => Some(SyncOperation::Insert),
            "UPDATE" => Some(SyncOperation::Update),
            "DELETE" => Some(SyncOperation::Delete),
            _ => None,
        }
    }
}

/// Configuration for conflict resolution
#[derive(Clone, Debug)]
pub enum ConflictResolution {
    LocalWins,
    RemoteWins,
    LatestTimestampWins,
    Manual, // Requires user intervention
}

/// Change tracker that logs all modifications
pub struct ChangeTracker {
    db: DatabaseConnection,
    device_id: String,
}

impl ChangeTracker {
    pub fn new(db: DatabaseConnection, device_id: String) -> Self {
        Self { db, device_id }
    }

    /// Log a change to the sync_log table
    pub async fn log_change(
        &self,
        table_name: &str,
        record_id: &str,
        operation: SyncOperation,
        data: serde_json::Value,
    ) -> Result<sync_log::Model, SyncError> {
        let timestamp = Utc::now();
        let version = self.get_next_version().await?;
        
        // Create hash for integrity verification
        let hash = self.compute_hash(table_name, record_id, &operation, &data, version);

        let change = sync_log::ActiveModel {
            table_name: Set(table_name.to_string()),
            record_id: Set(record_id.to_string()),
            operation: Set(operation.to_string()),
            change_data: Set(data),
            device_id: Set(self.device_id.clone()),
            version: Set(version),
            timestamp: Set(timestamp),
            hash: Set(hash),
            is_synced: Set(false),
            ..Default::default()
        };

        let result = SyncLogEntity::insert(change)
            .exec_with_returning(&self.db)
            .await?;

        Ok(result)
    }

    /// Get all unsynced changes
    pub async fn get_unsynced_changes(&self) -> Result<Vec<sync_log::Model>, SyncError> {
        let changes = SyncLogEntity::find()
            .filter(sync_log::Column::IsSynced.eq(false))
            .all(&self.db)
            .await?;

        Ok(changes)
    }

    /// Mark changes as synced
    pub async fn mark_as_synced(&self, change_ids: Vec<i32>) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;
        
        for id in change_ids {
            if let Some(change) = SyncLogEntity::find_by_id(id).one(&self.db).await? {
                let mut active: sync_log::ActiveModel = change.into();
                active.is_synced = Set(true);
                active.update(&self.db).await?;
            }
        }

        Ok(())
    }

    /// Get next version number
    async fn get_next_version(&self) -> Result<i64, SyncError> {
        use sea_orm::QueryOrder;
        
        let last_change = SyncLogEntity::find()
            .filter(sync_log::Column::DeviceId.eq(&self.device_id))
            .order_by_desc(sync_log::Column::Version)
            .one(&self.db)
            .await?;

        Ok(last_change.map(|c| c.version + 1).unwrap_or(1))
    }

    /// Compute SHA256 hash for integrity
    fn compute_hash(
        &self,
        table_name: &str,
        record_id: &str,
        operation: &SyncOperation,
        data: &serde_json::Value,
        version: i64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(table_name.as_bytes());
        hasher.update(record_id.as_bytes());
        hasher.update(operation.to_string().as_bytes());
        hasher.update(data.to_string().as_bytes());
        hasher.update(version.to_string().as_bytes());
        hasher.update(self.device_id.as_bytes());
        
        format!("{:x}", hasher.finalize())
    }

    /// Verify hash integrity
    pub fn verify_hash(&self, change: &sync_log::Model) -> bool {
        let operation = SyncOperation::from_str(&change.operation).unwrap();
        let computed_hash = self.compute_hash(
            &change.table_name,
            &change.record_id,
            &operation,
            &change.change_data,
            change.version,
        );
        computed_hash == change.hash
    }
}

/// Device authorization manager
pub struct DeviceManager {
    db: DatabaseConnection,
}

impl DeviceManager {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Register a new device
    pub async fn register_device(
        &self,
        device_id: &str,
        device_name: &str,
        public_key: &str,
        tailscale_ip: Option<String>,
    ) -> Result<sync_devices::Model, SyncError> {
        let device = sync_devices::ActiveModel {
            device_id: Set(device_id.to_string()),
            device_name: Set(device_name.to_string()),
            public_key: Set(public_key.to_string()),
            tailscale_ip: Set(tailscale_ip),
            is_authorized: Set(false), // Requires manual authorization
            last_sync_at: Set(None),
            created_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = SyncDevicesEntity::insert(device)
            .exec_with_returning(&self.db)
            .await?;

        Ok(result)
    }

    /// Authorize a device
    pub async fn authorize_device(&self, device_id: &str) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;
        
        let device = SyncDevicesEntity::find()
            .filter(sync_devices::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| SyncError::AuthenticationError("Device not found".to_string()))?;

        let mut active: sync_devices::ActiveModel = device.into();
        active.is_authorized = Set(true);
        active.update(&self.db).await?;

        Ok(())
    }

    /// Check if device is authorized
    pub async fn is_authorized(&self, device_id: &str) -> Result<bool, SyncError> {
        let device = SyncDevicesEntity::find()
            .filter(sync_devices::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await?;

        Ok(device.map(|d| d.is_authorized).unwrap_or(false))
    }

    /// Get all authorized devices
    pub async fn get_authorized_devices(&self) -> Result<Vec<sync_devices::Model>, SyncError> {
        let devices = SyncDevicesEntity::find()
            .filter(sync_devices::Column::IsAuthorized.eq(true))
            .all(&self.db)
            .await?;

        Ok(devices)
    }

    /// Update last sync time
    pub async fn update_last_sync(&self, device_id: &str) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;
        
        if let Some(device) = SyncDevicesEntity::find()
            .filter(sync_devices::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await?
        {
            let mut active: sync_devices::ActiveModel = device.into();
            active.last_sync_at = Set(Some(Utc::now()));
            active.update(&self.db).await?;
        }

        Ok(())
    }

    /// Revoke device authorization
    pub async fn revoke_device(&self, device_id: &str) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;
        
        if let Some(device) = SyncDevicesEntity::find()
            .filter(sync_devices::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await?
        {
            let mut active: sync_devices::ActiveModel = device.into();
            active.is_authorized = Set(false);
            active.update(&self.db).await?;
        }

        Ok(())
    }
}
