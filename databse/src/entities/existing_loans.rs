use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "existing_loans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub position: i16,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub loan_balance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub monthly_payment: Decimal,
    pub remaining_term_months: u32,
    #[sea_orm(column_type = "Decimal(Some((5, 3)))")]
    pub interest_rate: Decimal,
    pub is_subordinate: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}