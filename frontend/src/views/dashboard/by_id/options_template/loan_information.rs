use dioxus::prelude::*;

#[component]
pub fn LoanInformationSection() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4", "Loan Information" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Property"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Occupancy"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Loan"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Term (mo)"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Purpose"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Appraisal Waiver"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    name: "PropertyType",
                                    id: "PropertyType",
                                    class: "w-full px-2 py-1 border rounded",
                                    option { value: "sfr", "SFR" }
                                    option { value: "manufactured", "Manufactured" }
                                    option { value: "multiUnit", "Multi Unit" }
                                    option { value: "condo", "Condo" }
                                    option { value: "pud", "PUD" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    name: "occupancy",
                                    id: "occupancy",
                                    class: "w-full px-2 py-1 border rounded",
                                    option { value: "primary", "Primary" }
                                    option { value: "secondary", "Secondary" }
                                    option { value: "investment", "Investment" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    name: "loanType",
                                    id: "loanType",
                                    class: "w-full px-2 py-1 border rounded",
                                    option { value: "cnv", "CNV" }
                                    option { value: "fha", "FHA" }
                                    option { value: "va", "VA" }
                                    option { value: "nonQM", "Non-QM" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "newTerm",
                                    id: "newTerm",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    name: "loanPurpose",
                                    id: "loanPurpose",
                                    class: "w-full px-2 py-1 border rounded",
                                    option { value: "purchase", "Purchase" }
                                    option { value: "cashOut", "Cash Out" }
                                    option { value: "refinance", "Refinance" }
                                    option { value: "irrrl", "IRRRL/Streamline" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "appraisalWaiver",
                                    id: "appraisalWaiver",
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