// database/src/lib.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
pub mod entities;
pub mod local;
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

/// Get the database URL - for SQLite, we use file-based storage in the user's app data directory
pub fn get_database_url() -> Result<String, DatabaseError> {
    // Check if DATABASE_URL environment variable is set (useful for development/testing)
    if let Ok(url) = std::env::var("DATABASE_URL") {
        return Ok(url);
    }

    // Use platform-specific application data directory
    // macOS: ~/Library/Application Support/income_calculator_data/
    // Windows: C:\Users\<Username>\AppData\Roaming\income_calculator_data\
    // Linux: ~/.local/share/income_calculator_data/
    let proj_dirs = directories::ProjectDirs::from("", "", "income_calculator_data")
        .ok_or_else(|| DatabaseError::PathError(
            "Could not determine application data directory. This might be a system configuration issue.".to_string()
        ))?;
    
    let data_dir = proj_dirs.data_dir();
    
    // Check if the data directory exists and is not a directory (e.g., it's a file)
    if data_dir.exists() && !data_dir.is_dir() {
        return Err(DatabaseError::PathError(format!(
            "Database directory path '{}' exists but is not a directory. Please remove or rename the conflicting file.",
            data_dir.display()
        )));
    }
    
    // Ensure the directory exists
    std::fs::create_dir_all(data_dir)
        .map_err(|e| DatabaseError::PathError(format!(
            "Failed to create database directory '{}': {}", 
            data_dir.display(), e
        )))?;
    
    let db_path = data_dir.join("income_calculator.db");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    
    Ok(url)
}

/// Get the database file path (useful for backups)
pub fn get_database_path() -> Result<std::path::PathBuf, DatabaseError> {
    let proj_dirs = directories::ProjectDirs::from("", "", "income_calculator_data")
        .ok_or_else(|| DatabaseError::PathError(
            "Could not determine application data directory.".to_string()
        ))?;
    
    Ok(proj_dirs.data_dir().join("income_calculator.db"))
}
