//! Note repository - handles note CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait, QueryFilter, QueryOrder, ColumnTrait};
use shared::models::*;
use database::entities::note;
use crate::converters::*;

pub struct NoteRepository;

impl NoteRepository {
    pub async fn create(db: &DatabaseConnection, note_data: CreateNoteRequest) -> Result<Note, Box<dyn std::error::Error>> {
        let active_model = note::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            client_id: Set(note_data.client_id),
            user_id: Set(note_data.user_id),
            content: Set(note_data.content),
            created_at: Set(chrono::Utc::now()),
        };
        let inserted = active_model.insert(db).await?;
        Ok(note_to_domain(&inserted))
    }

    pub async fn get_by_client_id(db: &DatabaseConnection, client_id: i32) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let entities = note::Entity::find()
            .filter(note::Column::ClientId.eq(client_id))
            .order_by_desc(note::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(note_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Note>, Box<dyn std::error::Error>> {
        let entity = note::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(note_to_domain))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        note::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn count_by_client(db: &DatabaseConnection, client_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let count = note::Entity::find()
            .filter(note::Column::ClientId.eq(client_id))
            .count(db)
            .await?;
        Ok(count as i64)
    }
}