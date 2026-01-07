use dioxus::prelude::*;

#[component]
pub fn NewLoanSection() -> Element {
    rsx! {
        div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "New Loan" }

            // Main form grid - stacks on mobile, side-by-side on larger screens
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Left column - Loan details form
                div { class: "lg:col-span-2",
                    div { class: "space-y-4",
                        // Market Value
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "marketValue",
                                class: "text-sm font-semibold text-gray-700 sm:col-span-1",
                                "Market Value:"
                            }
                            input {
                                r#type: "number",
                                name: "marketValue",
                                id: "marketValue",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter market value",
                            }
                        }

                        // Sales Price
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "salesPrice",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "Sales Price:"
                            }
                            input {
                                r#type: "number",
                                name: "salesPrice",
                                id: "salesPrice",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter sales price",
                            }
                        }

                        // Down Payment
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "downPayment",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "Down Payment:"
                            }
                            input {
                                r#type: "number",
                                name: "downPayment",
                                id: "downPayment",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter down payment",
                            }
                        }

                        // Base Loan Amount
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "baseLoanAmount",
                                class: "text-sm font-semibold text-gray-700 sm:col-span-1",
                                "Base Loan Amount:"
                            }
                            input {
                                r#type: "number",
                                name: "baseLoanAmount",
                                id: "baseLoanAmount",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter base loan amount",
                            }
                        }

                        // Subordinated
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "subordinated",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "Subordinated:"
                            }
                            input {
                                r#type: "number",
                                name: "subordinatedAmount",
                                id: "subordinatedAmount",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter subordinated amount",
                            }
                        }

                        // FF/UMIP
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "ffUmip",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "FF/UMIP:"
                            }
                            div { class: "sm:col-span-2 flex",
                                input {
                                    r#type: "number",
                                    name: "ffUmip",
                                    id: "ffUmip",
                                    class: "flex-1 px-3 py-2 border border-gray-300 rounded-l-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                    placeholder: "Enter FF/UMIP",
                                }
                                span { class: "inline-flex items-center px-3 py-2 border border-l-0 border-gray-300 bg-gray-50 rounded-r-md text-sm text-gray-500",
                                    "%"
                                }
                            }
                        }

                        // UMIP Refund
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "umipRefund",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "UMIP Refund:"
                            }
                            input {
                                r#type: "number",
                                name: "umipRefund",
                                id: "umipRefund",
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                                placeholder: "Enter UMIP refund",
                            }
                        }

                        // Total Loan Amount (readonly)
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "totalLoanAmount",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "Total Loan Amount:"
                            }
                            input {
                                r#type: "text",
                                name: "totalLoanAmount",
                                id: "totalLoanAmount",
                                readonly: true,
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-700",
                                placeholder: "Calculated automatically",
                            }
                        }

                        // Note Rate (readonly)
                        div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 items-center",
                            label {
                                r#for: "noteRate",
                                class: "text-sm font-medium text-gray-700 sm:col-span-1",
                                "Note Rate:"
                            }
                            input {
                                r#type: "number",
                                name: "noteRate",
                                id: "noteRate",
                                readonly: true,
                                class: "sm:col-span-2 px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-700",
                                placeholder: "Calculated automatically",
                            }
                        }
                    }
                }

                // Right column - Calculated values
                div { class: "space-y-4",
                    // LTV
                    div {
                        label {
                            r#for: "LTV",
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "LTV"
                        }
                        input {
                            r#type: "number",
                            value: "0",
                            id: "LTVvalue",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-700",
                        }
                    }

                    // CLTV
                    div {
                        label {
                            r#for: "CLTV",
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "CLTV"
                        }
                        input {
                            r#type: "number",
                            value: "0",
                            id: "CLTVvalue",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-700",
                        }
                    }
                }
            }
        }
    }
}