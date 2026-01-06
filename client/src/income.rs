use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use shared::models::*;
use database::entities::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::Client;

impl Client {
    // Income Information CRUD operations
    pub async fn save_income_information(&self, income: IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = income_information::ActiveModel {
            id: Set(income.id),
            borrower_monthly_income: Set(Decimal::from_f64_retain(income.borrower_monthly_income)),
            coborrower_monthly_income: Set(Decimal::from_f64_retain(income.coborrower_monthly_income)),
            front_end_ratio: Set(Decimal::from_f64_retain(income.front_end_ratio)),
            back_end_ratio: Set(Decimal::from_f64_retain(income.back_end_ratio)),
            created_at: Set(income.created_at),
            updated_at: Set(income.updated_at),
        };
        active_model.insert(&*db).await?;
        Ok(())
    }

    pub async fn get_income_information(&self, id: Uuid) -> Result<Option<IncomeInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entity = income_information::Entity::find_by_id(id).one(&*db).await?;
        match entity {
            Some(model) => {
                let income = IncomeInformation {
                    id: model.id,
                    borrower_monthly_income: model.borrower_monthly_income.to_f64().unwrap_or(0.0),
                    coborrower_monthly_income: model.coborrower_monthly_income.to_f64().unwrap_or(0.0),
                    front_end_ratio: model.front_end_ratio.to_f64().unwrap_or(0.0),
                    back_end_ratio: model.back_end_ratio.to_f64().unwrap_or(0.0),
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                };
                Ok(Some(income))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_income_information(&self) -> Result<Vec<IncomeInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entities = income_information::Entity::find().all(&*db).await?;
        let incomes = entities.into_iter().map(|model| IncomeInformation {
            id: model.id,
            borrower_monthly_income: model.borrower_monthly_income.to_f64().unwrap_or(0.0),
            coborrower_monthly_income: model.coborrower_monthly_income.to_f64().unwrap_or(0.0),
            front_end_ratio: model.front_end_ratio.to_f64().unwrap_or(0.0),
            back_end_ratio: model.back_end_ratio.to_f64().unwrap_or(0.0),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }).collect();
        Ok(incomes)
    }

    pub async fn update_income_information(&self, income: IncomeInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = income_information::ActiveModel {
            id: Set(income.id),
            borrower_monthly_income: Set(Decimal::from_f64_retain(income.borrower_monthly_income)),
            coborrower_monthly_income: Set(Decimal::from_f64_retain(income.coborrower_monthly_income)),
            front_end_ratio: Set(Decimal::from_f64_retain(income.front_end_ratio)),
            back_end_ratio: Set(Decimal::from_f64_retain(income.back_end_ratio)),
            created_at: Set(income.created_at),
            updated_at: Set(income.updated_at),
        };
        active_model.update(&*db).await?;
        Ok(())
    }

    pub async fn delete_income_information(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        income_information::Entity::delete_by_id(id).exec(&*db).await?;
        Ok(())
    }
}