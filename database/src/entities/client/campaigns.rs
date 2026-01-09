use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "campaigns")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: String,
    pub template_id: i32,
    pub segment_criteria: Json,
    pub status: String,
    pub scheduled_date: Option<DateTimeUtc>,
    pub completed_date: Option<DateTimeUtc>,
    pub target_audience_count: Option<i32>,
    pub sent_count: i32,
    pub opened_count: i32,
    pub clicked_count: i32,
    pub converted_count: i32,
    pub created_by: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::outreach_templates::Entity",
        from = "Column::TemplateId",
        to = "super::outreach_templates::Column::Id"
    )]
    Template,
    #[sea_orm(has_many = "super::ab_tests::Entity")]
    ABTests,
}

impl Related<super::outreach_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Template.def()
    }
}

impl Related<super::ab_tests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ABTests.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}