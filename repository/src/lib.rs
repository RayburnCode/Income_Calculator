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

    pub async fn get_total_loans_count_in_date_range(&self, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<i64, Box<dyn std::error::Error>> {
        let db = self.db().await;
        loan::LoanRepository::count_in_date_range(&db, start_date, end_date).await
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

    pub async fn get_total_income_sum_in_date_range(&self, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<f64, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income::get_total_income_sum_in_date_range(&db, start_date, end_date).await
    }

    // ===== Options Template Operations =====
    // Delegated to options_template module (complex operations)
    pub async fn save_options_template(&self, template: shared::models::OptionsTemplateData, borrower_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::save_options_template(&db, template, borrower_id).await
    }

    pub async fn get_options_template(&self, borrower_id: i32) -> Result<Option<shared::models::OptionsTemplateData>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::get_options_template(&db, borrower_id).await
    }

    pub async fn get_all_mortgage_refinance_options(&self) -> Result<Vec<shared::models::MortgageRefinanceOptions>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        options_template::get_all_mortgage_refinance_options(&db).await
    }

    // ===== Timeline Events Operations =====
    pub async fn create_timeline_event(&self, event: shared::models::TimelineEvent) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::save(&db, event).await
    }

    pub async fn get_timeline_events(&self, borrower_id: i32) -> Result<Vec<shared::models::TimelineEvent>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::get_by_borrower_id(&db, borrower_id).await
    }

    pub async fn get_timeline_event(&self, id: i32) -> Result<Option<shared::models::TimelineEvent>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::get_by_id(&db, id).await
    }

    pub async fn update_timeline_event(&self, event: shared::models::TimelineEvent) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::update(&db, event).await
    }

    pub async fn delete_timeline_event(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::delete(&db, id).await
    }

    pub async fn get_timeline_events_count(&self, borrower_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let db = self.db().await;
        timeline_events::TimelineEventsRepository::count_by_borrower(&db, borrower_id).await
    }

    // ===== Outreach Templates Operations =====
    pub async fn save_outreach_template(&self, template: shared::models::OutreachTemplate) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::save(&db, template).await
    }

    pub async fn get_all_outreach_templates(&self) -> Result<Vec<shared::models::OutreachTemplate>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::get_all_active(&db).await
    }

    pub async fn get_outreach_templates_by_type(&self, template_type: shared::models::TemplateType) -> Result<Vec<shared::models::OutreachTemplate>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::get_by_type(&db, template_type).await
    }

    pub async fn get_outreach_template(&self, id: i32) -> Result<Option<shared::models::OutreachTemplate>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::get_by_id(&db, id).await
    }

    pub async fn update_outreach_template(&self, template: shared::models::OutreachTemplate) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::update(&db, template).await
    }

    pub async fn delete_outreach_template(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::delete(&db, id).await
    }

    pub async fn get_default_outreach_templates(&self) -> Result<Vec<shared::models::OutreachTemplate>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::get_default_templates(&db).await
    }

    pub async fn get_user_outreach_templates(&self, user_id: &str) -> Result<Vec<shared::models::OutreachTemplate>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        outreach_templates::OutreachTemplatesRepository::get_user_templates(&db, user_id).await
    }

    // ===== Campaign Operations =====
    pub async fn save_campaign(&self, campaign: shared::models::Campaign) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::save(&db, campaign).await
    }

    pub async fn get_all_campaigns(&self) -> Result<Vec<shared::models::Campaign>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::get_all(&db).await
    }

    pub async fn get_campaign(&self, id: i32) -> Result<Option<shared::models::Campaign>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::get_by_id(&db, id).await
    }

    pub async fn get_campaigns_by_status(&self, status: shared::models::CampaignStatus) -> Result<Vec<shared::models::Campaign>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::get_by_status(&db, status).await
    }

    pub async fn update_campaign(&self, campaign: shared::models::Campaign) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::update(&db, campaign).await
    }

    pub async fn delete_campaign(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::delete(&db, id).await
    }

    pub async fn update_campaign_analytics(&self, campaign_id: i32, sent: i32, opened: i32, clicked: i32, converted: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::CampaignsRepository::update_analytics(&db, campaign_id, sent, opened, clicked, converted).await
    }

    // ===== A/B Test Operations =====
    pub async fn save_ab_test(&self, ab_test: shared::models::ABTest) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::ABTestsRepository::save(&db, ab_test).await
    }

    pub async fn get_ab_tests_by_campaign(&self, campaign_id: i32) -> Result<Vec<shared::models::ABTest>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::ABTestsRepository::get_by_campaign_id(&db, campaign_id).await
    }

    pub async fn update_ab_test_results(&self, test_id: i32, sent_a: i32, sent_b: i32, opened_a: i32, opened_b: i32, clicked_a: i32, clicked_b: i32, winner: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        campaigns::ABTestsRepository::update_results(&db, test_id, sent_a, sent_b, opened_a, opened_b, clicked_a, clicked_b, winner).await
    }

    // ===== Note Operations =====

    pub async fn create_note(&self, note_request: shared::models::CreateNoteRequest) -> Result<shared::models::Note, Box<dyn std::error::Error>> {
        let db = self.db().await;
        note::NoteRepository::create(&db, note_request).await
    }

    pub async fn get_notes_by_client(&self, client_id: i32) -> Result<Vec<shared::models::Note>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        note::NoteRepository::get_by_client_id(&db, client_id).await
    }

    pub async fn delete_note(&self, note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        note::NoteRepository::delete(&db, note_id).await
    }

    // ===== Document Operations =====

    pub async fn upload_document(&self, upload_request: shared::models::UploadDocumentRequest) -> Result<shared::models::Document, Box<dyn std::error::Error>> {
        let db = self.db().await;
        document::DocumentRepository::create(&db, upload_request).await
    }

    pub async fn get_documents_by_client(&self, client_id: i32) -> Result<Vec<shared::models::Document>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        document::DocumentRepository::get_by_client_id(&db, client_id).await
    }

    pub async fn delete_document(&self, document_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        document::DocumentRepository::delete(&db, document_id).await
    }

    // ===== Condition Operations =====

    pub async fn create_condition(&self, condition_request: shared::models::CreateConditionRequest) -> Result<shared::models::Condition, Box<dyn std::error::Error>> {
        let db = self.db().await;
        condition::ConditionRepository::create(&db, condition_request).await
    }

    pub async fn get_conditions_by_client(&self, client_id: i32) -> Result<Vec<shared::models::Condition>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        condition::ConditionRepository::get_by_client_id(&db, client_id).await
    }

    pub async fn get_condition_by_id(&self, condition_id: i32) -> Result<Option<shared::models::Condition>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        condition::ConditionRepository::get_by_id(&db, condition_id).await
    }

    pub async fn update_condition(&self, condition_id: i32, update_request: shared::models::UpdateConditionRequest) -> Result<Option<shared::models::Condition>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        condition::ConditionRepository::update(&db, condition_id, update_request).await
    }

    pub async fn delete_condition(&self, condition_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db().await;
        condition::ConditionRepository::delete(&db, condition_id).await
    }

    pub async fn download_document(&self, document_id: i32) -> Result<Option<(shared::models::Document, Vec<u8>)>, Box<dyn std::error::Error>> {
        let db = self.db().await;
        document::DocumentRepository::get_file_data(&db, document_id).await
    }

    // Additional legacy methods can be added here as needed
}

// Maintain backward compatibility by re-exporting Repository as Client
pub type Client = Repository;
