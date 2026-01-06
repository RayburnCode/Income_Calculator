use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::consumer_debt::ConsumerDebt;
use super::loan_models::{ExistingLoan, NewLoanDetails, LoanInformation};
use super::financial_models::{BenefitToBorrower, OtherFees, PricingOption, IncomeInformation, SavingsCalculation};

// Borrower Information Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Borrower {
    pub id: i32,
    pub name: String,
    pub employer_name: Option<String>,
    pub income_type: Option<String>,
    pub loan_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Borrower {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            name: String::new(),
            employer_name: None,
            income_type: None,
            loan_number: None,
            created_at: now,
            updated_at: now,
        }
    }
}

// Main Mortgage Refinance Options Template Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MortgageRefinanceOptions {
    pub id: Uuid,
    pub borrower_id: i32, // Reference to borrower/client

    // Core information
    pub loan_information: LoanInformation,
    pub new_loan_details: NewLoanDetails,

    // Existing loans to be paid off
    pub existing_loans: Vec<ExistingLoan>,

    // Comparison data
    pub benefit_to_borrower: BenefitToBorrower,

    // Financial details
    pub other_fees: OtherFees,
    pub pricing_options: Vec<PricingOption>,
    pub consumer_debts: Vec<ConsumerDebt>,
    pub income_information: IncomeInformation,

    // Calculations
    pub savings_calculation: SavingsCalculation,

    // Metadata
    pub status: String, // draft, submitted, approved, etc.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

impl Default for MortgageRefinanceOptions {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            borrower_id: 0, // Should be set when creating
            loan_information: LoanInformation::default(),
            new_loan_details: NewLoanDetails::default(),
            existing_loans: vec![ExistingLoan::default()],
            benefit_to_borrower: BenefitToBorrower::default(),
            other_fees: OtherFees::default(),
            pricing_options: vec![
                PricingOption::default(),
                PricingOption::default(),
                PricingOption::default(),
            ],
            consumer_debts: Vec::new(),
            income_information: IncomeInformation::default(),
            savings_calculation: SavingsCalculation::default(),
            status: "draft".to_string(),
            created_at: now,
            updated_at: now,
            submitted_at: None,
        }
    }
}