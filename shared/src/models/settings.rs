use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub id: i32,
    pub theme: String,
    pub currency: String,
    pub default_loan_term: i32,
    pub dti_threshold: f64,
    pub auto_backup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: 1,
            theme: "light".to_string(),
            currency: "USD ($)".to_string(),
            default_loan_term: 30,
            dti_threshold: 43.0,
            auto_backup: false,
        }
    }
}

impl AppSettings {
    pub fn currency_symbol(&self) -> &'static str {
        match self.currency.as_str() {
            "USD ($)" => "$",
            "EUR (€)" => "€",
            "GBP (£)" => "£",
            _ => "$",
        }
    }

    pub fn format_currency(&self, amount: f64) -> String {
        format!("{}{:.2}", self.currency_symbol(), amount)
    }
}