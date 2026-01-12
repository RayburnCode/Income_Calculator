//! Outreach Templates repository - handles template CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryOrder, QueryFilter, ColumnTrait};
use shared::models::*;
use database::entities::outreach_templates;
use crate::converters::*;

pub struct OutreachTemplatesRepository;

impl OutreachTemplatesRepository {
    pub async fn save(db: &DatabaseConnection, template: OutreachTemplate) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = outreach_templates::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(template.name),
            template_type: Set(serde_json::to_string(&template.template_type)?),
            subject: Set(template.subject),
            content: Set(template.content),
            description: Set(template.description),
            is_default: Set(template.is_default),
            is_active: Set(template.is_active),
            created_by: Set(template.created_by),
            created_at: Set(template.created_at),
            updated_at: Set(template.updated_at),
        };
        active_model.insert(db).await?;
        Ok(())
    }

    pub async fn get_all_active(db: &DatabaseConnection) -> Result<Vec<OutreachTemplate>, Box<dyn std::error::Error>> {
        let entities = outreach_templates::Entity::find()
            .filter(outreach_templates::Column::IsActive.eq(true))
            .order_by_desc(outreach_templates::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(outreach_template_to_domain).collect())
    }

    pub async fn get_by_type(db: &DatabaseConnection, template_type: TemplateType) -> Result<Vec<OutreachTemplate>, Box<dyn std::error::Error>> {
        let entities = outreach_templates::Entity::find()
            .filter(outreach_templates::Column::TemplateType.eq(serde_json::to_string(&template_type)?))
            .filter(outreach_templates::Column::IsActive.eq(true))
            .order_by_desc(outreach_templates::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(outreach_template_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<OutreachTemplate>, Box<dyn std::error::Error>> {
        let entity = outreach_templates::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(outreach_template_to_domain))
    }

    pub async fn update(db: &DatabaseConnection, template: OutreachTemplate) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = outreach_templates::ActiveModel {
            id: Set(template.id),
            name: Set(template.name),
            template_type: Set(serde_json::to_string(&template.template_type)?),
            subject: Set(template.subject),
            content: Set(template.content),
            description: Set(template.description),
            is_default: Set(template.is_default),
            is_active: Set(template.is_active),
            created_by: Set(template.created_by),
            created_at: Set(template.created_at),
            updated_at: Set(template.updated_at),
        };
        active_model.update(db).await?;
        Ok(())
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        outreach_templates::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn get_default_templates(db: &DatabaseConnection) -> Result<Vec<OutreachTemplate>, Box<dyn std::error::Error>> {
        let entities = outreach_templates::Entity::find()
            .filter(outreach_templates::Column::IsDefault.eq(true))
            .filter(outreach_templates::Column::IsActive.eq(true))
            .order_by_desc(outreach_templates::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(outreach_template_to_domain).collect())
    }

    pub async fn get_user_templates(db: &DatabaseConnection, user_id: &str) -> Result<Vec<OutreachTemplate>, Box<dyn std::error::Error>> {
        let entities = outreach_templates::Entity::find()
            .filter(outreach_templates::Column::CreatedBy.eq(user_id))
            .filter(outreach_templates::Column::IsActive.eq(true))
            .order_by_desc(outreach_templates::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(outreach_template_to_domain).collect())
    }
}