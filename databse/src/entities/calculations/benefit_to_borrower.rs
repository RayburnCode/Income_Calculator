use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "benefit_to_borrower")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    // Existing loan payments
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_pi: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_insurance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_flood_insurance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_pmi: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_hoa: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_mortgage_payment: Decimal,

    // Proposed loan payments
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_pi: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_insurance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_flood_insurance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_pmi: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_hoa: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_mortgage_payment: Decimal,

    // Escrow checkboxes
    pub escrow_taxes: bool,
    pub escrow_insurance: bool,
    pub escrow_flood_insurance: bool,

    // Calculations
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub overage_shortage: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub debt_paydown: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub existing_total_obligations: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub proposed_total_obligations: Decimal,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}