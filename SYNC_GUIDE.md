<!-- @format -->

# Secure P2P Database Synchronization over Tailscale

## Overview

This implementation provides a secure, production-ready peer-to-peer database synchronization system designed to work over Tailscale VPN. It includes:

- ✅ **Authentication & Authorization** - Device-based access control
- ✅ **Change Tracking** - Complete audit trail of all modifications
- ✅ **Conflict Resolution** - Automatic and manual conflict handling
- ✅ **Encryption** - SHA256 integrity hashing + Tailscale WireGuard encryption
- ✅ **REST API** - Cross-platform compatible sync protocol
- ✅ **Versioning** - Vector clock-like versioning for distributed sync

## Architecture

```
┌─────────────┐         Tailscale VPN          ┌─────────────┐
│  Device A   │◄──────────────────────────────►│  Device B   │
│             │    Encrypted Connection        │             │
│  SQLite DB  │                                │  SQLite DB  │
│  + Sync API │                                │  + Sync API │
└─────────────┘                                └─────────────┘
       │                                              │
       │         Push/Pull Changes                    │
       │         Conflict Detection                   │
       │         Authorization Check                  │
       └──────────────────┬───────────────────────────┘
                          │
                   Merge or Conflict
```

## Security Features

### 1. Network Layer (Tailscale)

- WireGuard encryption for all traffic
- Zero-trust networking
- Automatic key rotation
- NAT traversal

### 2. Application Layer (This Implementation)

- Device registration and authorization
- Public key authentication (Ed25519)
- SHA256 integrity hashing for all changes
- Device-level access control

### 3. Data Integrity

- Hash verification for all synchronized data
- Version tracking to detect tampering
- Immutable change log

## Quick Start

### 1. Setup Tailscale

First, install and configure Tailscale on all devices:

```bash
# Install Tailscale (macOS)
brew install tailscale

# Start Tailscale
sudo tailscale up

# Get your Tailscale IP
tailscale ip -4
# Example output: 100.64.0.5
```

### 2. Run Database Migrations

The sync tables will be created automatically when you run migrations:

```bash
cd database
cargo run --bin migration
```

This creates three new tables:

- `sync_log` - Tracks all database changes
- `sync_devices` - Manages authorized devices
- `sync_conflicts` - Handles merge conflicts

### 3. Initialize Your Device

```rust
use database::local::{SyncManager, crypto};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to database
    let db = database::establish_connection().await?;

    // Generate device keys
    let keypair = crypto::generate_keypair();
    let public_key = crypto::export_public_key(&keypair);

    // Create sync manager
    let mut manager = SyncManager::new(db, "device-id".to_string());

    // Initialize this device
    let device_id = manager.initialize_device(
        "MacBook Pro",
        &public_key,
        Some("100.64.0.5".to_string()),  // Your Tailscale IP
    ).await?;

    println!("Device ID: {}", device_id);
    // Save device_id and keypair securely!

    Ok(())
}
```

### 4. Start Sync Server

On each device that will accept sync connections:

```rust
use database::local::SyncManager;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = database::establish_connection().await?;
    let device_id = "your-device-id".to_string();

    let mut manager = SyncManager::new(db, device_id);

    // Bind to Tailscale IP
    let addr: SocketAddr = "100.64.0.5:8080".parse()?;
    manager.start_server(addr).await?;

    println!("🚀 Sync server running on {}", addr);

    // Keep running
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

### 5. Authorize Devices

Before devices can sync, they must be authorized:

```bash
# Using curl or your API client
curl -X POST http://100.64.0.5:8080/devices/authorize \
  -H "Content-Type: application/json" \
  -d '{"device_id": "device-id-to-authorize"}'
```

Or programmatically:

```rust
use database::local::DeviceManager;

let manager = DeviceManager::new(db);
manager.authorize_device("device-id-to-authorize").await?;
```

### 6. Sync Between Devices

```rust
use database::local::SyncManager;

let manager = SyncManager::new(db, your_device_id);

