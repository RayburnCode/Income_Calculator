use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn SocialSecurity() -> Element {
    // State management - With Documentation
    let mut annual_benefit = use_signal(|| String::new());
    let mut taxable_portion = use_signal(|| String::new());
    let mut non_taxable = use_signal(|| String::new());
    let mut months_with_doc = use_signal(|| String::from("12"));
    
    // State management - Without Taxation Documentation
    let mut total_annual_benefit = use_signal(|| String::new());
    let mut months_without_doc = use_signal(|| String::from("12"));
    
    // Calculate With Documentation
    let taxable_income = use_memo(move || {
        if let Ok(taxable) = taxable_portion().parse::<f64>() {
            taxable * 1.0 // x 100%
        } else {
            0.0
        }
    });
    
    let non_taxable_income = use_memo(move || {
        if let Ok(non_tax) = non_taxable().parse::<f64>() {
            non_tax * 1.25 // x 125%
        } else {
            0.0
        }
    });
    
    let total_income_with_doc = use_memo(move || {
        taxable_income() + non_taxable_income()
    });
    
    let monthly_income_with_doc = use_memo(move || {
        if let Ok(months) = months_with_doc().parse::<f64>() {
            if months > 0.0 {
                total_income_with_doc() / months
            } else {
                0.0
            }
        } else {
            0.0
        }
    });
    
    // Calculate Without Documentation
    let annual_benefit_85 = use_memo(move || {
        if let Ok(benefit) = total_annual_benefit().parse::<f64>() {
            benefit * 0.85 // Annual Benefit x 85%
        } else {
            0.0
        }
    });
    
    let annual_benefit_15 = use_memo(move || {
        if let Ok(benefit) = total_annual_benefit().parse::<f64>() {
            benefit * 0.15 // Annual Benefit x 15%
        } else {
            0.0
        }
    });
    
    let gross_up_25 = use_memo(move || {
        annual_benefit_15() * 1.25 // Gross up 25%
    });
    
    let total_gross_up = use_memo(move || {
        annual_benefit_85() + gross_up_25()
    });
    
    let monthly_income_without_doc = use_memo(move || {
        if let Ok(months) = months_without_doc().parse::<f64>() {
            if months > 0.0 {
                total_gross_up() / months
            } else {
                0.0
            }
        } else {
            0.0
        }
    });
    
    // Helper to format money
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    
    rsx! {
        div { class: "space-y-8",
            // With Documentation Section
            div { class: "bg-gradient-to-br from-blue-50 to-indigo-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "📋" }
                    "With Documentation"
                }
                div { class: "space-y-4",
                    // Annual Benefit
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Annual Benefit",
                                r#type: "number",
                                name: "annual_benefit",
                                value: "{annual_benefit}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| annual_benefit.set(evt.value()),
                            }
                        }
                    }
                    // Taxable and Non-Taxable
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Taxable Portion",
                                r#type: "number",
                                name: "taxable_portion",
                                value: "{taxable_portion}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| taxable_portion.set(evt.value()),
                            }
                        }
                        div { class: "col-span-2 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Multiplier"
                            }
                            div { class: "px-4 py-3 bg-blue-100 border-2 border-blue-300 rounded-lg text-gray-900 font-semibold text-center",
                                "× 100%"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Result"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(taxable_income())}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Non-Taxable",
                                r#type: "number",
                                name: "non_taxable",
                                value: "{non_taxable}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| non_taxable.set(evt.value()),
                            }
                        }
                        div { class: "col-span-2 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Multiplier"
                            }
                            div { class: "px-4 py-3 bg-blue-100 border-2 border-blue-300 rounded-lg text-gray-900 font-semibold text-center",
                                "× 125%"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Result"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(non_taxable_income())}"
                            }
                        }
                    }
                    // Total Income
                    div { class: "grid grid-cols-12 gap-4 items-end pt-4 border-t-2 border-blue-300",
                        div { class: "col-span-4",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "months_with_doc",
                                value: "{months_with_doc}",
                                oninput: move |evt: Event<FormData>| months_with_doc.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Total Annual Income"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 font-bold text-right text-lg",
                                "{format_money(total_income_with_doc())}"
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Total Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right text-lg",
                                "{format_money(monthly_income_with_doc())}"
                            }
                        }
                    }
                }
            }
            // Without Taxation Documentation Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "📊" }
                    "Without Taxation Documentation"
                }
                div { class: "space-y-4",
                    // Total Annual Benefit
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Total Annual Benefit",
                                r#type: "number",
                                name: "total_annual_benefit",
                                value: "{total_annual_benefit}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| total_annual_benefit.set(evt.value()),
                            }
                        }
                    }
                    // 85% Calculation
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Annual Benefit × 85%"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(annual_benefit_85())}"
                            }
                        }
                    }
                    // 15% Calculation
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Annual Benefit × 15%"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(annual_benefit_15())}"
                            }
                        }
                    }
                    // Gross up 25%
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Gross up 25%"
                            }
                            div { class: "px-4 py-3 bg-green-100 border-2 border-green-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(gross_up_25())}"
                            }
                        }
                    }
                    // Total and Monthly Income
                    div { class: "grid grid-cols-12 gap-4 items-end pt-4 border-t-2 border-green-300",
                        div { class: "col-span-4",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "months_without_doc",
                                value: "{months_without_doc}",
                                oninput: move |evt: Event<FormData>| months_without_doc.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Total Gross Up"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right text-lg",
                                "{format_money(total_gross_up())}"
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Total Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 font-bold text-right text-lg",
                                "{format_money(monthly_income_without_doc())}"
                            }
                        }
                    }
                }
            }
        }
    }
}