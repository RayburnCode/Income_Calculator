use dioxus::prelude::*;

#[component]
pub fn NewLoanSection() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4", "New Loan" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    table { class: "w-full border-collapse border border-gray-300",
                        tbody {
                            tr {
                                td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                    "Market Value:"
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "marketValue",
                                        id: "marketValue",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "Sales Price:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "salesPrice",
                                        id: "salesPrice",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "Down Payment:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "downPayment",
                                        id: "downPayment",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                    "Base Loan Amount:"
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "baseLoanAmount",
                                        id: "baseLoanAmount",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "Subordinated:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "subordinated",
                                        id: "subordinatedAmount",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "FF/UMIP:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    div { class: "flex",
                                        input {
                                            r#type: "number",
                                            name: "ffUmip",
                                            id: "ffUmip",
                                            class: "flex-1 px-2 py-1 border rounded",
                                        }
                                        span { class: "ml-2 self-center", "%" }
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "UMIP Refund:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "umipRefund",
                                        id: "umipRefund",
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "Total Loan Amount:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "text",
                                        name: "totalLoanAmount",
                                        id: "totalLoanAmount",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            tr {
                                td { class: "border border-gray-300 px-4 py-2", "Note Rate:" }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        name: "noteRate",
                                        id: "noteRate",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "md:ml-6",
                    div { class: "mb-4",
                        label {
                            r#for: "LTV",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "LTV"
                        }
                        input {
                            r#type: "number",
                            value: "0",
                            id: "LTVvalue",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50",
                        }
                    }
                    div {
                        label {
                            r#for: "CLTV",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "CLTV"
                        }
                        input {
                            r#type: "number",
                            value: "0",
                            id: "CLTVvalue",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50",
                        }
                    }
                }
            }
        }
    }
}