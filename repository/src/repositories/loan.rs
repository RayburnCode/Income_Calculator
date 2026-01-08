//! Loan repository - handles loan-related CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use shared::models::*;
use database::entities::loan_information;
use uuid::Uuid;
use crate::converters::*;

pub struct LoanRepository;

impl LoanRepository {
    pub async fn get_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<LoanInformation>, Box<dyn std::error::Error>> {
        let entity = loan_information::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(loan_information_to_domain))
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<LoanInformation>, Box<dyn std::error::Error>> {
        let entities = loan_information::Entity::find().all(db).await?;
        Ok(entities.iter().map(loan_information_to_domain).collect())
    }

    pub async fn count(db: &DatabaseConnection) -> Result<i64, Box<dyn std::error::Error>> {
        let count = loan_information::Entity::find().count(db).await?;
        Ok(count as i64)
    }
}
