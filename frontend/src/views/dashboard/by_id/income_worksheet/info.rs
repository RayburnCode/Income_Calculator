use dioxus::prelude::*;
use crate::components::Input;
use shared::models::Borrower;

#[component]
pub fn Information() -> Element {
    // State for borrower information
    let borrower = use_signal(|| Borrower::default());

    rsx! {
        div { class: "space-y-6",
            // Borrower Basic Information
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Input {
                    placeholder: "Borrower's Full Name",
                    label: "Borrower's Name",
                    value: borrower().name.clone(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.name = evt.value();
                        borrower.set(b);
                    },
                }
                Input {
                    placeholder: "Optional - Loan Application Number",
                    label: "Loan Number",
                    value: borrower().loan_number.clone().unwrap_or_default(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.loan_number = if evt.value().is_empty() { None } else { Some(evt.value()) };
                        borrower.set(b);
                    },
                }
            }

            // Employment Information
            div { class: "bg-gray-50 p-6 rounded-lg",
                h3 { class: "text-lg font-semibold text-gray-900 mb-4", "Employment Information" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    Input {
                        placeholder: "Current Employer",
                        label: "Employer Name",
                        value: borrower().employer_name.clone().unwrap_or_default(),
                        oninput: move |evt: FormEvent| {
                            let mut borrower = borrower;
                            let mut b = borrower();
                            b.employer_name = if evt.value().is_empty() { None } else { Some(evt.value()) };
                            borrower.set(b);
                        },
                    }
                    Input {
                        placeholder: "Salary, Hourly, Commission, etc.",
                        label: "Income Type",
                        value: borrower().income_type.clone().unwrap_or_default(),
                        oninput: move |evt: FormEvent| {
                            let mut borrower = borrower;
                            let mut b = borrower();
                            b.income_type = if evt.value().is_empty() { None } else { Some(evt.value()) };
                            borrower.set(b);
                        },
                    }
                }
            }
        }
    }
}
 