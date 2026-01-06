// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
pub mod entities;
 
pub use migration::Migrator;
 
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // Use in-memory SQLite for embedded database
    let conn = Database::connect("sqlite::memory:").await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}

/// Get the database URL - for SQLite, we use in-memory
pub fn get_database_url() -> String {
    "sqlite::memory:".to_string()
}