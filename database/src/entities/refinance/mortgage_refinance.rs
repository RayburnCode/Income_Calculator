use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mortgage_refinance_options")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub borrower_id: i32,
    pub loan_information_id: Uuid,
    pub new_loan_details_id: Uuid,
    pub benefit_to_borrower_id: Uuid,
    pub other_fees_id: Uuid,
    pub income_information_id: Uuid,
    pub savings_calculation_id: Uuid,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub submitted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entities::loans::loan_information::Entity",
        from = "Column::LoanInformationId",
        to = "crate::entities::loans::loan_information::Column::Id"
    )]
    LoanInformation,
    #[sea_orm(
        belongs_to = "crate::entities::loans::new_loan_details::Entity",
        from = "Column::NewLoanDetailsId",
        to = "crate::entities::loans::new_loan_details::Column::Id"
    )]
    NewLoanDetails,
    #[sea_orm(
        belongs_to = "crate::entities::calculations::benefit_to_borrower::Entity",
        from = "Column::BenefitToBorrowerId",
        to = "crate::entities::calculations::benefit_to_borrower::Column::Id"
    )]
    BenefitToBorrower,
    #[sea_orm(
        belongs_to = "crate::entities::refinance::other_fees::Entity",
        from = "Column::OtherFeesId",
        to = "crate::entities::refinance::other_fees::Column::Id"
    )]
    OtherFees,
    #[sea_orm(
        belongs_to = "crate::entities::income::income_information::Entity",
        from = "Column::IncomeInformationId",
        to = "crate::entities::income::income_information::Column::Id"
    )]
    IncomeInformation,
    #[sea_orm(
        belongs_to = "crate::entities::calculations::savings_calculations::Entity",
        from = "Column::SavingsCalculationId",
        to = "crate::entities::calculations::savings_calculations::Column::Id"
    )]
    SavingsCalculation,
    // Note: Many-to-many relationships with existing_loans, pricing_options, and consumer_debt
    // are handled through junction tables and require custom implementation
}

impl Related<crate::entities::loans::loan_information::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoanInformation.def()
    }
}

impl Related<crate::entities::loans::new_loan_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NewLoanDetails.def()
    }
}

impl Related<crate::entities::calculations::benefit_to_borrower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BenefitToBorrower.def()
    }
}

impl Related<crate::entities::refinance::other_fees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OtherFees.def()
    }
}

impl Related<crate::entities::income::income_information::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IncomeInformation.def()
    }
}

impl Related<crate::entities::calculations::savings_calculations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SavingsCalculation.def()
    }
}

// Note: Many-to-many relationships require custom junction table handling
// and are not implemented with the standard Related trait

impl ActiveModelBehavior for ActiveModel {}