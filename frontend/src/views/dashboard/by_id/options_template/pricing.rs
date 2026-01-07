use dioxus::prelude::*;
use shared::models::PricingData;

#[component]
pub fn PricingSection(data: PricingData, total_loan_amount: f64, on_change: EventHandler<PricingData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    let pricing_options = local_data().pricing_options.clone();
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Pricing" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Description"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Rate"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "YSP %"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "YSP $"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "BD %"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "BD $"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Select"
                            }
                        }
                    }
                    tbody {
                        // Render pricing options dynamically
                        {pricing_options.iter().enumerate().map(|(i, option)| rsx! {
                            tr {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "text",
                                        value: "{option.description}",
                                        oninput: move |e| {
                                            local_data.write().pricing_options[i].description = e.value();
                                            on_change(local_data());
                                        },
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        value: "{option.note_rate}",
                                        oninput: move |e| {
                                            if let Ok(val) = e.value().parse::<f64>() {
                                                local_data.write().pricing_options[i].note_rate = val;
                                                on_change(local_data());
                                            }
                                        },
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        value: "{option.ysp_percentage}",
                                        oninput: move |e| {
                                            if let Ok(val) = e.value().parse::<f64>() {
                                                local_data.write().pricing_options[i].ysp_percentage = val;
                                                local_data.write().pricing_options[i].ysp_dollar = val * total_loan_amount
                                                    / 100.0;
                                                on_change(local_data());
                                            }
                                        },
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        value: "{option.ysp_percentage * total_loan_amount / 100.0:.2}",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        value: "{option.bd_percentage}",
                                        oninput: move |e| {
                                            if let Ok(val) = e.value().parse::<f64>() {
                                                local_data.write().pricing_options[i].bd_percentage = val;
                                                local_data.write().pricing_options[i].bd_dollar = val * total_loan_amount
                                                    / 100.0;
                                                on_change(local_data());
                                            }
                                        },
                                        class: "w-full px-2 py-1 border rounded",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        value: "{option.bd_percentage * total_loan_amount / 100.0:.2}",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                                td { class: "border border-gray-300 px-4 py-2 text-center",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{option.is_selected}",
                                        onchange: move |e| {
                                            local_data.write().pricing_options[i].is_selected = e.checked();
                                            on_change(local_data());
                                        },
                                        class: "w-4 h-4",
                                    }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}