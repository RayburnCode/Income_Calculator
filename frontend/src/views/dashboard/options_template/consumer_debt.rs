use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DebtRow {
    pub debtor_name: String,
    pub credit_type: String,
    pub balance: String,
    pub monthly_payment: String,
    pub term: String,
    pub rate: String,
    pub omit: bool,
    pub pay: bool,
}

#[component]
pub fn ConsumerDebtSection() -> Element {
    let mut debt_rows = use_signal(|| vec![
        DebtRow {
            debtor_name: String::new(),
            credit_type: String::new(),
            balance: String::new(),
            monthly_payment: String::new(),
            term: String::new(),
            rate: String::new(),
            omit: false,
            pay: false,
        }
    ]);

    let add_row = move |_| {
        debt_rows.with_mut(|rows| {
            rows.push(DebtRow {
                debtor_name: String::new(),
                credit_type: String::new(),
                balance: String::new(),
                monthly_payment: String::new(),
                term: String::new(),
                rate: String::new(),
                omit: false,
                pay: false,
            });
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
                        for (index , row) in debt_rows().iter().enumerate() {
                            tr {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "text",
                                        name: "debtorName",
                                        value: "{row.debtor_name}",
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
                                        value: "{row.balance}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "monthlyDebtPayment",
                                        value: "{row.monthly_payment}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "termDebt",
                                        value: "{row.term}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "rateDebt",
                                        value: "{row.rate}",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "omitDebt",
                                        checked: row.omit,
                                        class: "w-4 h-4",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        name: "paydebt",
                                        checked: row.pay,
                                        class: "w-4 h-4",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    button {
                                        r#type: "button",
                                        class: "bg-red-500 hover:bg-red-700 text-white px-3 py-1 rounded text-sm",
                                        name: "delete",
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
                    onclick: add_row,
                    "Add Row"
                }
            }
        }
    }
}