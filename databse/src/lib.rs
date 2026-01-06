// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
pub mod entities;
 
pub use migration::Migrator;
 
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // Use file-based SQLite database relative to the executable
    let exe_dir = std::env::current_exe()
        .map_err(|e| DbErr::Custom(format!("Failed to get executable path: {}", e)))?
        .parent()
        .ok_or_else(|| DbErr::Custom("Failed to get executable directory".to_string()))?
        .to_path_buf();
    let db_path = exe_dir.join("income_calculator.db");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    let conn = Database::connect(&url).await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}

/// Get the database URL - for SQLite, we use file-based
pub fn get_database_url() -> String {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let db_path = exe_dir.join("income_calculator.db");
    format!("sqlite://{}?mode=rwc", db_path.display())
}