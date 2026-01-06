use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::enums::{PropertyType, OccupancyType, LoanType, LoanPurpose};

// Existing Loan (Payoff) Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct ExistingLoan {
    pub id: Uuid,
    pub position: u8, // 1st, 2nd, 3rd mortgage
    #[validate(range(min = 0.0))]
    pub loan_balance: f64,
    #[validate(range(min = 0.0))]
    pub monthly_payment: f64,
    #[validate(range(min = 1, max = 360))]
    pub remaining_term_months: u32,
    #[validate(range(min = 0.0, max = 100.0))]
    pub interest_rate: f64,
    pub is_subordinate: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ExistingLoan {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            position: 1,
            loan_balance: 0.0,
            monthly_payment: 0.0,
            remaining_term_months: 360,
            interest_rate: 0.0,
            is_subordinate: false,
            created_at: now,
            updated_at: now,
        }
    }
}

// New Loan Details Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct NewLoanDetails {
    pub id: Uuid,
    #[validate(range(min = 0.0))]
    pub market_value: f64,
    #[validate(range(min = 0.0))]
    pub sales_price: f64,
    #[validate(range(min = 0.0))]
    pub down_payment: f64,
    #[validate(range(min = 0.0))]
    pub base_loan_amount: f64,
    #[validate(range(min = 0.0))]
    pub subordinated_amount: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub ff_umip_percentage: f64,
    #[validate(range(min = 0.0))]
    pub umip_refund: f64,
    #[validate(range(min = 0.0))]
    pub total_loan_amount: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub note_rate: f64,
    pub appraisal_waiver: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for NewLoanDetails {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            market_value: 0.0,
            sales_price: 0.0,
            down_payment: 0.0,
            base_loan_amount: 0.0,
            subordinated_amount: 0.0,
            ff_umip_percentage: 0.0,
            umip_refund: 0.0,
            total_loan_amount: 0.0,
            note_rate: 0.0,
            appraisal_waiver: false,
            created_at: now,
            updated_at: now,
        }
    }
}

// Loan Information Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanInformation {
    pub id: Uuid,
    pub property_type: PropertyType,
    pub occupancy_type: OccupancyType,
    pub loan_type: LoanType,
    pub new_term_months: u32,
    pub loan_purpose: LoanPurpose,
    pub appraisal_waiver: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for LoanInformation {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            property_type: PropertyType::SFR,
            occupancy_type: OccupancyType::Primary,
            loan_type: LoanType::CNV,
            new_term_months: 360,
            loan_purpose: LoanPurpose::Refinance,
            appraisal_waiver: false,
            created_at: now,
            updated_at: now,
        }
    }
}