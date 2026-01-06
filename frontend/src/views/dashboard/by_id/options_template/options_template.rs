use dioxus::prelude::*;
use crate::views::dashboard::by_id::options_template::*;

#[component]
pub fn OptionsTemplate() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto p-6",
            h1 { class: "text-3xl font-bold text-gray-900 mb-8", "Mortgage Options Template" }

            form { action: "/workbook/submit", method: "post",
                div { class: "mb-6 text-right",
                    button {
                        r#type: "submit",
                        class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-lg transition duration-300",
                    }
                    "Submit"
                }

                // All the sections
                SavingsSection {}
                LoanInformationSection {}
                MortgagePayoffsSection {}
                NewLoanSection {}
                BenefitToBorrowerSection {}
                OtherFeesSection {}
                PricingSection {}
                ConsumerDebtSection {}
                DebtToIncomeSection {}
                TitleFeesSection {}
            }
        }
    }
}