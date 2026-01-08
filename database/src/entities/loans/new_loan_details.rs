use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "new_loan_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub market_value: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub sales_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub down_payment: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub base_loan_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub subordinated_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ff_umip_percentage: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub umip_refund: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub total_loan_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 3)))")]
    pub note_rate: Decimal,
    pub appraisal_waiver: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}