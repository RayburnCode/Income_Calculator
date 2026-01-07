use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Benefit to Borrower Comparison Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct BenefitToBorrower {
    pub id: Uuid,
    // Existing loan payments
    #[validate(range(min = 0.0))]
    pub existing_pi: f64,
    #[validate(range(min = 0.0))]
    pub existing_taxes: f64,
    #[validate(range(min = 0.0))]
    pub existing_insurance: f64,
    #[validate(range(min = 0.0))]
    pub existing_flood_insurance: f64,
    #[validate(range(min = 0.0))]
    pub existing_pmi: f64,
    #[validate(range(min = 0.0))]
    pub existing_hoa: f64,
    #[validate(range(min = 0.0))]
    pub existing_mortgage_payment: f64,

    // Proposed loan payments
    #[validate(range(min = 0.0))]
    pub proposed_pi: f64,
    #[validate(range(min = 0.0))]
    pub proposed_taxes: f64,
    #[validate(range(min = 0.0))]
    pub proposed_insurance: f64,
    #[validate(range(min = 0.0))]
    pub proposed_flood_insurance: f64,
    #[validate(range(min = 0.0))]
    pub proposed_pmi: f64,
    #[validate(range(min = 0.0))]
    pub proposed_hoa: f64,
    #[validate(range(min = 0.0))]
    pub proposed_mortgage_payment: f64,

    // Escrow checkboxes
    pub escrow_taxes: bool,
    pub escrow_insurance: bool,
    pub escrow_flood_insurance: bool,

    // Calculations
    #[validate(range(min = 0.0))]
    pub overage_shortage: f64,
    #[validate(range(min = 0.0))]
    pub debt_paydown: f64,
    #[validate(range(min = 0.0))]
    pub existing_total_obligations: f64,
    #[validate(range(min = 0.0))]
    pub proposed_total_obligations: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for BenefitToBorrower {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            existing_pi: 0.0,
            existing_taxes: 0.0,
            existing_insurance: 0.0,
            existing_flood_insurance: 0.0,
            existing_pmi: 0.0,
            existing_hoa: 0.0,
            existing_mortgage_payment: 0.0,
            proposed_pi: 0.0,
            proposed_taxes: 0.0,
            proposed_insurance: 0.0,
            proposed_flood_insurance: 0.0,
            proposed_pmi: 0.0,
            proposed_hoa: 0.0,
            proposed_mortgage_payment: 0.0,
            escrow_taxes: false,
            escrow_insurance: false,
            escrow_flood_insurance: false,
            overage_shortage: 0.0,
            debt_paydown: 0.0,
            existing_total_obligations: 0.0,
            proposed_total_obligations: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}

// Other Fees Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct OtherFees {
    pub id: Uuid,
    #[validate(range(min = 0.0))]
    pub third_party_fees: f64,
    #[validate(range(min = 0.0))]
    pub appraisal_fee: f64,
    #[validate(range(min = 0.0))]
    pub investor_fee: f64,
    #[validate(range(min = 0.0))]
    pub padded_taxes: f64,
    #[validate(range(min = 0))]
    pub padded_taxes_months: u32,
    #[validate(range(min = 0.0))]
    pub padded_insurance: f64,
    #[validate(range(min = 0))]
    pub padded_insurance_months: u32,
    #[validate(range(min = 0.0))]
    pub lender_credit: f64,
    #[validate(range(min = 0.0))]
    pub admin_fees: f64,
    #[validate(range(min = 0.0))]
    pub tax_service: f64,
    #[validate(range(min = 0.0))]
    pub flood_certification: f64,
    #[validate(range(min = 0.0))]
    pub total_closing_costs: f64,
    #[validate(range(min = 0.0))]
    pub cash_out_amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for OtherFees {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            third_party_fees: 0.0,
            appraisal_fee: 0.0,
            investor_fee: 0.0,
            padded_taxes: 0.0,
            padded_taxes_months: 0,
            padded_insurance: 0.0,
            padded_insurance_months: 0,
            lender_credit: 0.0,
            admin_fees: 895.0,
            tax_service: 88.0,
            flood_certification: 8.0,
            total_closing_costs: 0.0,
            cash_out_amount: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}

// Pricing Option Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct PricingOption {
    pub id: Uuid,
    pub description: String,
    #[validate(range(min = 0.0, max = 100.0))]
    pub note_rate: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub ysp_percentage: f64,
    #[validate(range(min = 0.0))]
    pub ysp_dollar: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub bd_percentage: f64,
    #[validate(range(min = 0.0))]
    pub bd_dollar: f64,
    pub is_selected: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for PricingOption {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            description: String::new(),
            note_rate: 0.0,
            ysp_percentage: 0.0,
            ysp_dollar: 0.0,
            bd_percentage: 0.0,
            bd_dollar: 0.0,
            is_selected: false,
            created_at: now,
            updated_at: now,
        }
    }
}

// Income Information Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct IncomeInformation {
    pub id: Uuid,
    #[validate(range(min = 0.0))]
    pub borrower_monthly_income: f64,
    #[validate(range(min = 0.0))]
    pub coborrower_monthly_income: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub front_end_ratio: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub back_end_ratio: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for IncomeInformation {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            borrower_monthly_income: 0.0,
            coborrower_monthly_income: 0.0,
            front_end_ratio: 0.0,
            back_end_ratio: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}

// Savings Calculation Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct SavingsCalculation {
    pub id: Uuid,
    #[validate(range(min = 0.0))]
    pub monthly_savings: f64,
    #[validate(range(min = 0.0))]
    pub annual_savings: f64,
    #[validate(range(min = 0.0))]
    pub debt_paid: f64,
    #[validate(range(min = 0.0))]
    pub payment_reduction: f64,
    #[validate(range(min = 0.0))]
    pub recoup_period_months: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for SavingsCalculation {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            monthly_savings: 0.0,
            annual_savings: 0.0,
            debt_paid: 0.0,
            payment_reduction: 0.0,
            recoup_period_months: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}