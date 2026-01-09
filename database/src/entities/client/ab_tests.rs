use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ab_tests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub campaign_id: i32,
    pub test_name: String,
    pub subject_a: String,
    pub subject_b: String,
    pub winner: Option<String>,
    pub sent_a: i32,
    pub sent_b: i32,
    pub opened_a: i32,
    pub opened_b: i32,
    pub clicked_a: i32,
    pub clicked_b: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::campaigns::Entity",
        from = "Column::CampaignId",
        to = "super::campaigns::Column::Id"
    )]
    Campaign,
}

impl Related<super::campaigns::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Campaign.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}