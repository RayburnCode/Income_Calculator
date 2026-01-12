use axum::{
    Router,
    routing::{get, post},
    extract::{State, Json, Path},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use super::change_tracker::{ChangeTracker, DeviceManager, SyncError, SyncOperation};
use super::conflict_resolver::{ConflictResolver, ConflictResolution, Winner};
use crate::entities::{sync_log, sync_devices, sync_conflicts};
use crate::entities::prelude::{SyncLogEntity, SyncDevicesEntity};

/// Shared state for the API
#[derive(Clone)]
pub struct ApiState {
    pub db: DatabaseConnection,
    pub device_id: String,
}

/// API error response
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for SyncError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SyncError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            SyncError::ConflictError(msg) => (StatusCode::CONFLICT, msg),
            SyncError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            SyncError::EncryptionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            SyncError::NetworkError(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

/// Request to register a new device
#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: String,
    pub device_name: String,
    pub public_key: String,
    pub tailscale_ip: Option<String>,
}

/// Response for device registration
#[derive(Serialize)]
pub struct RegisterDeviceResponse {
    pub device_id: String,
    pub is_authorized: bool,
    pub message: String,
}

/// Request to push changes
#[derive(Deserialize)]
pub struct PushChangesRequest {
    pub device_id: String,
    pub changes: Vec<SyncChangeRequest>,
}

#[derive(Deserialize, Serialize)]
pub struct SyncChangeRequest {
    pub table_name: String,
    pub record_id: String,
    pub operation: String,
    pub data: serde_json::Value,
    pub version: i64,
    pub timestamp: String,
    pub hash: String,
}

/// Response for push changes
#[derive(Serialize)]
pub struct PushChangesResponse {
    pub accepted: usize,
    pub conflicts: Vec<ConflictInfo>,
}

#[derive(Serialize)]
pub struct ConflictInfo {
    pub table_name: String,
    pub record_id: String,
    pub conflict_id: i32,
}

/// Request to pull changes
#[derive(Deserialize)]
pub struct PullChangesRequest {
    pub device_id: String,
    pub since_version: Option<i64>,
}

/// Response for pull changes
#[derive(Serialize)]
pub struct PullChangesResponse {
    pub changes: Vec<sync_log::Model>,
    pub conflicts: Vec<sync_conflicts::Model>,
}

/// Request to authorize a device
#[derive(Deserialize)]
pub struct AuthorizeDeviceRequest {
    pub device_id: String,
}

/// Request to resolve a conflict
#[derive(Deserialize)]
pub struct ResolveConflictRequest {
    pub winner: String, // "local" or "remote"
    pub custom_data: Option<serde_json::Value>,
}

/// Create the sync API router
pub fn create_sync_api(db: DatabaseConnection, device_id: String) -> Router {
    let state = ApiState { db, device_id };

    Router::new()
        .route("/health", get(health_check))
        .route("/devices/register", post(register_device))
        .route("/devices/authorize", post(authorize_device))
        .route("/devices/list", get(list_devices))
        .route("/devices/:device_id/revoke", post(revoke_device))
        .route("/sync/push", post(push_changes))
        .route("/sync/pull", post(pull_changes))
        .route("/sync/status", get(sync_status))
        .route("/conflicts/list", get(list_conflicts))
        .route("/conflicts/:id/resolve", post(resolve_conflict))
        .route("/conflicts/:id/ignore", post(ignore_conflict))
        .layer(CorsLayer::permissive()) // Configure CORS as needed
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Register a new device
async fn register_device(
    State(state): State<ApiState>,
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<Json<RegisterDeviceResponse>, SyncError> {
    let manager = DeviceManager::new(state.db);
    
    let device = manager.register_device(
        &req.device_id,
        &req.device_name,
        &req.public_key,
        req.tailscale_ip,
    ).await?;

    Ok(Json(RegisterDeviceResponse {
        device_id: device.device_id,
        is_authorized: device.is_authorized,
        message: "Device registered. Awaiting authorization.".to_string(),
    }))
}

/// Authorize a device (admin only in production)
async fn authorize_device(
    State(state): State<ApiState>,
    Json(req): Json<AuthorizeDeviceRequest>,
) -> Result<StatusCode, SyncError> {
    let manager = DeviceManager::new(state.db);
    manager.authorize_device(&req.device_id).await?;
    Ok(StatusCode::OK)
}

/// List all devices
async fn list_devices(
    State(state): State<ApiState>,
) -> Result<Json<Vec<sync_devices::Model>>, SyncError> {
    let manager = DeviceManager::new(state.db);
    let devices = manager.get_authorized_devices().await?;
    Ok(Json(devices))
}

/// Revoke device authorization
async fn revoke_device(
    State(state): State<ApiState>,
    Path(device_id): Path<String>,
) -> Result<StatusCode, SyncError> {
    let manager = DeviceManager::new(state.db);
    manager.revoke_device(&device_id).await?;
    Ok(StatusCode::OK)
}

/// Push changes from a device
async fn push_changes(
    State(state): State<ApiState>,
    Json(req): Json<PushChangesRequest>,
) -> Result<Json<PushChangesResponse>, SyncError> {
    // Verify device is authorized
    let manager = DeviceManager::new(state.db.clone());
    if !manager.is_authorized(&req.device_id).await? {
        return Err(SyncError::AuthenticationError(
            "Device not authorized".to_string()
        ));
    }

    let tracker = ChangeTracker::new(state.db.clone(), req.device_id.clone());
    let resolver = ConflictResolver::new(state.db.clone(), ConflictResolution::LatestTimestampWins);
    
    let mut accepted = 0;
    let mut conflicts = Vec::new();

    for change_req in req.changes {
        let operation = SyncOperation::from_str(&change_req.operation)
            .ok_or_else(|| SyncError::DatabaseError("Invalid operation".to_string()))?;

        // Check for conflicts with existing changes
        let existing_changes = tracker.get_unsynced_changes().await?;
        let mut has_conflict = false;

        for existing in existing_changes {
            if existing.table_name == change_req.table_name 
                && existing.record_id == change_req.record_id 
                && existing.hash != change_req.hash {
                
                // Create a temporary model for conflict detection
                let remote_change = sync_log::Model {
                    id: 0,
                    table_name: change_req.table_name.clone(),
                    record_id: change_req.record_id.clone(),
                    operation: change_req.operation.clone(),
                    change_data: change_req.data.clone(),
                    device_id: req.device_id.clone(),
                    version: change_req.version,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&change_req.timestamp)
                        .map_err(|e| SyncError::DatabaseError(e.to_string()))?
                        .with_timezone(&chrono::Utc),
                    hash: change_req.hash.clone(),
                    is_synced: false,
                };

                if resolver.detect_conflict(&existing, &remote_change).await? {
                    let conflict = resolver.log_conflict(&existing, &remote_change).await?;
                    conflicts.push(ConflictInfo {
                        table_name: conflict.table_name,
                        record_id: conflict.record_id,
                        conflict_id: conflict.id,
                    });
                    has_conflict = true;
                    break;
                }
            }
        }

        if !has_conflict {
            tracker.log_change(
                &change_req.table_name,
                &change_req.record_id,
                operation,
                change_req.data,
            ).await?;
            accepted += 1;
        }
    }

    // Update last sync time
    manager.update_last_sync(&req.device_id).await?;

    Ok(Json(PushChangesResponse {
        accepted,
        conflicts,
    }))
}

/// Pull changes for a device
async fn pull_changes(
    State(state): State<ApiState>,
    Json(req): Json<PullChangesRequest>,
) -> Result<Json<PullChangesResponse>, SyncError> {
    use sea_orm::{QueryFilter, ColumnTrait, EntityTrait};

    // Verify device is authorized
    let manager = DeviceManager::new(state.db.clone());
    if !manager.is_authorized(&req.device_id).await? {
        return Err(SyncError::AuthenticationError(
            "Device not authorized".to_string()
        ));
    }

    // Get changes since the specified version
    let mut query = SyncLogEntity::find()
        .filter(sync_log::Column::DeviceId.ne(&req.device_id)); // Don't return device's own changes

    if let Some(since) = req.since_version {
        query = query.filter(sync_log::Column::Version.gt(since));
    }

    let changes = query.all(&state.db).await?;

    // Get pending conflicts
    let resolver = ConflictResolver::new(state.db.clone(), ConflictResolution::LatestTimestampWins);
    let conflicts = resolver.get_pending_conflicts().await?;

    // Update last sync time
    manager.update_last_sync(&req.device_id).await?;

    Ok(Json(PullChangesResponse {
        changes,
        conflicts,
    }))
}

/// Get sync status
async fn sync_status(
    State(state): State<ApiState>,
) -> Result<Json<serde_json::Value>, SyncError> {
    use sea_orm::{EntityTrait, PaginatorTrait};

    let tracker = ChangeTracker::new(state.db.clone(), state.device_id.clone());
    let unsynced = tracker.get_unsynced_changes().await?;
    
    let total_changes: u64 = SyncLogEntity::find().count(&state.db).await?;
    let total_devices: u64 = SyncDevicesEntity::find().count(&state.db).await?;
    
    let resolver = ConflictResolver::new(state.db.clone(), ConflictResolution::LatestTimestampWins);
    let pending_conflicts = resolver.get_pending_conflicts().await?;

    Ok(Json(serde_json::json!({
        "device_id": state.device_id,
        "total_changes": total_changes,
        "unsynced_changes": unsynced.len(),
        "pending_conflicts": pending_conflicts.len(),
        "total_devices": total_devices,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// List all conflicts
async fn list_conflicts(
    State(state): State<ApiState>,
) -> Result<Json<Vec<sync_conflicts::Model>>, SyncError> {
    let resolver = ConflictResolver::new(state.db, ConflictResolution::LatestTimestampWins);
    let conflicts = resolver.get_pending_conflicts().await?;
    Ok(Json(conflicts))
}

/// Resolve a specific conflict
async fn resolve_conflict(
    State(state): State<ApiState>,
    Path(id): Path<i32>,
    Json(req): Json<ResolveConflictRequest>,
) -> Result<StatusCode, SyncError> {
    let resolver = ConflictResolver::new(state.db, ConflictResolution::Manual);
    
    let winner = match req.winner.to_lowercase().as_str() {
        "local" => Winner::Local,
        "remote" => Winner::Remote,
        _ => return Err(SyncError::ConflictError("Invalid winner".to_string())),
    };

    resolver.manual_resolve(id, winner, req.custom_data).await?;
    Ok(StatusCode::OK)
}

/// Ignore a conflict
async fn ignore_conflict(
    State(state): State<ApiState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, SyncError> {
    let resolver = ConflictResolver::new(state.db, ConflictResolution::Manual);
    resolver.ignore_conflict(id).await?;
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    // Add tests for your API endpoints
}
