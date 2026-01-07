use serde::{Deserialize, Serialize};

// Data structures for the options template
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionsTemplateData {
    pub loan_information: LoanInformationData,
    pub mortgage_payoffs: MortgagePayoffsData,
    pub new_loan: NewLoanData,
    pub benefit_to_borrower: BenefitToBorrowerData,
    pub other_fees: OtherFeesData,
    pub pricing: PricingData,
    pub consumer_debt: ConsumerDebtData,
    pub debt_to_income: DebtToIncomeData,
    pub title_fees: TitleFeesData,
    pub income_information: IncomeInformationData,
    pub savings: SavingsData,
}

// Placeholder data structures - these would need to be defined based on actual form fields
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoanInformationData {
    pub property_type: String,
    pub occupancy: String,
    pub loan_type: String,
    pub term_months: u32,
    pub purpose: String,
    pub appraisal_waiver: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MortgagePayoffsData {
    pub existing_loans: Vec<ExistingLoanData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExistingLoanData {
    pub position: u8,
    pub loan_balance: f64,
    pub monthly_payment: f64,
    pub remaining_term_months: u32,
    pub interest_rate: f64,
    pub is_subordinate: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewLoanData {
    pub market_value: f64,
    pub sales_price: f64,
    pub down_payment: f64,
    pub down_payment_percent: f64,
    pub base_loan_amount: f64,
    pub subordinated_amount: f64,
    pub total_loan_amount: f64,
    pub note_rate: f64,
    pub appraisal_waiver: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitToBorrowerData {
    // Existing loan payments
    pub existing_pi: f64,
    pub existing_taxes: f64,
    pub existing_insurance: f64,
    pub existing_flood_insurance: f64,
    pub existing_pmi: f64,
    pub existing_hoa: f64,
    pub existing_mortgage_payment: f64,

    // Proposed loan payments
    pub proposed_pi: f64,
    pub proposed_taxes: f64,
    pub proposed_insurance: f64,
    pub proposed_flood_insurance: f64,
    pub proposed_pmi: f64,
    pub proposed_hoa: f64,
    pub proposed_mortgage_payment: f64,

    // Escrow checkboxes
    pub escrow_taxes: bool,
    pub escrow_insurance: bool,
    pub escrow_flood_insurance: bool,

    // Calculations
    pub overage_shortage: f64,
    pub debt_paydown: f64,
    pub existing_total_obligations: f64,
    pub proposed_total_obligations: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OtherFeesData {
    pub third_party_fees: f64,
    pub appraisal_fee: f64,
    pub investor_fee: f64,
    pub padded_taxes: f64,
    pub padded_taxes_months: u32,
    pub padded_insurance: f64,
    pub padded_insurance_months: u32,
    pub lender_credit: f64,
    pub admin_fees: f64,
    pub tax_service: f64,
    pub flood_certification: f64,
    pub total_closing_costs: f64,
    pub cash_out_amount: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingData {
    pub pricing_options: Vec<PricingOptionData>,
}

impl Default for PricingData {
    fn default() -> Self {
        Self {
            pricing_options: vec![
                PricingOptionData::default(),
                PricingOptionData::default(),
                PricingOptionData::default(),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingOptionData {
    pub description: String,
    pub note_rate: f64,
    pub ysp_percentage: f64,
    pub ysp_dollar: f64,
    pub bd_percentage: f64,
    pub bd_dollar: f64,
    pub is_selected: bool,
}

impl Default for PricingOptionData {
    fn default() -> Self {
        Self {
            description: "Rate Option".to_string(),
            note_rate: 0.0,
            ysp_percentage: 0.0,
            ysp_dollar: 0.0,
            bd_percentage: 0.0,
            bd_dollar: 0.0,
            is_selected: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerDebtData {
    pub consumer_debts: Vec<ConsumerDebtItemData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerDebtItemData {
    pub debtor_name: String,
    pub credit_type: String,
    pub balance: f64,
    pub monthly_payment: f64,
    pub term_months: Option<u32>,
    pub interest_rate: Option<f64>,
    pub omit_from_dti: bool,
    pub pay_off_at_closing: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebtToIncomeData {
    pub front_end_ratio: f64,
    pub back_end_ratio: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TitleFeesData {
    pub title_insurance: f64,
    pub title_search: f64,
    pub title_examination: f64,
    pub title_closing_fee: f64,
    pub title_courier_fee: f64,
    pub title_other_fees: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeSource {
    pub id: String,
    pub name: String,
    pub monthly_amount: f64,
    pub category: String,
    pub included_in_dti: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncomeInformationData {
    pub borrower_monthly_income: f64,
    pub coborrower_monthly_income: f64,
    pub front_end_ratio: f64,
    pub back_end_ratio: f64,
    pub w2_jobs_data: Option<W2JobsData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct W2JobsData {
    pub jobs: Vec<W2Job>,
    pub is_verified: bool,
    pub verified_at: Option<String>,
}

impl Default for W2JobsData {
    fn default() -> Self {
        Self {
            jobs: vec![W2Job::default()],
            is_verified: false,
            verified_at: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct W2Job {
    pub employer_name: String,
    pub job_title: String,
    pub years_employed: String,
    pub months_employed: String,
    pub annual_salary: String,
    pub hourly_rate: String,
    pub hours_per_week: String,
    pub commission_monthly: String,
    pub bonus_monthly: String,
    pub overtime_monthly: String,
}

impl Default for W2Job {
    fn default() -> Self {
        Self {
            employer_name: String::new(),
            job_title: String::new(),
            years_employed: String::new(),
            months_employed: String::new(),
            annual_salary: String::new(),
            hourly_rate: String::new(),
            hours_per_week: String::new(),
            commission_monthly: String::new(),
            bonus_monthly: String::new(),
            overtime_monthly: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsData {
    pub monthly_savings: f64,
    pub annual_savings: f64,
    pub debt_paid: f64,
    pub payment_reduction: f64,
    pub recoup_period_months: f64,
}