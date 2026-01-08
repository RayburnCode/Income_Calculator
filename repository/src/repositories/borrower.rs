//! Borrower repository - handles borrower CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait};
use shared::models::*;
use database::entities::borrower;
use crate::converters::*;

pub struct BorrowerRepository;

impl BorrowerRepository {
    pub async fn save(db: &DatabaseConnection, borrower: Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = borrower::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(borrower.name),
            employer_name: Set(borrower.employer_name),
            income_type: Set(borrower.income_type),
            loan_number: Set(borrower.loan_number),
            status: Set(borrower.status.map(|s| s.to_string())),
            email: Set(borrower.email),
            phone_number: Set(borrower.phone_number),
            created_at: Set(borrower.created_at),
            updated_at: Set(borrower.updated_at),
        };
        active_model.insert(db).await?;
        Ok(())
    }

    pub async fn update(db: &DatabaseConnection, borrower: Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = borrower::ActiveModel {
            id: Set(borrower.id),
            name: Set(borrower.name),
            employer_name: Set(borrower.employer_name),
            income_type: Set(borrower.income_type),
            loan_number: Set(borrower.loan_number),
            status: Set(borrower.status.map(|s| s.to_string())),
            email: Set(borrower.email),
            phone_number: Set(borrower.phone_number),
            created_at: Set(borrower.created_at),
            updated_at: Set(borrower.updated_at),
        };
        active_model.update(db).await?;
        Ok(())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Borrower>, Box<dyn std::error::Error>> {
        let entity = borrower::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(borrower_to_domain))
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Borrower>, Box<dyn std::error::Error>> {
        let entities = borrower::Entity::find().all(db).await?;
        Ok(entities.iter().map(borrower_to_domain).collect())
    }

    pub async fn count(db: &DatabaseConnection) -> Result<i64, Box<dyn std::error::Error>> {
        let count = borrower::Entity::find().count(db).await?;
        Ok(count as i64)
    }
}
