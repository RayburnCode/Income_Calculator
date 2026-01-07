use dioxus::prelude::*;
use shared::models::OtherFeesData;

#[component]
pub fn OtherFeesSection(data: OtherFeesData, on_change: EventHandler<OtherFeesData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Other Fees" }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "3rd Party Fee:" }
                    input {
                        r#type: "number",
                        name: "thirdPartyFees",
                        value: "{local_data().third_party_fees}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().third_party_fees = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Appraisal Fee:" }
                    input {
                        r#type: "number",
                        name: "appraisalFee",
                        value: "{local_data().appraisal_fee}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().appraisal_fee = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Investor Fee:" }
                    input {
                        r#type: "number",
                        name: "investorFee",
                        value: "{local_data().investor_fee}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().investor_fee = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Padded Taxes:" }
                    input {
                        r#type: "number",
                        name: "paymentPadTaxes",
                        value: "{local_data().padded_taxes}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().padded_taxes = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Padded Taxes Months:" }
                    input {
                        r#type: "number",
                        name: "paddedTaxesMonths",
                        value: "{local_data().padded_taxes_months}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                local_data.write().padded_taxes_months = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Padded Insurance:" }
                    input {
                        r#type: "number",
                        name: "paymentPadInsurance",
                        value: "{local_data().padded_insurance}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().padded_insurance = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Padded Insurance Months:"
                    }
                    input {
                        r#type: "number",
                        name: "paddedInsuranceMonths",
                        value: "{local_data().padded_insurance_months}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                local_data.write().padded_insurance_months = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Lender Credit:" }
                    input {
                        r#type: "number",
                        name: "lenderCredit",
                        value: "{local_data().lender_credit}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().lender_credit = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Admin:" }
                    input {
                        r#type: "number",
                        id: "adminFees",
                        value: "{local_data().admin_fees}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().admin_fees = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Tax Service:" }
                    input {
                        r#type: "number",
                        id: "taxService",
                        value: "{local_data().tax_service}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().tax_service = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Flood Cert:" }
                    input {
                        r#type: "number",
                        id: "floodCert",
                        value: "{local_data().flood_certification}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().flood_certification = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div { class: "md:col-span-2 lg:col-span-3 grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Total Closing Costs:"
                        }
                        input {
                            r#type: "number",
                            name: "closingCosts",
                            value: "{local_data().total_closing_costs}",
                            oninput: move |e| {
                                if let Ok(val) = e.value().parse::<f64>() {
                                    local_data.write().total_closing_costs = val;
                                    on_change.call(local_data());
                                }
                            },
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Cash Out (Bring to Close):"
                        }
                        input {
                            r#type: "number",
                            name: "cashOutAmount",
                            value: "{local_data().cash_out_amount}",
                            oninput: move |e| {
                                if let Ok(val) = e.value().parse::<f64>() {
                                    local_data.write().cash_out_amount = val;
                                    on_change.call(local_data());
                                }
                            },
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        }
                    }
                }
            }
        }
    }
}