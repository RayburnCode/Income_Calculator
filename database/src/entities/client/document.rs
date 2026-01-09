use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub client_id: i32,
    pub filename: String,
    pub file_size: i64,
    pub file_type: String,
    pub file_path: String,
    pub mime_type: String,
    pub upload_date: DateTimeUtc,
    pub description: Option<String>,
    pub checksum: Option<String>,
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