// One-time sync
let report = manager.sync_with_peer(
    "http://100.64.0.5:8080",  // Peer's Tailscale address
    &your_device_id,
).await?;

println!("Synced: pushed={}, pulled={}, conflicts={}",
    report.pushed, report.pulled, report.conflicts);

// Or enable automatic syncing every 5 minutes
let handle = manager.enable_auto_sync(
    "http://100.64.0.5:8080".to_string(),
    your_device_id.to_string(),
    300,  // 5 minutes
).await;
```

## Usage Examples

### Track Changes

Every database modification should be logged:

```rust
use database::local::{ChangeTracker, SyncOperation};

let tracker = ChangeTracker::new(db.clone(), device_id.clone());

// After inserting/updating/deleting a record
tracker.log_change(
    "loan_information",           // table name
    &loan_id.to_string(),         // record ID
    SyncOperation::Update,        // operation
    serde_json::json!({           // the data
        "loan_amount": 250000,
        "interest_rate": 6.5,
    }),
).await?;
```

### Handle Conflicts

When conflicts occur, you can resolve them automatically or manually:

```rust
use database::local::{ConflictResolver, ConflictResolution, Winner};

// Automatic resolution (latest timestamp wins)
let resolver = ConflictResolver::new(
    db.clone(),
    ConflictResolution::LatestTimestampWins
);

let conflicts = resolver.get_pending_conflicts().await?;
for conflict in conflicts {
    let resolution = resolver.resolve_conflict(&conflict).await?;
    println!("Resolved: {:?} wins", resolution.winner);
}

// Manual resolution
let resolver = ConflictResolver::new(db.clone(), ConflictResolution::Manual);
resolver.manual_resolve(
    conflict_id,
    Winner::Local,  // or Winner::Remote
    None,           // or Some(custom_merged_data)
).await?;
```

### List Connected Devices

```rust
use database::local::DeviceManager;

let manager = DeviceManager::new(db);
let devices = manager.get_authorized_devices().await?;

for device in devices {
    println!("{}: {} (last sync: {:?})",
        device.device_id,
        device.device_name,
        device.last_sync_at
    );
}
```

## REST API Reference

### Device Management

#### Register Device

```http
POST /devices/register
Content-Type: application/json

{
  "device_id": "uuid-here",
  "device_name": "iPhone 15",
  "public_key": "base64-encoded-key",
  "tailscale_ip": "100.64.0.10"
}
```

#### Authorize Device (Admin)

```http
POST /devices/authorize
Content-Type: application/json

{
  "device_id": "uuid-here"
}
```

#### List Devices

```http
GET /devices/list
```

#### Revoke Device

```http
POST /devices/{device_id}/revoke
```

### Synchronization

#### Push Changes

```http
POST /sync/push
Content-Type: application/json

{
  "device_id": "your-device-id",
  "changes": [
    {
      "table_name": "loan_information",
      "record_id": "123",
      "operation": "UPDATE",
      "data": { ... },
      "version": 45,
      "timestamp": "2026-01-12T10:30:00Z",
      "hash": "sha256-hash"
    }
  ]
}
```

#### Pull Changes

```http
POST /sync/pull
Content-Type: application/json

{
  "device_id": "your-device-id",
  "since_version": 40
}
```

#### Sync Status

```http
GET /sync/status
```

### Conflict Management

#### List Conflicts

```http
GET /conflicts/list
```

#### Resolve Conflict

```http
POST /conflicts/{conflict_id}/resolve
Content-Type: application/json

{
  "winner": "local",  // or "remote"
  "custom_data": null  // or merged data
}
```

#### Ignore Conflict

```http
POST /conflicts/{conflict_id}/ignore
```

## Configuration

### Conflict Resolution Strategies

Choose how conflicts are resolved:

```rust
use database::local::ConflictResolution;

// 1. Local always wins
ConflictResolution::LocalWins

// 2. Remote always wins
ConflictResolution::RemoteWins

