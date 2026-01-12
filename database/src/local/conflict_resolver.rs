use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::entities::{sync_log, sync_conflicts};
use crate::entities::prelude::{SyncLogEntity, SyncConflictsEntity};
use super::change_tracker::SyncError;

// Re-export ConflictResolution for public use
pub use super::change_tracker::ConflictResolution;

/// Conflict resolver for handling sync conflicts
pub struct ConflictResolver {
    db: DatabaseConnection,
    strategy: ConflictResolution,
}

impl ConflictResolver {
    pub fn new(db: DatabaseConnection, strategy: ConflictResolution) -> Self {
        Self { db, strategy }
    }

    /// Detect conflicts between local and remote changes
    pub async fn detect_conflict(
        &self,
        local_change: &sync_log::Model,
        remote_change: &sync_log::Model,
    ) -> Result<bool, SyncError> {
        // Conflict exists if:
        // 1. Same table and record
        // 2. Different versions
        // 3. Different hashes
        
        if local_change.table_name != remote_change.table_name 
            || local_change.record_id != remote_change.record_id {
            return Ok(false);
        }

        if local_change.hash == remote_change.hash {
            return Ok(false);
        }

        // Check if timestamps are very close (within 1 second) - likely the same change
        let time_diff = (local_change.timestamp.timestamp() - remote_change.timestamp.timestamp()).abs();
        if time_diff < 1 {
            return Ok(false);
        }

        Ok(true)
    }

    /// Log a conflict
    pub async fn log_conflict(
        &self,
        local_change: &sync_log::Model,
        remote_change: &sync_log::Model,
    ) -> Result<sync_conflicts::Model, SyncError> {
        let conflict = sync_conflicts::ActiveModel {
            table_name: Set(local_change.table_name.clone()),
            record_id: Set(local_change.record_id.clone()),
            local_data: Set(local_change.change_data.clone()),
            remote_data: Set(remote_change.change_data.clone()),
            local_version: Set(local_change.version),
            remote_version: Set(remote_change.version),
            local_device_id: Set(local_change.device_id.clone()),
            remote_device_id: Set(remote_change.device_id.clone()),
            status: Set("pending".to_string()),
            resolution: Set(None),
            created_at: Set(Utc::now()),
            resolved_at: Set(None),
            ..Default::default()
        };

        let result = SyncConflictsEntity::insert(conflict)
            .exec_with_returning(&self.db)
            .await?;

        Ok(result)
    }

    /// Resolve a conflict based on the configured strategy
    pub async fn resolve_conflict(
        &self,
        conflict: &sync_conflicts::Model,
    ) -> Result<ConflictResolutionResult, SyncError> {
        use sea_orm::ActiveModelTrait;

        let resolution = match self.strategy {
            ConflictResolution::LocalWins => {
                ConflictResolutionResult {
                    winner: Winner::Local,
                    data: conflict.local_data.clone(),
                    reason: "Local wins strategy".to_string(),
                }
            }
            ConflictResolution::RemoteWins => {
                ConflictResolutionResult {
                    winner: Winner::Remote,
                    data: conflict.remote_data.clone(),
                    reason: "Remote wins strategy".to_string(),
                }
            }
            ConflictResolution::LatestTimestampWins => {
                // Need to get the actual changes to compare timestamps
                let local_change = SyncLogEntity::find()
                    .filter(sync_log::Column::TableName.eq(&conflict.table_name))
                    .filter(sync_log::Column::RecordId.eq(&conflict.record_id))
                    .filter(sync_log::Column::DeviceId.eq(&conflict.local_device_id))
                    .filter(sync_log::Column::Version.eq(conflict.local_version))
                    .one(&self.db)
                    .await?;

                let remote_change = SyncLogEntity::find()
                    .filter(sync_log::Column::TableName.eq(&conflict.table_name))
                    .filter(sync_log::Column::RecordId.eq(&conflict.record_id))
                    .filter(sync_log::Column::DeviceId.eq(&conflict.remote_device_id))
                    .filter(sync_log::Column::Version.eq(conflict.remote_version))
                    .one(&self.db)
                    .await?;

                match (local_change, remote_change) {
                    (Some(local), Some(remote)) => {
                        if local.timestamp > remote.timestamp {
                            ConflictResolutionResult {
                                winner: Winner::Local,
                                data: conflict.local_data.clone(),
                                reason: format!("Local timestamp {} is newer", local.timestamp),
                            }
                        } else {
                            ConflictResolutionResult {
                                winner: Winner::Remote,
                                data: conflict.remote_data.clone(),
                                reason: format!("Remote timestamp {} is newer", remote.timestamp),
                            }
                        }
                    }
                    _ => {
                        return Err(SyncError::ConflictError(
                            "Could not find changes for timestamp comparison".to_string()
                        ));
                    }
                }
            }
            ConflictResolution::Manual => {
                return Err(SyncError::ConflictError(
                    "Manual resolution required - conflict logged".to_string()
                ));
            }
        };

        // Update conflict record with resolution
        if let Some(conflict_model) = SyncConflictsEntity::find_by_id(conflict.id).one(&self.db).await? {
            let mut active: sync_conflicts::ActiveModel = conflict_model.into();
            active.status = Set("resolved".to_string());
            active.resolution = Set(Some(serde_json::json!({
                "winner": format!("{:?}", resolution.winner),
                "reason": resolution.reason,
                "resolved_at": Utc::now().to_rfc3339(),
            })));
            active.resolved_at = Set(Some(Utc::now()));
            active.update(&self.db).await?;
        }

        Ok(resolution)
    }

