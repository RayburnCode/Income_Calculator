//! Timeline Events repository - handles timeline event CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait, QueryOrder, QueryFilter, ColumnTrait};
use shared::models::*;
use database::entities::timeline_events;
use crate::converters::*;

pub struct TimelineEventsRepository;

impl TimelineEventsRepository {
    pub async fn save(db: &DatabaseConnection, event: TimelineEvent) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = timeline_events::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            borrower_id: Set(event.borrower_id),
            event_type: Set(serde_json::to_string(&event.event_type)?),
            title: Set(event.title),
            description: Set(event.description),
            metadata: Set(event.metadata),
            user_id: Set(event.user_id),
            created_at: Set(event.created_at),
        };
        active_model.insert(db).await?;
        Ok(())
    }

    pub async fn get_by_borrower_id(db: &DatabaseConnection, borrower_id: i32) -> Result<Vec<TimelineEvent>, Box<dyn std::error::Error>> {
        let entities = timeline_events::Entity::find()
            .filter(timeline_events::Column::BorrowerId.eq(borrower_id))
            .order_by_desc(timeline_events::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(timeline_event_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<TimelineEvent>, Box<dyn std::error::Error>> {
        let entity = timeline_events::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(timeline_event_to_domain))
    }

    pub async fn update(db: &DatabaseConnection, event: TimelineEvent) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = timeline_events::ActiveModel {
            id: Set(event.id),
            borrower_id: Set(event.borrower_id),
            event_type: Set(serde_json::to_string(&event.event_type)?),
            title: Set(event.title),
            description: Set(event.description),
            metadata: Set(event.metadata),
            user_id: Set(event.user_id),
            created_at: Set(event.created_at),
        };
        active_model.update(db).await?;
        Ok(())
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        timeline_events::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn count_by_borrower(db: &DatabaseConnection, borrower_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let count = timeline_events::Entity::find()
            .filter(timeline_events::Column::BorrowerId.eq(borrower_id))
            .count(db)
            .await?;
        Ok(count as i64)
    }
}