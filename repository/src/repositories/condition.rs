//! Condition repository - handles condition CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait, QueryFilter, QueryOrder, ColumnTrait};
use shared::models::*;
use database::entities::client::condition;
use crate::converters::*;

pub struct ConditionRepository;

impl ConditionRepository {
    pub async fn create(db: &DatabaseConnection, condition_data: CreateConditionRequest) -> Result<Condition, Box<dyn std::error::Error>> {
        let active_model = condition::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            client_id: Set(condition_data.client_id),
            title: Set(condition_data.title),
            description: Set(condition_data.description),
            condition_type: Set(format!("{:?}", condition_data.condition_type)),
            severity: Set(format!("{:?}", condition_data.severity)),
            status: Set(format!("{:?}", condition_data.status)),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };
        let inserted = active_model.insert(db).await?;
        Ok(condition_to_domain(&inserted))
    }

    pub async fn get_by_client_id(db: &DatabaseConnection, client_id: i32) -> Result<Vec<Condition>, Box<dyn std::error::Error>> {
        let entities = condition::Entity::find()
            .filter(condition::Column::ClientId.eq(client_id))
            .order_by_desc(condition::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(condition_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Condition>, Box<dyn std::error::Error>> {
        let entity = condition::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(condition_to_domain))
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: UpdateConditionRequest) -> Result<Option<Condition>, Box<dyn std::error::Error>> {
        let mut active_model: condition::ActiveModel = condition::Entity::find_by_id(id).one(db).await?.ok_or("Condition not found")?.into();

        if let Some(title) = update_data.title {
            active_model.title = Set(title);
        }
        if let Some(description) = update_data.description {
            active_model.description = Set(description);
        }
        if let Some(condition_type) = update_data.condition_type {
            active_model.condition_type = Set(format!("{:?}", condition_type));
        }
        if let Some(severity) = update_data.severity {
            active_model.severity = Set(format!("{:?}", severity));
        }
        if let Some(status) = update_data.status {
            active_model.status = Set(format!("{:?}", status));
        }
        active_model.updated_at = Set(chrono::Utc::now());

        let updated = active_model.update(db).await?;
        Ok(Some(condition_to_domain(&updated)))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        condition::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn count_by_client(db: &DatabaseConnection, client_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let count = condition::Entity::find()
            .filter(condition::Column::ClientId.eq(client_id))
            .count(db)
            .await?;
        Ok(count as i64)
    }
}