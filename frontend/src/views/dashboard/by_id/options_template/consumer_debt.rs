use dioxus::prelude::*;
use shared::models::ConsumerDebt;

#[component]
pub fn ConsumerDebtSection() -> Element {
    let mut consumer_debts = use_signal(|| vec![
        ConsumerDebt::default()
    ]);

    let add_debt = move |_| {
        consumer_debts.with_mut(|debts| {
            debts.push(ConsumerDebt::default());
        });
    };

    let mut remove_debt = move |index: usize| {
        consumer_debts.with_mut(|debts| {
            if debts.len() > 1 {
                debts.remove(index);
            }
        });
    };

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4", "Consumer Debt" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Debtor"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Type"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Balance"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Monthly"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Term"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Rate"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Omit"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Pay"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Action"
                            }
                        }
                    }
                    tbody {
                        for (index , debt) in consumer_debts().iter().enumerate() {
                            tr {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "text",
                                        name: "debtorName",
                                        value: "{debt.debtor_name}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    select {
                                        name: "creditType",
                                        class: "w-full px-2 py-1 border rounded",
                                        option { value: "", "" }
                                        option { value: "Installment", "Installment" }
                                        option { value: "Mortgage", "Mortgage" }
                                        option { value: "Revolving", "Revolving" }
                                        option { value: "Lease", "Lease" }
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "balanceTotal",
                                        value: "{debt.balance}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "monthlyDebtPayment",
                                        value: "{debt.monthly_payment}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "termDebt",
                                        value: "{debt.term_months.unwrap_or(0)}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "rateDebt",
                                        value: "{debt.interest_rate.unwrap_or(0.0)}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "omitDebt",
                                        checked: debt.omit_from_dti,
                                        class: "w-4 h-4",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "paydebt",
                                        checked: debt.pay_off_at_closing,
                                        class: "w-4 h-4",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    button {
                                        r#type: "button",
                                        class: "bg-red-500 hover:bg-red-700 text-white px-3 py-1 rounded text-sm",
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