use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "income_information")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub borrower_monthly_income: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub coborrower_monthly_income: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub front_end_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub back_end_ratio: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}