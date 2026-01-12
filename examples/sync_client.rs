// Example: Sync client that connects to a peer
// Run this to sync your local database with a remote peer

use database::{establish_connection, local::SyncManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Income Calculator Sync Client");
    println!("=================================\n");
    
    // Get configuration from environment or args
    let device_id = std::env::var("DEVICE_ID")
        .expect("DEVICE_ID environment variable not set");
    
    let peer_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Usage: cargo run --example sync_client <peer_url>");
            println!("Example: cargo run --example sync_client http://100.64.0.5:8080");
            std::process::exit(1);
        });
    
    println!("📱 Device ID: {}", device_id);
    println!("🌐 Peer URL: {}\n", peer_url);
    
    // Connect to database
    println!("📊 Connecting to database...");
    let db = establish_connection().await?;
    println!("✅ Database connected\n");
    
    // Create sync manager
    let manager = SyncManager::new(db, device_id.clone());
    
    // Perform sync
    println!("🔄 Syncing with peer...");
    match manager.sync_with_peer(&peer_url, &device_id).await {
        Ok(report) => {
            println!("✅ Sync complete!");
            println!("   📤 Pushed: {} changes", report.pushed);
            println!("   📥 Pulled: {} changes", report.pulled);
            
            if report.conflicts > 0 {
                println!("   ⚠️  Conflicts: {} (requires resolution)", report.conflicts);
                println!("\nResolve conflicts:");
                println!("   curl http://{}/conflicts/list", peer_url);
            } else {
                println!("   ✅ No conflicts");
            }
        }
        Err(e) => {
            eprintln!("❌ Sync failed: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("   1. Is the peer server running?");
            eprintln!("   2. Is this device authorized? Check with:");
            eprintln!("      curl {}/devices/list", peer_url);
            eprintln!("   3. Can you reach the peer?");
            eprintln!("      ping <peer_tailscale_ip>");
            std::process::exit(1);
        }
    }
    
    Ok(())
}
