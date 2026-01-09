use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conditions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub client_id: i32,
    pub title: String,
    pub description: String,
    pub condition_type: String,
    pub severity: String,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::borrower::Entity",
        from = "Column::ClientId",
        to = "super::borrower::Column::Id"
    )]
    Borrower,
}

impl Related<super::borrower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Borrower.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}