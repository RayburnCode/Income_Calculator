<!-- @format -->

# P2P Sync Architecture

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Tailscale VPN                           │
│                  (WireGuard Encrypted Tunnel)                   │
│                                                                 │
│  ┌──────────────────┐                    ┌──────────────────┐  │
│  │   Device A       │                    │   Device B       │  │
│  │  (MacBook Pro)   │◄──────────────────►│  (iPhone)        │  │
│  │                  │   REST API Calls   │                  │  │
│  │  ┌────────────┐  │                    │  ┌────────────┐  │  │
│  │  │ Sync API   │  │                    │  │ Sync API   │  │  │
│  │  │ (Port 8080)│  │                    │  │ (Port 8080)│  │  │
│  │  └──────┬─────┘  │                    │  └──────┬─────┘  │  │
│  │         │        │                    │         │        │  │
│  │  ┌──────▼─────┐  │                    │  ┌──────▼─────┐  │  │
│  │  │ Change     │  │                    │  │ Change     │  │  │
│  │  │ Tracker    │  │                    │  │ Tracker    │  │  │
│  │  └──────┬─────┘  │                    │  └──────┬─────┘  │  │
│  │         │        │                    │         │        │  │
│  │  ┌──────▼─────┐  │                    │  ┌──────▼─────┐  │  │
│  │  │ SQLite DB  │  │                    │  │ SQLite DB  │  │  │
│  │  │ + Sync     │  │                    │  │ + Sync     │  │  │
│  │  │ Tables     │  │                    │  │ Tables     │  │  │
│  │  └────────────┘  │                    │  └────────────┘  │  │
│  │                  │                    │                  │  │
│  └──────────────────┘                    └──────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Database Schema

```
┌─────────────────────┐
│     sync_log        │  <- Tracks all changes
├─────────────────────┤
│ id                  │
│ table_name          │
│ record_id           │
│ operation           │  (INSERT/UPDATE/DELETE)
│ change_data (JSON)  │
│ device_id           │
│ version             │
│ timestamp           │
│ hash (SHA256)       │
│ is_synced           │
└─────────────────────┘

┌─────────────────────┐
│   sync_devices      │  <- Authorized devices
├─────────────────────┤
│ id                  │
│ device_id (unique)  │
│ device_name         │
│ public_key          │
│ tailscale_ip        │
│ is_authorized       │
│ last_sync_at        │
│ created_at          │
└─────────────────────┘

┌─────────────────────┐
│   sync_conflicts    │  <- Conflict resolution
├─────────────────────┤
│ id                  │
│ table_name          │
│ record_id           │
│ local_data (JSON)   │
│ remote_data (JSON)  │
│ local_version       │
│ remote_version      │
│ local_device_id     │
│ remote_device_id    │
│ status              │  (pending/resolved/ignored)
│ resolution (JSON)   │
│ created_at          │
│ resolved_at         │
└─────────────────────┘
```

## Sync Flow

```
Device A                           Device B
   │                                  │
   │  1. Make local changes           │
   │  2. Log to sync_log              │
   │                                  │
   │  3. Push changes ───────────────►│
   │     POST /sync/push              │
   │                                  │
   │                      4. Check authorization
   │                      5. Detect conflicts
   │                      6. Log to sync_log
   │                                  │
   │◄──────────── Acknowledgment ─────│
   │     (accepted: N, conflicts: M)  │
   │                                  │
   │  7. Pull changes ────────────────►│
   │     POST /sync/pull              │
   │                                  │
   │◄────── 8. Return changes ────────│
   │     (changes: [...])             │
   │                                  │
   │  9. Apply changes                │
   │ 10. Resolve conflicts            │
   │ 11. Mark as synced               │
   │                                  │
```

## Conflict Resolution Flow

```
Local Change                Remote Change
  version: 10                version: 10
  timestamp: 14:30:00        timestamp: 14:30:05
  data: {amount: 250k}       data: {amount: 300k}
         │                           │
         └─────────┬─────────────────┘
                   │
                   ▼
          Conflict Detected!
                   │
                   ▼
        ┌──────────────────────┐
        │  Resolution Strategy │
        └──────────────────────┘
                   │
         ┌─────────┼─────────┬─────────┐
         │         │         │         │
         ▼         ▼         ▼         ▼
    Local Wins  Remote   Timestamp  Manual
                 Wins      Wins    Resolution
         │         │         │         │
         └─────────┴─────────┴─────────┘
                   │
                   ▼
           Applied Change
                   │
                   ▼
        Update sync_conflicts
         status = "resolved"
```

## Security Layers

```
┌────────────────────────────────────────────┐
│  Application Layer                         │
│  • Device Authorization                    │
│  • Public Key Authentication               │
│  • SHA256 Hash Verification                │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│  Transport Layer (Tailscale/WireGuard)     │
│  • End-to-End Encryption                   │
│  • Perfect Forward Secrecy                 │
│  • Automatic Key Rotation                  │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│  Network Layer                             │
│  • Zero Trust Networking                   │
│  • NAT Traversal                           │
│  • Firewall Bypass                         │
└────────────────────────────────────────────┘
```

## Data Flow Timeline

```
T0: User edits loan on Device A
    ├─ Update loan_information table
    └─ Log change to sync_log (version 45)

T1: Auto-sync triggered (5 min later)
    ├─ Connect to Device B via Tailscale
    ├─ Authenticate using device_id
    └─ Push changes to Device B

T2: Device B receives changes
    ├─ Verify device authorization
    ├─ Check for conflicts
    ├─ No conflict: Apply change
    └─ Update last_sync_at

T3: Device A pulls changes
    ├─ Request changes since version 45
    ├─ Receive 3 new changes from Device B
    └─ Apply changes locally

T4: Both devices in sync
    ├─ All changes synced
    └─ No conflicts
```

## Component Diagram

```
┌─────────────────────────────────────────────────┐
│              Application Code                   │
│  (Create/Update/Delete records)                 │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│           ChangeTracker                         │
│  • log_change()                                 │
│  • get_unsynced_changes()                       │
│  • mark_as_synced()                             │
│  • verify_hash()                                │
└────────────────┬────────────────────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
        ▼                 ▼
┌──────────────┐  ┌──────────────────┐
│ SyncManager  │  │ ConflictResolver │
│ • sync_with  │  │ • detect()       │
│   _peer()    │  │ • resolve()      │
│ • auto_sync()│  │ • manual()       │
└──────┬───────┘  └────────┬─────────┘
       │                   │
       └──────────┬────────┘
                  │
                  ▼
        ┌─────────────────┐
        │   REST API      │
        │  /sync/push     │
        │  /sync/pull     │
        │  /conflicts/*   │
        └────────┬────────┘
                 │
                 ▼
        ┌─────────────────┐
        │   Database      │
        │  • sync_log     │
        │  • sync_devices │
        │  • sync_conflicts│
        └─────────────────┘
```

## Error Handling

```
Sync Request
     │
     ├─► Device Not Authorized
     │   └─► Return 401 Unauthorized
     │
     ├─► Network Error
     │   └─► Retry with exponential backoff
     │
     ├─► Conflict Detected
     │   ├─► Log to sync_conflicts
     │   ├─► Return conflict info
     │   └─► Await resolution
     │
     ├─► Hash Mismatch
     │   └─► Reject change, request re-push
     │
     └─► Success
         └─► Update sync_log, return OK
```
