use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export shared enums for database use
pub use shared::models::{
    PropertyType, OccupancyType, LoanType, LoanPurpose, CreditType,
};

// Database entities for mortgage refinance models

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "consumer_debts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub debtor_name: String,
    pub credit_type: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub balance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub monthly_payment: Decimal,
    pub term_months: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub interest_rate: Option<Decimal>,
    pub omit_from_dti: bool,
    pub pay_off_at_closing: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}