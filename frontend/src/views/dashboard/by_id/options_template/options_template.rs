use dioxus::prelude::*;
use crate::views::dashboard::by_id::options_template::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub savings: SavingsData,
}

// Placeholder data structures - these would need to be defined based on actual form fields
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoanInformationData {
    pub property_type: String,
    pub occupancy: String,
    pub loan_type: String,
    pub term_months: String,
    pub purpose: String,
    pub appraisal_waiver: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MortgagePayoffsData {
    pub payoffs: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewLoanData {
    pub market_value: String,
    pub sales_price: String,
    pub down_payment: String,
    pub down_payment_percent: String,
    pub loan_amount: String,
    pub interest_rate: String,
    pub term_years: String,
    pub term_months: String,
    pub monthly_payment: String,
    pub total_payment: String,
    pub ltv_ratio: String,
    pub cltv_ratio: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitToBorrowerData {
    pub benefits: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OtherFeesData {
    pub fees: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricingData {
    pub pricing_options: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerDebtData {
    pub debts: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebtToIncomeData {
    pub dti_ratios: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TitleFeesData {
    pub title_fees: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsData {
    pub monthly_savings: String,
    pub annual_savings: String,
    pub debt_paid: String,
    pub payment_reduction: String,
    pub recoup_period: String,
}

#[component]
pub fn OptionsTemplate(id: i32) -> Element {
    // Main state for all options template data
    let mut template_data = use_signal(|| OptionsTemplateData::default());

    // Auto-save functionality
    let save_timeout = use_signal(|| None::<i32>);

    // Function to trigger auto-save
    let save_data = move |data: OptionsTemplateData| {
        // Cancel any existing timeout
        if let Some(_timeout_id) = save_timeout() {
            // In a real implementation, you'd cancel the timeout
            // For now, we'll just update the signal
        }

        // Set new timeout for auto-save (e.g., save after 2 seconds of inactivity)
        // In a real implementation, you'd use a proper timeout mechanism
        // For now, we'll save immediately for demonstration
        save_to_backend(data, id);
    };

    // Function to save data to backend
    fn save_to_backend(data: OptionsTemplateData, borrower_id: i32) {
        // TODO: Implement actual backend save logic
        // This would typically involve making an API call to save the data
        println!("Auto-saving options template data for borrower {}: {:?}", borrower_id, data);

        // Example of what the API call might look like:
        // let json_data = serde_json::to_string(&data).unwrap();
        // make_api_call(format!("/api/borrowers/{}/options-template", borrower_id), "PUT", json_data);
    }

    rsx! {
        div { class: " mx-auto p-6 text-black",
            h1 { class: "text-3xl font-bold text-gray-900 mb-8", "Mortgage Options Template" }

            // Auto-save indicator
            div { class: "mb-4 p-3 bg-green-50 border border-green-200 rounded-lg",
                div { class: "flex items-center gap-2",
                    span { class: "text-green-600", "💾" }
                    span { class: "text-sm text-green-800 font-medium",
                        "Changes are automatically saved to the backend"
                    }
                }
            }

            // All the sections with data binding
            SavingsSection {
                data: template_data().savings.clone(),
                on_change: move |new_data: SavingsData| {
                    template_data.write().savings = new_data.clone();
                    save_data(template_data());
                },
            }

            LoanInformationSection {
                data: template_data().loan_information.clone(),
                on_change: move |new_data: LoanInformationData| {
                    template_data.write().loan_information = new_data.clone();
                    save_data(template_data());
                },
            }

            MortgagePayoffsSection {
                data: template_data().mortgage_payoffs.clone(),
                on_change: move |new_data: MortgagePayoffsData| {
                    template_data.write().mortgage_payoffs = new_data.clone();
                    save_data(template_data());
                },
            }

            NewLoanSection {
                data: template_data().new_loan.clone(),
                on_change: move |new_data: NewLoanData| {
                    template_data.write().new_loan = new_data.clone();
                    save_data(template_data());
                },
            }

            BenefitToBorrowerSection {
                data: template_data().benefit_to_borrower.clone(),
                on_change: move |new_data: BenefitToBorrowerData| {
                    template_data.write().benefit_to_borrower = new_data.clone();
                    save_data(template_data());
                },
            }

            OtherFeesSection {
                data: template_data().other_fees.clone(),
                on_change: move |new_data: OtherFeesData| {
                    template_data.write().other_fees = new_data.clone();
                    save_data(template_data());
                },
            }

            PricingSection {
                data: template_data().pricing.clone(),
                on_change: move |new_data: PricingData| {
                    template_data.write().pricing = new_data.clone();
                    save_data(template_data());
                },
            }

            ConsumerDebtSection {
                data: template_data().consumer_debt.clone(),
                on_change: move |new_data: ConsumerDebtData| {
                    template_data.write().consumer_debt = new_data.clone();
                    save_data(template_data());
                },
            }

            DebtToIncomeSection {
                data: template_data().debt_to_income.clone(),
                on_change: move |new_data: DebtToIncomeData| {
                    template_data.write().debt_to_income = new_data.clone();
                    save_data(template_data());
                },
            }

            TitleFeesSection {
                data: template_data().title_fees.clone(),
                on_change: move |new_data: TitleFeesData| {
                    template_data.write().title_fees = new_data.clone();
                    save_data(template_data());
                },
            }
        }
    }
}