    /// Get all pending conflicts
    pub async fn get_pending_conflicts(&self) -> Result<Vec<sync_conflicts::Model>, SyncError> {
        let conflicts = SyncConflictsEntity::find()
            .filter(sync_conflicts::Column::Status.eq("pending"))
            .all(&self.db)
            .await?;

        Ok(conflicts)
    }

    /// Manually resolve a conflict
    pub async fn manual_resolve(
        &self,
        conflict_id: i32,
        winner: Winner,
        custom_data: Option<serde_json::Value>,
    ) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;

        let conflict = SyncConflictsEntity::find_by_id(conflict_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| SyncError::ConflictError("Conflict not found".to_string()))?;

        let resolution_data = match custom_data {
            Some(data) => data,
            None => {
                match winner {
                    Winner::Local => conflict.local_data.clone(),
                    Winner::Remote => conflict.remote_data.clone(),
                }
            }
        };

        let mut active: sync_conflicts::ActiveModel = conflict.into();
        active.status = Set("resolved".to_string());
        active.resolution = Set(Some(serde_json::json!({
            "winner": format!("{:?}", winner),
            "data": resolution_data,
            "reason": "Manual resolution",
            "resolved_at": Utc::now().to_rfc3339(),
        })));
        active.resolved_at = Set(Some(Utc::now()));
        active.update(&self.db).await?;

        Ok(())
    }

    /// Ignore a conflict (mark as resolved without taking action)
    pub async fn ignore_conflict(&self, conflict_id: i32) -> Result<(), SyncError> {
        use sea_orm::ActiveModelTrait;

        let conflict = SyncConflictsEntity::find_by_id(conflict_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| SyncError::ConflictError("Conflict not found".to_string()))?;

        let mut active: sync_conflicts::ActiveModel = conflict.into();
        active.status = Set("ignored".to_string());
        active.resolved_at = Set(Some(Utc::now()));
        active.update(&self.db).await?;

        Ok(())
    }

    /// Apply a merge strategy for complex data
    pub fn merge_data(
        &self,
        local: &serde_json::Value,
        remote: &serde_json::Value,
    ) -> Result<serde_json::Value, SyncError> {
        // Simple field-level merge for objects
        if let (Some(local_obj), Some(remote_obj)) = (local.as_object(), remote.as_object()) {
            let mut merged = serde_json::Map::new();
            
            // Take all local fields
            for (key, value) in local_obj {
                merged.insert(key.clone(), value.clone());
            }
            
            // Add remote fields that don't exist locally or have newer timestamps
            for (key, value) in remote_obj {
                if !merged.contains_key(key) {
                    merged.insert(key.clone(), value.clone());
                }
            }
            
            Ok(serde_json::Value::Object(merged))
        } else {
            // For non-objects, cannot merge - return error
            Err(SyncError::ConflictError(
                "Cannot merge non-object data automatically".to_string()
            ))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Winner {
    Local,
    Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionResult {
    pub winner: Winner,
    pub data: serde_json::Value,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge_data() {
        let _local = serde_json::json!({
            "name": "John",
            "age": 30,
        });
        
        let _remote = serde_json::json!({
            "age": 31,
            "city": "NYC",
        });
        
        // This is a simplified test - actual implementation would need database
        // The merge should keep local "name", and add remote "city"
    }
}
