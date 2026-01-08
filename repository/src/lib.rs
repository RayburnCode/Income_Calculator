//! Repository layer for database access
//! 
//! This module provides a clean abstraction over database operations,
//! converting between domain models (in `shared`) and database entities (in `database`).

use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

mod converters;
mod repositories;

// Re-export for convenience
pub use converters::*;
pub use repositories::*;

/// Main repository client providing access to all data operations
#[derive(Clone)]
pub struct Repository {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl Repository {
    /// Create a new repository instance with database connection
    pub async fn new() -> Result<Self, String> {
        match database::establish_connection().await {
            Ok(conn) => Ok(Self {
                db: Arc::new(Mutex::new(conn)),
            }),
            Err(database::DatabaseError::ConnectionFailed(msg)) => {
                Err(format!("Failed to connect to database. Please check the database setup.\nDetails: {}", msg))
            },
            Err(database::DatabaseError::MigrationFailed(msg)) => {
                Err(format!("Database migration failed. The database may be corrupted or have an incompatible schema.\nDetails: {}", msg))
            },
            Err(database::DatabaseError::PathError(msg)) => {
                Err(format!("Database path error. Please check file permissions and directory structure.\nDetails: {}", msg))
            },
        }
    }

    /// Get a reference to the database connection
    async fn db(&self) -> tokio::sync::MutexGuard<'_, DatabaseConnection> {
        self.db.lock().await
    }

    // ===== Borrower Operations =====

    pub async fn save_borrower(&self, borrower: shared::models::Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        borrower::BorrowerRepository::save(&db, borrower).await
    }

    pub async fn update_borrower(&self, borrower: shared::models::Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        borrower::BorrowerRepository::update(&db, borrower).await
    }

    pub async fn get_borrower(&self, id: i32) -> Result<Option<shared::models::Borrower>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        borrower::BorrowerRepository::get_by_id(&db, id).await
    }

    pub async fn get_all_borrowers(&self) -> Result<Vec<shared::models::Borrower>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        borrower::BorrowerRepository::get_all(&db).await
    }

    pub async fn get_total_clients_count(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let db = self.db().await;
        borrower::BorrowerRepository::count(&db).await
    }

    // ===== W2 Jobs Operations =====

    pub async fn get_w2_jobs_data(&self, borrower_id: i32) -> Result<Option<shared::models::W2JobsData>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        w2_jobs::W2JobsRepository::get_by_borrower(&db, borrower_id).await
    }

    pub async fn save_w2_jobs_data(&self, borrower_id: i32, w2_data: &shared::models::W2JobsData) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        w2_jobs::W2JobsRepository::save_for_borrower(&db, borrower_id, w2_data).await
    }

    // ===== Loan Operations =====

    pub async fn get_loan_information(&self, id: uuid::Uuid) -> Result<Option<shared::models::LoanInformation>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        loan::LoanRepository::get_by_id(&db, id).await
    }

    pub async fn get_all_loan_information(&self) -> Result<Vec<shared::models::LoanInformation>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        loan::LoanRepository::get_all(&db).await
    }

    pub async fn get_total_loans_count(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let db = self.db().await;
        loan::LoanRepository::count(&db).await
    }

    // ===== Settings Operations =====

    pub async fn get_settings(&self) -> Result<shared::models::AppSettings, Box<dyn std::error::Error>> {
        let db = self.db().await;
        settings::SettingsRepository::get(&db).await
    }

    pub async fn save_settings(&self, settings: shared::models::AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        settings::SettingsRepository::save(&db, settings).await
    }

    // ===== Income Operations =====
    // Delegated to income module
    pub async fn save_income_information(&self, income: shared::models::IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::save_income_information(&db, income).await
    }

    pub async fn get_income_information(&self, id: uuid::Uuid) -> Result<Option<shared::models::IncomeInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::get_income_information(&db, id).await
    }

    pub async fn get_all_income_information(&self) -> Result<Vec<shared::models::IncomeInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::get_all_income_information(&db).await
    }

    pub async fn update_income_information(&self, income: shared::models::IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::update_income_information(&db, income).await
    }

    pub async fn delete_income_information(&self, id: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::delete_income_information(&db, id).await
    }

    pub async fn get_total_income_sum(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::get_total_income_sum(&db).await
    }

    // ===== Options Template Operations =====
    // Delegated to options_template module (complex operations)
    pub async fn save_options_template(&self, template: shared::models::OptionsTemplateData, borrower_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::save_options_template(&db, template, borrower_id).await
    }

    pub async fn get_options_template(&self, id: uuid::Uuid) -> Result<Option<shared::models::OptionsTemplateData>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::get_options_template(&db, id).await
    }

    pub async fn get_all_mortgage_refinance_options(&self) -> Result<Vec<shared::models::MortgageRefinanceOptions>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::get_all_mortgage_refinance_options(&db).await
    }

    // Additional legacy methods can be added here as needed
}

// Maintain backward compatibility by re-exporting Repository as Client
pub type Client = Repository;
