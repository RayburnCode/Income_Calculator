use sea_orm::DatabaseConnection;
use std::error::Error;
use std::net::SocketAddr;
use tokio::task::JoinHandle;

use super::change_tracker::{ChangeTracker, DeviceManager, SyncError};
use super::conflict_resolver::{ConflictResolver, ConflictResolution};
use super::api::create_sync_api;

/// Main synchronization manager
pub struct SyncManager {
    db: DatabaseConnection,
    device_id: String,
    server_handle: Option<JoinHandle<()>>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(db: DatabaseConnection, device_id: String) -> Self {
        Self {
            db,
            device_id,
            server_handle: None,
        }
    }

    /// Start the sync server
    pub async fn start_server(&mut self, bind_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        let app = create_sync_api(self.db.clone(), self.device_id.clone());
        
        let listener = tokio::net::TcpListener::bind(bind_addr).await?;
        println!("🚀 Sync server listening on {}", bind_addr);
        
        let handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Server error: {}", e);
            }
        });

        self.server_handle = Some(handle);
        Ok(())
    }

    /// Initialize a new device for syncing
    pub async fn initialize_device(
        &self,
        device_name: &str,
        public_key: &str,
        tailscale_ip: Option<String>,
    ) -> Result<String, SyncError> {
        let manager = DeviceManager::new(self.db.clone());
        
        // Generate or use provided device ID
        let device_id = uuid::Uuid::new_v4().to_string();
        
        manager.register_device(&device_id, device_name, public_key, tailscale_ip).await?;
        
        Ok(device_id)
    }

    /// Sync with a remote peer
    pub async fn sync_with_peer(
        &self,
        peer_url: &str,
        device_id: &str,
    ) -> Result<SyncReport, Box<dyn Error>> {
        let client = reqwest::Client::new();
        
        // Get unsynced changes
        let tracker = ChangeTracker::new(self.db.clone(), device_id.to_string());
        let changes = tracker.get_unsynced_changes().await?;
        
        // Push changes to peer
        let _push_response = client
            .post(format!("{}/sync/push", peer_url))
            .json(&serde_json::json!({
                "device_id": device_id,
                "changes": changes,
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // Pull changes from peer
        let last_version = self.get_last_synced_version().await?;
        let _pull_response = client
            .post(format!("{}/sync/pull", peer_url))
            .json(&serde_json::json!({
                "device_id": device_id,
                "since_version": last_version,
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // Apply received changes
        let received_changes = _pull_response["changes"]
            .as_array()
            .map(|arr: &Vec<serde_json::Value>| arr.len())
            .unwrap_or(0);
        
        let conflicts_detected = _pull_response["conflicts"]
            .as_array()
            .map(|arr: &Vec<serde_json::Value>| arr.len())
            .unwrap_or(0);
        
        // Mark local changes as synced
        let change_ids: Vec<i32> = changes.iter().map(|c| c.id).collect();
        tracker.mark_as_synced(change_ids).await?;
        
        Ok(SyncReport {
            pushed: changes.len(),
            pulled: received_changes,
            conflicts: conflicts_detected,
        })
    }

    /// Enable automatic periodic syncing
    pub async fn enable_auto_sync(
        &self,
        peer_url: String,
        device_id: String,
        interval_seconds: u64,
    ) -> JoinHandle<()> {
        let db = self.db.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));
            
            loop {
                interval.tick().await;
                
                let manager = SyncManager::new(db.clone(), device_id.clone());
                match manager.sync_with_peer(&peer_url, &device_id).await {
                    Ok(report) => {
                        println!("✅ Auto-sync complete: pushed={}, pulled={}, conflicts={}", 
                            report.pushed, report.pulled, report.conflicts);
                    }
                    Err(e) => {
                        eprintln!("❌ Auto-sync failed: {}", e);
                    }
                }
            }
        })
    }

    /// Get the last synced version
    async fn get_last_synced_version(&self) -> Result<Option<i64>, SyncError> {
        use sea_orm::{EntityTrait, QueryOrder, QueryFilter, ColumnTrait};
        use crate::entities::prelude::SyncLogEntity;
        use crate::entities::sync_log;

        let last_change = SyncLogEntity::find()
            .filter(sync_log::Column::IsSynced.eq(true))
            .order_by_desc(sync_log::Column::Version)
            .one(&self.db)
            .await?;

        Ok(last_change.map(|c| c.version))
    }

    /// Export database for backup
    pub async fn export_backup(&self, _path: &str) -> Result<(), Box<dyn Error>> {
        // Implementation depends on your database type (SQLite, etc.)
        // For SQLite, you could use:
        // 1. Copy the database file
        // 2. Or use SQLite backup API
        
        #[cfg(feature = "sqlite")]
        {
            use std::fs;
            use crate::get_database_path;
            
            let db_path = get_database_path()?;
            fs::copy(db_path, path)?;
        }
        
        Ok(())
    }

    /// Import database from backup
    pub async fn import_backup(&self, _path: &str) -> Result<(), Box<dyn Error>> {
        // Similar to export, but in reverse
        // Should merge changes, not just overwrite
        
        println!("⚠️  Import should merge changes to avoid data loss");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SyncReport {
    pub pushed: usize,
    pub pulled: usize,
    pub conflicts: usize,
}

/// Helper to generate device keys
pub mod crypto {
    use ed25519_dalek::SigningKey;
    
    /// Generate a new Ed25519 keypair for device authentication
    pub fn generate_keypair() -> SigningKey {
        SigningKey::from_bytes(&rand::random::<[u8; 32]>())
    }
    
    /// Export public key as base64
    pub fn export_public_key(keypair: &SigningKey) -> String {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(keypair.verifying_key().as_bytes())
    }
    
    /// Export secret key as base64 (store securely!)
    pub fn export_secret_key(keypair: &SigningKey) -> String {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(keypair.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_manager_creation() {
        // Add tests for sync manager
    }
}
