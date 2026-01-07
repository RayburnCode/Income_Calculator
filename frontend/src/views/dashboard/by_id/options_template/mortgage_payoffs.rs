use dioxus::prelude::*;
use shared::models::MortgagePayoffsData;

#[component]
pub fn MortgagePayoffsSection(data: MortgagePayoffsData, on_change: EventHandler<MortgagePayoffsData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Mortgage Payoffs and Payments" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                ""
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Loan Balance"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Payment"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Term"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Rate"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Sub"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "1st"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "loanBalance1",
                                    id: "loanBalance1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "payment1",
                                    id: "payment1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldTerm1",
                                    id: "oldTerm1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldRate1",
                                    id: "oldRate1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "sub1",
                                    id: "sub1",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "2nd"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "loanBalance2",
                                    id: "loanBalance2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "payment2",
                                    id: "payment2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldTerm2",
                                    id: "oldTerm2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldRate2",
                                    id: "oldRate2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "sub2",
                                    id: "sub2",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "3rd"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "loanBalance3",
                                    id: "loanBalance3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "payment3",
                                    id: "payment3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldTerm3",
                                    id: "oldTerm3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "oldRate3",
                                    id: "oldRate3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "sub3",
                                    id: "sub3",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}