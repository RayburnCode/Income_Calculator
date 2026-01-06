use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "other_fees")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub third_party_fees: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub appraisal_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub investor_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub padded_taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub padded_insurance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub lender_credit: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub admin_fees: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub tax_service: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub flood_certification: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub total_closing_costs: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub cash_out_amount: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}