// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
pub mod entities;

pub use migration::Migrator;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    MigrationFailed(String),
    PathError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Failed to connect to database: {}", msg),
            DatabaseError::MigrationFailed(msg) => write!(f, "Failed to run database migrations: {}", msg),
            DatabaseError::PathError(msg) => write!(f, "Database path error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        DatabaseError::ConnectionFailed(err.to_string())
    }
}

pub async fn establish_connection() -> Result<DatabaseConnection, DatabaseError> {
    // Get database URL
    let url = get_database_url().map_err(|e| e)?;

    // Attempt to connect to database
    let conn = Database::connect(&url)
        .await
        .map_err(|e| {
            DatabaseError::ConnectionFailed(format!(
                "Could not connect to database at '{}'. Please check that the database file exists and is accessible. Error: {}",
                url, e
            ))
        })?;

    // Run migrations
    Migrator::up(&conn, None)
        .await
        .map_err(|e| DatabaseError::MigrationFailed(format!(
            "Database migration failed. This might indicate a database schema issue. Error: {}",
            e
        )))?;

    Ok(conn)
}

/// Get the database URL - for SQLite, we use file-based
pub fn get_database_url() -> Result<String, DatabaseError> {
    // Check if DATABASE_URL environment variable is set
    if let Ok(url) = std::env::var("DATABASE_URL") {
        return Ok(url);
    }

    // Use absolute path to the database in the project root
    let db_path = std::path::PathBuf::from("/Users/DylanRayburn/Documents/GitHub/Income_Calculator/income_calculator.db");
    
    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| DatabaseError::PathError(format!("Failed to create database directory '{}': {}", parent.display(), e)))?;
    }

    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    Ok(url)
}