use dioxus::prelude::*;
use crate::views::dashboard::by_id::options_template::options_template::BenefitToBorrowerData;

#[component]
pub fn BenefitToBorrowerSection(data: BenefitToBorrowerData, on_change: EventHandler<BenefitToBorrowerData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Benefit to the Borrower" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                ""
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Existing Loan"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Proposed Loan"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Escrow"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "PI:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "text",
                                    id: "existingPI",
                                    name: "existingPI",
                                    value: "From Cust tab",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedPI",
                                    name: "proposedPI",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Taxes:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingTaxes",
                                    name: "existingTaxes",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedTaxes",
                                    name: "proposedTaxes",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowTaxes",
                                    name: "escrowTaxes",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Insurance:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingInsurance",
                                    name: "existingInsurance",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedInsurance",
                                    name: "proposedInsurance",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowInsurance",
                                    name: "escrowInsurance",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Flood Insurance:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingFloodInsurance",
                                    name: "existingFloodInsurance",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedFloodInsurance",
                                    name: "proposedFloodInsurance",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowFloodInsurance",
                                    name: "escrowFloodInsurance",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "PMI:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingPMI",
                                    name: "existingPMI",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                div { class: "flex space-x-2",
                                    input {
                                        r#type: "number",
                                        id: "proposedPMI",
                                        name: "proposedPMI",
                                        class: "flex-1 px-2 py-1 border rounded",
                                    }
                                    input {
                                        r#type: "number",
                                        id: "proposedPMIamount",
                                        name: "proposedPMIamount",
                                        class: "flex-1 px-2 py-1 border rounded",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "HOA:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingHOA",
                                    name: "existingHOA",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedHOA",
                                    name: "proposedHOA",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Mortgage Payment:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingmortgagePayment",
                                    name: "existingmortgagePayment",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedmortgagePayment",
                                    name: "proposedmortgagePayment",
                                    value: "123456",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Overage/Shortage:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "overageShortage",
                                    name: "overageShortage",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Debt Paydown:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "debtPaydown",
                                    name: "debtPaydown",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Total Obligations:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "existingObligation",
                                    name: "existingObligation",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedObligation",
                                    name: "proposedObligation",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                    }
                }
            }
        }
    }
}