use dioxus::prelude::*;
use shared::models::{ConsumerDebtData, ConsumerDebtItemData};

#[component]
pub fn ConsumerDebtSection(data: ConsumerDebtData, on_change: EventHandler<ConsumerDebtData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    let add_debt = move |_| {
        local_data.with_mut(|data| {
            data.consumer_debts.push(ConsumerDebtItemData::default());
            on_change.call(data.clone());
        });
    };

    let mut remove_debt = move |index: usize| {
        local_data.with_mut(|data| {
            if data.consumer_debts.len() > 1 {
                data.consumer_debts.remove(index);
            }
            on_change.call(data.clone());
        });
    };

    let mut update_debt = move |index: usize, field: String, value: String| {
        local_data.with_mut(|data| {
            if let Some(debt) = data.consumer_debts.get_mut(index) {
                match field.as_str() {
                    "debtor_name" => debt.debtor_name = value,
                    "credit_type" => debt.credit_type = value,
                    "balance" => debt.balance = value.parse().unwrap_or(0.0),
                    "monthly_payment" => debt.monthly_payment = value.parse().unwrap_or(0.0),
                    "term_months" => debt.term_months = value.parse().ok(),
                    "interest_rate" => debt.interest_rate = value.parse().ok(),
                    "omit_from_dti" => debt.omit_from_dti = value == "true",
                    "pay_off_at_closing" => debt.pay_off_at_closing = value == "true",
                    _ => {}
                }
            }
            on_change.call(data.clone());
        });
    };

    rsx! {
        div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Consumer Debt" }
            div { class: "overflow-x-auto scrollbar-hide",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Debtor"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Type"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Balance"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Monthly"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Term"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Rate"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Omit"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Pay"
                            }
                            th { class: "border border-gray-300 px-2 sm:px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Action"
                            }
                        }
                    }
                    tbody {
                        for (index , debt) in local_data().consumer_debts.iter().enumerate() {
                            tr {
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    input {
                                        r#type: "text",
                                        name: "debtorName",
                                        value: "{debt.debtor_name}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        oninput: move |e| update_debt(index, "debtor_name".to_string(), e.value()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    select {
                                        name: "creditType",
                                        value: "{debt.credit_type}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        onchange: move |e| update_debt(index, "credit_type".to_string(), e.value()),
                                        option { value: "", "" }
                                        option { value: "Installment", "Installment" }
                                        option { value: "Mortgage", "Mortgage" }
                                        option { value: "Revolving", "Revolving" }
                                        option { value: "Lease", "Lease" }
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "balanceTotal",
                                        value: "{debt.balance}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        oninput: move |e| update_debt(index, "balance".to_string(), e.value()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "monthlyDebtPayment",
                                        value: "{debt.monthly_payment}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        oninput: move |e| update_debt(index, "monthly_payment".to_string(), e.value()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "termDebt",
                                        value: "{debt.term_months.unwrap_or(0)}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        oninput: move |e| update_debt(index, "term_months".to_string(), e.value()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "rateDebt",
                                        value: "{debt.interest_rate.unwrap_or(0.0)}",
                                        class: "w-full px-1 sm:px-2 py-1 border rounded text-sm",
                                        oninput: move |e| update_debt(index, "interest_rate".to_string(), e.value()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "omitDebt",
                                        checked: debt.omit_from_dti,
                                        class: "w-3 h-3 sm:w-4 sm:h-4",
                                        onchange: move |e| update_debt(index, "omit_from_dti".to_string(), e.checked().to_string()),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "paydebt",
                                        checked: debt.pay_off_at_closing,
                                        class: "w-3 h-3 sm:w-4 sm:h-4",
                                        onchange: move |e| update_debt(
                                            index,
                                            "pay_off_at_closing".to_string(),
                                            e.checked().to_string(),
                                        ),
                                    }
                                }
                                td { class: "border border-gray-300 px-2 sm:px-4 py-2 text-center",
                                    button {
                                        r#type: "button",
                                        class: "bg-red-500 hover:bg-red-700 text-white px-2 sm:px-3 py-1 rounded text-xs sm:text-sm",
                                        onclick: move |_| remove_debt(index),
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-4",
                button {
                    r#type: "button",
                    class: "bg-blue-500 hover:bg-blue-700 text-white px-4 py-2 rounded",
                    onclick: add_debt,
                    "Add Row"
                }
            }
        }
    }
}