// 3. Latest timestamp wins (recommended)
ConflictResolution::LatestTimestampWins

// 4. Manual resolution required
ConflictResolution::Manual
```

### Sync Intervals

```rust
// Sync every 5 minutes
manager.enable_auto_sync(peer_url, device_id, 300).await;

// Sync every hour
manager.enable_auto_sync(peer_url, device_id, 3600).await;

// Sync every 30 seconds (testing only)
manager.enable_auto_sync(peer_url, device_id, 30).await;
```

## Security Best Practices

### 1. Device Keys

- Store private keys securely (use OS keychain)
- Never commit keys to version control
- Rotate keys periodically

```rust
// macOS Keychain example
use keyring::Entry;

let entry = Entry::new("income_calculator", "device_key")?;
entry.set_password(&crypto::export_secret_key(&keypair))?;
```

### 2. Authorization

- Manually authorize each new device
- Regularly audit authorized devices
- Immediately revoke lost/stolen devices

### 3. Network

- Always use Tailscale (don't expose to public internet)
- Enable Tailscale ACLs for additional control
- Monitor Tailscale audit logs

### 4. Backups

- Regular database backups
- Test restoration process
- Keep backups encrypted

```rust
// Create encrypted backup
manager.export_backup("/secure/path/backup.db").await?;
```

## Troubleshooting

### Connection Issues

```bash
# Verify Tailscale connection
tailscale status

# Check if peer is reachable
ping 100.64.0.5

# Test sync server
curl http://100.64.0.5:8080/health
```

### Sync Failures

```rust
// Check sync status
let status = manager.sync_status().await?;
println!("{:?}", status);

// View unsynced changes
let tracker = ChangeTracker::new(db, device_id);
let unsynced = tracker.get_unsynced_changes().await?;
println!("Unsynced changes: {}", unsynced.len());
```

### Conflict Resolution

```rust
// List all conflicts
let resolver = ConflictResolver::new(db, strategy);
let conflicts = resolver.get_pending_conflicts().await?;

for conflict in conflicts {
    println!("Conflict in {}.{}: local v{} vs remote v{}",
        conflict.table_name,
        conflict.record_id,
        conflict.local_version,
        conflict.remote_version
    );
}
```

## Performance Optimization

### Batch Operations

```rust
// Batch multiple changes
let changes = vec![
    // ... multiple changes
];

// Push all at once
manager.sync_with_peer(peer_url, device_id).await?;
```

### Indexes

The migration creates indexes on:

- `sync_log(table_name, record_id)`
- `sync_log(is_synced)`
- `sync_log(timestamp)`

### Database Size

Monitor sync log growth:

```sql
-- Clean old synced changes (older than 90 days)
DELETE FROM sync_log
WHERE is_synced = 1
AND timestamp < datetime('now', '-90 days');
```

## Integration with Your App

### Example: Loan Application

```rust
use database::local::ChangeTracker;
use your_app::entities::loan_information;

async fn create_loan(
    db: &DatabaseConnection,
    tracker: &ChangeTracker,
    loan_data: LoanData,
) -> Result<loan_information::Model, Error> {
    // Create loan
    let loan = create_loan_in_db(db, loan_data).await?;

    // Track the change
    tracker.log_change(
        "loan_information",
        &loan.id.to_string(),
        SyncOperation::Insert,
        serde_json::to_value(&loan)?,
    ).await?;

    Ok(loan)
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_registration() {
        let db = setup_test_db().await;
        let manager = DeviceManager::new(db);

        let device = manager.register_device(
            "test-device",
            "Test Device",
            "public-key",
            None,
        ).await.unwrap();

        assert_eq!(device.device_id, "test-device");
        assert_eq!(device.is_authorized, false);
    }

    #[tokio::test]
    async fn test_sync_flow() {
        // Test complete sync workflow
    }
}
```

## License

This synchronization system is part of the Income Calculator project.

## Support

For issues or questions:

1. Check the troubleshooting section
2. Review Tailscale documentation
3. Open an issue on GitHub
