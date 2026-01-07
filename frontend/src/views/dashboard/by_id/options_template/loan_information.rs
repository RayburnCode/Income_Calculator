use dioxus::prelude::*;
use shared::models::LoanInformationData;

#[component]
pub fn LoanInformationSection(data: LoanInformationData, on_change: EventHandler<LoanInformationData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    // Helper function to update data and trigger on_change
    let mut update_data = move |field: &str, value: String| {
        let mut new_data = local_data();
        match field {
            "property_type" => new_data.property_type = value,
            "occupancy" => new_data.occupancy = value,
            "loan_type" => new_data.loan_type = value,
            "term_months" => new_data.term_months = value.parse().unwrap_or(360),
            "purpose" => new_data.purpose = value,
            "appraisal_waiver" => new_data.appraisal_waiver = value == "true",
            _ => {}
        }
        local_data.set(new_data.clone());
        on_change.call(new_data);
    };

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Loan Information" }
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
                                    value: "{local_data().property_type}",
                                    class: "w-full px-2 py-1 border rounded",
                                    onchange: move |evt: Event<FormData>| update_data("property_type", evt.value()),
                                    option { value: "sfr", "SFR" }
                                    option { value: "manufactured", "Manufactured" }
                                    option { value: "multiUnit", "Multi Unit" }
                                    option { value: "condo", "Condo" }
                                    option { value: "pud", "PUD" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    value: "{local_data().occupancy}",
                                    class: "w-full px-2 py-1 border rounded",
                                    onchange: move |evt: Event<FormData>| update_data("occupancy", evt.value()),
                                    option { value: "primary", "Primary" }
                                    option { value: "secondary", "Secondary" }
                                    option { value: "investment", "Investment" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    value: "{local_data().loan_type}",
                                    class: "w-full px-2 py-1 border rounded",
                                    onchange: move |evt: Event<FormData>| update_data("loan_type", evt.value()),
                                    option { value: "cnv", "CNV" }
                                    option { value: "fha", "FHA" }
                                    option { value: "va", "VA" }
                                    option { value: "nonQM", "Non-QM" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().term_months}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("term_months", evt.value()),
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                select {
                                    value: "{local_data().purpose}",
                                    class: "w-full px-2 py-1 border rounded",
                                    onchange: move |evt: Event<FormData>| update_data("purpose", evt.value()),
                                    option { value: "purchase", "Purchase" }
                                    option { value: "cashOut", "Cash Out" }
                                    option { value: "refinance", "Refinance" }
                                    option { value: "irrrl", "IRRRL/Streamline" }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    checked: local_data().appraisal_waiver,
                                    class: "w-4 h-4",
                                    onchange: move |evt: Event<FormData>| {
                                        let value = if evt.checked() { "true" } else { "false" };
                                        update_data("appraisal_waiver", value.to_string());
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}