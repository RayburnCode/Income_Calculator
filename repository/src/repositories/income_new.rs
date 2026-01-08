use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait};
use shared::models::*;
use database::entities::income_information;
use uuid::Uuid;
use rust_decimal::{Decimal, prelude::ToPrimitive};

use crate::converters::*;

pub async fn save_income_information(db: &DatabaseConnection, income: IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
    let active_model = income_information::ActiveModel {
        id: Set(income.id),
        borrower_monthly_income: Set(Decimal::from_f64_retain(income.borrower_monthly_income).unwrap()),
        coborrower_monthly_income: Set(Decimal::from_f64_retain(income.coborrower_monthly_income).unwrap()),
        front_end_ratio: Set(Decimal::from_f64_retain(income.front_end_ratio).unwrap()),
        back_end_ratio: Set(Decimal::from_f64_retain(income.back_end_ratio).unwrap()),
        created_at: Set(income.created_at),
        updated_at: Set(income.updated_at),
    };
    active_model.insert(db).await?;
    Ok(())
}

pub async fn get_income_information(db: &DatabaseConnection, id: Uuid) -> Result<Option<IncomeInformation>, Box<dyn std::error::Error>> {
    let entity = income_information::Entity::find_by_id(id).one(db).await?;
    match entity {
        Some(model) => Ok(Some(income_information_to_domain(&model))),
        None => Ok(None),
    }
}

pub async fn get_all_income_information(db: &DatabaseConnection) -> Result<Vec<IncomeInformation>, Box<dyn std::error::Error>> {
    let entities = income_information::Entity::find().all(db).await?;
    Ok(entities.iter().map(income_information_to_domain).collect())
}

pub async fn update_income_information(db: &DatabaseConnection, income: IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
    let active_model = income_information::ActiveModel {
        id: Set(income.id),
        borrower_monthly_income: Set(Decimal::from_f64_retain(income.borrower_monthly_income).unwrap()),
        coborrower_monthly_income: Set(Decimal::from_f64_retain(income.coborrower_monthly_income).unwrap()),
        front_end_ratio: Set(Decimal::from_f64_retain(income.front_end_ratio).unwrap()),
        back_end_ratio: Set(Decimal::from_f64_retain(income.back_end_ratio).unwrap()),
        created_at: Set(income.created_at),
        updated_at: Set(income.updated_at),
    };
    active_model.update(db).await?;
    Ok(())
}

pub async fn delete_income_information(db: &DatabaseConnection, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
    income_information::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn get_total_income_sum(db: &DatabaseConnection) -> Result<f64, Box<dyn std::error::Error>> {
    let incomes = income_information::Entity::find().all(db).await?;
    let total: f64 = incomes.iter()
        .map(|income| {
            income.borrower_monthly_income.to_f64().unwrap_or(0.0) +
            income.coborrower_monthly_income.to_f64().unwrap_or(0.0)
        })
        .sum();
    Ok(total)
}
