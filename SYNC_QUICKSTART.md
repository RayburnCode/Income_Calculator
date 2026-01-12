<!-- @format -->

# P2P Sync Quick Start

## 🚀 5-Minute Setup

### 1. Install Tailscale

```bash
# macOS
brew install tailscale
sudo tailscale up

# Get your IP
tailscale ip -4
# Example: 100.64.0.5
```

### 2. Run Migrations

```bash
cd database
cargo run --bin migration
```

### 3. Start Server (Device A)

```bash
export DEVICE_ID=$(uuidgen)
cargo run --example sync_server --package database

# Save the output Device ID and keys!
```

### 4. Start Server (Device B)

```bash
# On your second device
export DEVICE_ID=$(uuidgen)
cargo run --example sync_server --package database
```

### 5. Authorize Devices

```bash
# From Device A, authorize Device B
curl -X POST http://100.64.0.5:8080/devices/authorize \
  -H "Content-Type: application/json" \
  -d '{"device_id": "DEVICE_B_ID"}'

# From Device B, authorize Device A
curl -X POST http://100.64.0.6:8080/devices/authorize \
  -H "Content-Type: application/json" \
  -d '{"device_id": "DEVICE_A_ID"}'
```

### 6. Sync!

```bash
# From Device A, sync with Device B
cargo run --example sync_client --package database -- http://100.64.0.6:8080
```

## 📝 Track Changes in Your App

```rust
use database::local::{ChangeTracker, SyncOperation};

// After any database modification
let tracker = ChangeTracker::new(db.clone(), device_id);
tracker.log_change(
    "your_table",
    &record_id,
    SyncOperation::Update,
    serde_json::to_value(&data)?,
).await?;
```

## 🔄 Enable Auto-Sync

```rust
use database::local::SyncManager;

let manager = SyncManager::new(db, device_id);

// Sync every 5 minutes
manager.enable_auto_sync(
    "http://peer-tailscale-ip:8080".to_string(),
    device_id,
    300, // seconds
).await;
```

## 📚 Full Documentation

See [SYNC_GUIDE.md](../SYNC_GUIDE.md) for complete documentation.

## 🔧 Configuration

### Environment Variables

- `DEVICE_ID` - Your unique device identifier (generate with `uuidgen`)
- `SYNC_PORT` - Port for sync server (default: 8080)
- `DATABASE_URL` - Override database location (optional)

### Conflict Resolution

Choose your strategy in code:

- `LatestTimestampWins` - Newest change wins (recommended)
- `LocalWins` - Always prefer local
- `RemoteWins` - Always prefer remote
- `Manual` - Requires user intervention

## 🛡️ Security Checklist

- ✅ Tailscale installed and running
- ✅ Only authorized devices can sync
- ✅ Device keys stored securely (use OS keychain)
- ✅ Regular backups enabled
- ✅ Monitor sync logs

## 🐛 Troubleshooting

**Can't connect to peer?**

```bash
# Check Tailscale
tailscale status
ping <peer-ip>

# Test server
curl http://<peer-ip>:8080/health
```

**Conflicts?**

```bash
# List conflicts
curl http://<peer-ip>:8080/conflicts/list

# Resolve (choose local or remote)
curl -X POST http://<peer-ip>:8080/conflicts/<id>/resolve \
  -H "Content-Type: application/json" \
  -d '{"winner": "local"}'
```

**Check sync status:**

```bash
curl http://<peer-ip>:8080/sync/status
```
