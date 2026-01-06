// client/src/lib.rs
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, NotSet};
use shared::models::*;
use database::entities::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

mod income;

#[derive(Clone)]
pub struct Client {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = database::establish_connection().await?;
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
        })
    }

    // Example function for loan information
    pub async fn save_loan_information(&self, info: LoanInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = loan_information::ActiveModel {
            id: Set(info.id),
            property_type: Set(info.property_type.to_string()),
            occupancy_type: Set(info.occupancy_type.to_string()),
            loan_type: Set(info.loan_type.to_string()),
            new_term_months: Set(info.new_term_months),
            loan_purpose: Set(info.loan_purpose.to_string()),
            appraisal_waiver: Set(info.appraisal_waiver),
            created_at: Set(info.created_at),
            updated_at: Set(info.updated_at),
        };
        active_model.insert(&*db).await?;
        Ok(())
    }

    pub async fn get_loan_information(&self, id: Uuid) -> Result<Option<LoanInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entity = loan_information::Entity::find_by_id(id).one(&*db).await?;
        match entity {
            Some(model) => {
                let info = LoanInformation {
                    id: model.id,
                    property_type: parse_property_type(&model.property_type),
                    occupancy_type: parse_occupancy_type(&model.occupancy_type),
                    loan_type: parse_loan_type(&model.loan_type),
                    new_term_months: model.new_term_months,
                    loan_purpose: parse_loan_purpose(&model.loan_purpose),
                    appraisal_waiver: model.appraisal_waiver,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                };
                Ok(Some(info))
            }
            None => Ok(None),
        }
    }

    // Add more functions for other models as needed
    pub async fn save_borrower(&self, borrower: Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = borrower::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(borrower.name),
            employer_name: Set(borrower.employer_name),
            income_type: Set(borrower.income_type),
            loan_number: Set(borrower.loan_number),
            created_at: Set(borrower.created_at),
            updated_at: Set(borrower.updated_at),
        };
        active_model.insert(&*db).await?;
        Ok(())
    }

    pub async fn get_borrower(&self, id: i32) -> Result<Option<Borrower>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entity = borrower::Entity::find_by_id(id).one(&*db).await?;
        match entity {
            Some(model) => {
                let borrower = Borrower {
                    id: model.id,
                    name: model.name,
                    employer_name: model.employer_name,
                    income_type: model.income_type,
                    loan_number: model.loan_number,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                };
                Ok(Some(borrower))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_borrowers(&self) -> Result<Vec<Borrower>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entities = borrower::Entity::find().all(&*db).await?;
        let borrowers = entities.into_iter().map(|model| Borrower {
            id: model.id,
            name: model.name,
            employer_name: model.employer_name,
            income_type: model.income_type,
            loan_number: model.loan_number,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }).collect();
        Ok(borrowers)
    }

    // Add more functions for other models as needed
}

fn parse_property_type(s: &str) -> PropertyType {
    match s {
        "SFR" => PropertyType::SFR,
        "Manufactured" => PropertyType::Manufactured,
        "MultiUnit" => PropertyType::MultiUnit,
        "Condo" => PropertyType::Condo,
        "PUD" => PropertyType::PUD,
        _ => PropertyType::SFR,
    }
}

fn parse_occupancy_type(s: &str) -> OccupancyType {
    match s {
        "Primary" => OccupancyType::Primary,
        "Secondary" => OccupancyType::Secondary,
        "Investment" => OccupancyType::Investment,
        _ => OccupancyType::Primary,
    }
}

fn parse_loan_type(s: &str) -> LoanType {
    match s {
        "CNV" => LoanType::CNV,
        "FHA" => LoanType::FHA,
        "VA" => LoanType::VA,
        "NonQM" => LoanType::NonQM,
        _ => LoanType::CNV,
    }
}

fn parse_loan_purpose(s: &str) -> LoanPurpose {
    match s {
        "Purchase" => LoanPurpose::Purchase,
        "CashOut" => LoanPurpose::CashOut,
        "Refinance" => LoanPurpose::Refinance,
        "IRRRLStreamline" => LoanPurpose::IRRRLStreamline,
        _ => LoanPurpose::Refinance,
    }
}