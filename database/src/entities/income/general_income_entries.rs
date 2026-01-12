use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "general_income_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub borrower_id: i32,
    pub income_type: String,
    pub source_name: String,
    pub description: String,
    pub monthly_amount: String,
    pub annual_amount: String,
    pub is_verified: bool,
    pub verified_at: Option<DateTimeUtc>,
    pub notes: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entities::client::borrower::Entity",
        from = "Column::BorrowerId",
        to = "crate::entities::client::borrower::Column::Id"
    )]
    Borrower,
}

impl Related<crate::entities::client::borrower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Borrower.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}