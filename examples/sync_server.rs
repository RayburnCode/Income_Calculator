// Example: Simple sync server
// Run this on each device that needs to participate in P2P sync

use database::{establish_connection, local::{SyncManager, crypto}};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Income Calculator P2P Sync Server");
    println!("====================================\n");
    
    // Step 1: Connect to database
    println!("📊 Connecting to database...");
    let db = establish_connection().await?;
    println!("✅ Database connected\n");
    
    // Step 2: Get or create device ID
    let device_id = match std::env::var("DEVICE_ID") {
        Ok(id) => {
            println!("📱 Using existing device ID: {}", id);
            id
        }
        Err(_) => {
            println!("📱 Generating new device ID...");
            let keypair = crypto::generate_keypair();
            let public_key = crypto::export_public_key(&keypair);
            let secret_key = crypto::export_secret_key(&keypair);
            
            let id = uuid::Uuid::new_v4().to_string();
            
            println!("✅ Device ID: {}", id);
            println!("🔑 Public Key: {}", public_key);
            println!("🔐 Secret Key: {} (SAVE THIS SECURELY!)", secret_key);
            println!("\n⚠️  Set DEVICE_ID environment variable:");
            println!("   export DEVICE_ID={}\n", id);
            
            id
        }
    };
    
    // Step 3: Get Tailscale IP
    let tailscale_ip = get_tailscale_ip().await.unwrap_or_else(|_| {
        println!("⚠️  Could not detect Tailscale IP. Using localhost.");
        "127.0.0.1".to_string()
    });
    
    let port = std::env::var("SYNC_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr: SocketAddr = format!("{}:{}", tailscale_ip, port).parse()?;
    
    println!("🌐 Tailscale IP: {}", tailscale_ip);
    println!("🔌 Binding to: {}\n", bind_addr);
    
    // Step 4: Start sync server
    let mut manager = SyncManager::new(db, device_id.clone());
    manager.start_server(bind_addr).await?;
    
    println!("✅ Sync server is running!");
    println!("\n📋 Next steps:");
    println!("   1. Start this server on another device");
    println!("   2. Authorize the device: curl -X POST http://{}:{}/devices/authorize -H 'Content-Type: application/json' -d '{{\"device_id\": \"OTHER_DEVICE_ID\"}}'", tailscale_ip, port);
    println!("   3. Sync: curl http://{}:{}/sync/status\n", tailscale_ip, port);
    
    println!("Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await?;
    
    println!("\n👋 Shutting down...");
    Ok(())
}

async fn get_tailscale_ip() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    let output = Command::new("tailscale")
        .arg("ip")
        .arg("-4")
        .output()?;
    
    if output.status.success() {
        let ip = String::from_utf8(output.stdout)?
            .trim()
            .to_string();
        Ok(ip)
    } else {
        Err("Tailscale not running or not installed".into())
    }
}
