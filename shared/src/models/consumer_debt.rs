use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::enums::CreditType;

// Consumer Debt Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConsumerDebt {
    pub id: Uuid,
    pub debtor_name: String,
    pub credit_type: CreditType,
    #[validate(range(min = 0.0))]
    pub balance: f64,
    #[validate(range(min = 0.0))]
    pub monthly_payment: f64,
    #[validate(range(min = 1, max = 360))]
    pub term_months: Option<u32>,
    #[validate(range(min = 0.0, max = 100.0))]
    pub interest_rate: Option<f64>,
    pub omit_from_dti: bool,
    pub pay_off_at_closing: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ConsumerDebt {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            debtor_name: String::new(),
            credit_type: CreditType::Installment,
            balance: 0.0,
            monthly_payment: 0.0,
            term_months: None,
            interest_rate: None,
            omit_from_dti: false,
            pay_off_at_closing: false,
            created_at: now,
            updated_at: now,
        }
    }
}