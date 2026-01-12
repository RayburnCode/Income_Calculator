// Example: Track a database change
// This shows how to integrate change tracking into your app

use database::{
    establish_connection,
    local::{ChangeTracker, SyncOperation},
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Change Tracking Example");
    println!("==========================\n");
    
    let device_id = std::env::var("DEVICE_ID")
        .unwrap_or_else(|_| "example-device".to_string());
    
    let db = establish_connection().await?;
    let tracker = ChangeTracker::new(db, device_id.clone());
    
    // Example: Track a loan update
    println!("Creating a sample change...");
    
    let loan_id = "loan-12345";
    let change_data = json!({
        "loan_amount": 350000,
        "interest_rate": 6.75,
        "property_value": 450000,
        "loan_type": "Conventional",
        "updated_at": chrono::Utc::now().to_rfc3339(),
    });
    
    let change = tracker.log_change(
        "loan_information",
        loan_id,
        SyncOperation::Update,
        change_data,
    ).await?;
    
    println!("✅ Change logged:");
    println!("   ID: {}", change.id);
    println!("   Table: {}", change.table_name);
    println!("   Record: {}", change.record_id);
    println!("   Operation: {}", change.operation);
    println!("   Version: {}", change.version);
    println!("   Hash: {}", change.hash);
    println!("   Synced: {}", change.is_synced);
    
    // Get all unsynced changes
    println!("\n📊 Checking unsynced changes...");
    let unsynced = tracker.get_unsynced_changes().await?;
    println!("   Unsynced changes: {}", unsynced.len());
    
    for change in unsynced.iter().take(5) {
        println!("   - {} {} (v{})", 
            change.operation, 
            change.record_id,
            change.version
        );
    }
    
    // Verify hash
    println!("\n🔐 Verifying integrity...");
    let is_valid = tracker.verify_hash(&change);
    println!("   Hash valid: {}", is_valid);
    
    Ok(())
}
