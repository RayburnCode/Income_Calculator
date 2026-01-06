use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pricing_options")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub description: String,
    #[sea_orm(column_type = "Decimal(Some((5, 3)))")]
    pub note_rate: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ysp_percentage: Decimal,
    #[sea_orm(column_type = "Decimal(Some((8, 2)))")]
    pub ysp_dollar: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub bd_percentage: Decimal,
    #[sea_orm(column_type = "Decimal(Some((8, 2)))")]
    pub bd_dollar: Decimal,
    pub is_selected: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}