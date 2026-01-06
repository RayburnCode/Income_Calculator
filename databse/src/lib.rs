// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
pub mod entities;
 
pub use migration::Migrator;
 
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // Use file-based SQLite database
    let conn = Database::connect("sqlite://../income_calculator.db?mode=rwc").await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}

/// Get the database URL - for SQLite, we use file-based
pub fn get_database_url() -> String {
    "sqlite://../income_calculator.db?mode=rwc".to_string()
}