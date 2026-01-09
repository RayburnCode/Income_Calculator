//! Campaigns repository - handles campaign CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait, QueryOrder, QueryFilter, ColumnTrait};
use shared::models::*;
use database::entities::{campaigns, ab_tests};
use crate::converters::*;

pub struct CampaignsRepository;

impl CampaignsRepository {
    pub async fn save(db: &DatabaseConnection, campaign: Campaign) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = campaigns::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(campaign.name),
            description: Set(campaign.description),
            campaign_type: Set(serde_json::to_string(&campaign.campaign_type)?),
            template_id: Set(campaign.template_id),
            segment_criteria: Set(campaign.segment_criteria),
            status: Set(serde_json::to_string(&campaign.status)?),
            scheduled_date: Set(campaign.scheduled_date),
            completed_date: Set(campaign.completed_date),
            target_audience_count: Set(campaign.target_audience_count),
            sent_count: Set(campaign.sent_count),
            opened_count: Set(campaign.opened_count),
            clicked_count: Set(campaign.clicked_count),
            converted_count: Set(campaign.converted_count),
            created_by: Set(campaign.created_by),
            created_at: Set(campaign.created_at),
            updated_at: Set(campaign.updated_at),
        };
        active_model.insert(db).await?;
        Ok(())
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Campaign>, Box<dyn std::error::Error>> {
        let entities = campaigns::Entity::find()
            .order_by_desc(campaigns::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(campaign_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Campaign>, Box<dyn std::error::Error>> {
        let entity = campaigns::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(campaign_to_domain))
    }

    pub async fn get_by_status(db: &DatabaseConnection, status: CampaignStatus) -> Result<Vec<Campaign>, Box<dyn std::error::Error>> {
        let entities = campaigns::Entity::find()
            .filter(campaigns::Column::Status.eq(serde_json::to_string(&status)?))
            .order_by_desc(campaigns::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(campaign_to_domain).collect())
    }

    pub async fn update(db: &DatabaseConnection, campaign: Campaign) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = campaigns::ActiveModel {
            id: Set(campaign.id),
            name: Set(campaign.name),
            description: Set(campaign.description),
            campaign_type: Set(serde_json::to_string(&campaign.campaign_type)?),
            template_id: Set(campaign.template_id),
            segment_criteria: Set(campaign.segment_criteria),
            status: Set(serde_json::to_string(&campaign.status)?),
            scheduled_date: Set(campaign.scheduled_date),
            completed_date: Set(campaign.completed_date),
            target_audience_count: Set(campaign.target_audience_count),
            sent_count: Set(campaign.sent_count),
            opened_count: Set(campaign.opened_count),
            clicked_count: Set(campaign.clicked_count),
            converted_count: Set(campaign.converted_count),
            created_by: Set(campaign.created_by),
            created_at: Set(campaign.created_at),
            updated_at: Set(campaign.updated_at),
        };
        active_model.update(db).await?;
        Ok(())
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        campaigns::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn update_analytics(db: &DatabaseConnection, campaign_id: i32, sent: i32, opened: i32, clicked: i32, converted: i32) -> Result<(), Box<dyn std::error::Error>> {
        use sea_orm::sea_query::Expr;
        use sea_orm::EntityTrait;

        campaigns::Entity::update_many()
            .col_expr(campaigns::Column::SentCount, Expr::col(campaigns::Column::SentCount).add(sent))
            .col_expr(campaigns::Column::OpenedCount, Expr::col(campaigns::Column::OpenedCount).add(opened))
            .col_expr(campaigns::Column::ClickedCount, Expr::col(campaigns::Column::ClickedCount).add(clicked))
            .col_expr(campaigns::Column::ConvertedCount, Expr::col(campaigns::Column::ConvertedCount).add(converted))
            .filter(campaigns::Column::Id.eq(campaign_id))
            .exec(db)
            .await?;
        Ok(())
    }
}

pub struct ABTestsRepository;

impl ABTestsRepository {
    pub async fn save(db: &DatabaseConnection, ab_test: ABTest) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = ab_tests::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            campaign_id: Set(ab_test.campaign_id),
            test_name: Set(ab_test.test_name),
            subject_a: Set(ab_test.subject_a),
            subject_b: Set(ab_test.subject_b),
            winner: Set(ab_test.winner),
            sent_a: Set(ab_test.sent_a),
            sent_b: Set(ab_test.sent_b),
            opened_a: Set(ab_test.opened_a),
            opened_b: Set(ab_test.opened_b),
            clicked_a: Set(ab_test.clicked_a),
            clicked_b: Set(ab_test.clicked_b),
            created_at: Set(ab_test.created_at),
        };
        active_model.insert(db).await?;
        Ok(())
    }

    pub async fn get_by_campaign_id(db: &DatabaseConnection, campaign_id: i32) -> Result<Vec<ABTest>, Box<dyn std::error::Error>> {
        let entities = ab_tests::Entity::find()
            .filter(ab_tests::Column::CampaignId.eq(campaign_id))
            .order_by_desc(ab_tests::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(entities.iter().map(ab_test_to_domain).collect())
    }

    pub async fn update_results(db: &DatabaseConnection, test_id: i32, sent_a: i32, sent_b: i32, opened_a: i32, opened_b: i32, clicked_a: i32, clicked_b: i32, winner: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = ab_tests::ActiveModel {
            id: Set(test_id),
            winner: Set(winner),
            sent_a: Set(sent_a),
            sent_b: Set(sent_b),
            opened_a: Set(opened_a),
            opened_b: Set(opened_b),
            clicked_a: Set(clicked_a),
            clicked_b: Set(clicked_b),
            ..Default::default()
        };
        active_model.update(db).await?;
        Ok(())
    }
}