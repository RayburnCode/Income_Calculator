use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "savings_calculations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub monthly_savings: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub annual_savings: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub debt_paid: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub payment_reduction: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub recoup_period_months: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}