use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn Pension() -> Element {
    // State management
    let mut monthly_pension = use_signal(|| String::new());
    let mut taxable_portion = use_signal(|| String::from("100"));
    let mut pension_type = use_signal(|| String::from("defined_benefit"));
    let mut years_of_service = use_signal(|| String::new());
    let mut vesting_percentage = use_signal(|| String::from("100"));
    
    // Calculate gross monthly pension
    let gross_monthly = use_memo(move || {
        monthly_pension().parse::<f64>().unwrap_or(0.0)
    });
    
    // Calculate taxable portion
    let taxable_amount = use_memo(move || {
        let gross = gross_monthly();
        let tax_pct = taxable_portion().parse::<f64>().unwrap_or(100.0) / 100.0;
        gross * tax_pct
    });
    
    // Calculate non-taxable portion
    let non_taxable_amount = use_memo(move || {
        let gross = gross_monthly();
        let tax_pct = taxable_portion().parse::<f64>().unwrap_or(100.0) / 100.0;
        gross * (1.0 - tax_pct)
    });
    
    // Calculate Fannie Mae qualifying income (70/85 rule)
    let fannie_mae_income = use_memo(move || {
        let gross = gross_monthly();
        if gross <= 1000.0 {
            gross * 0.70 // 70% of first $1,000
        } else {
            let first_1000 = 1000.0 * 0.70;
            let remainder = gross - 1000.0;
            let remainder_qualified = remainder * 0.85; // 85% of amounts over $1,000
            first_1000 + remainder_qualified
        }
    });
    
    // Calculate annualized pension income
    let annualized_pension = use_memo(move || {
        gross_monthly() * 12.0
    });
    
    // Calculate vesting-adjusted income
    let vested_income = use_memo(move || {
        let annual = annualized_pension();
        let vesting_pct = vesting_percentage().parse::<f64>().unwrap_or(100.0) / 100.0;
        annual * vesting_pct
    });
    
    // Determine final qualifying income (lowest of Fannie Mae method or vested amount)
    let qualifying_income = use_memo(move || {
        let fannie_mae = fannie_mae_income();
        let vested = vested_income();
        fannie_mae.min(vested)
    });
    
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    
    rsx! {
        div { class: "space-y-8",
            // Pension Type Selection Section
            div { class: "bg-gradient-to-br from-emerald-50 to-teal-50 p-6 rounded-xl shadow-md border-2 border-emerald-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-emerald-600", "🏛️" }
                    "Pension Plan Information"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Pension Type"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 dark:text-gray-100 text-sm rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{pension_type}",
                                onchange: move |evt: Event<FormData>| pension_type.set(evt.value()),
                                option { value: "defined_benefit", "Defined Benefit Pension" }
                                option { value: "defined_contribution", "Defined Contribution (401k/403b)" }
                                option { value: "military", "Military Pension" }
                                option { value: "government", "Government Pension" }
                            }
                        }
                        div { class: "col-span-3 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Years of Service"
                            }
                            Input {
                                label: "",
                                r#type: "number",
                                name: "years_of_service",
                                value: "{years_of_service}",
                                placeholder: "0",
                                oninput: move |evt: Event<FormData>| years_of_service.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Vesting %"
                            }
                            Input {
                                label: "",
                                r#type: "number",
                                name: "vesting_percentage",
                                value: "{vesting_percentage}",
                                placeholder: "100",
                                oninput: move |evt: Event<FormData>| vesting_percentage.set(evt.value()),
                            }
                        }
                    }
                }
            }

            // Pension Amount Section
            div { class: "bg-gradient-to-br from-cyan-50 to-blue-50 p-6 rounded-xl shadow-md border-2 border-cyan-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-cyan-600", "💰" }
                    "Pension Income Details"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Monthly Pension Amount",
                                r#type: "number",
                                name: "monthly_pension",
                                value: "{monthly_pension}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| monthly_pension.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Taxable Portion (%)"
                            }
                            Input {
                                label: "",
                                r#type: "number",
                                name: "taxable_portion",
                                value: "{taxable_portion}",
                                placeholder: "100",
                                oninput: move |evt: Event<FormData>| taxable_portion.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Annualized Pension"
                            }
                            div { class: "px-4 py-3 bg-cyan-200 border-2 border-cyan-400 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(annualized_pension())}"
                            }
                        }
                    }

                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-blue-700", "ℹ️ Note: " }
                            "Enter the gross monthly pension amount before taxes. The taxable portion will be used for qualifying income calculations."
                        }
                    }
                }
            }

            // Tax Treatment Section
            div { class: "bg-gradient-to-br from-indigo-50 to-purple-50 p-6 rounded-xl shadow-md border-2 border-indigo-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-indigo-600", "📊" }
                    "Tax Treatment & Calculations"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-indigo-100 border-2 border-indigo-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Gross Monthly Pension"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(gross_monthly())}"
                            }
                        }
                    }

                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-indigo-100 border-2 border-indigo-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Taxable Amount"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(taxable_amount())}"
                            }
                        }
                    }

                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-indigo-100 border-2 border-indigo-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Non-Taxable Amount"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(non_taxable_amount())}"
                            }
                        }
                    }

                    div { class: "grid grid-cols-12 gap-4 items-center pt-4 border-t-2 border-indigo-300",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-red-200 border-2 border-red-400 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-center min-w-[200px]",
                                "Fannie Mae Qualifying Income"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-red-100 border-2 border-red-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right text-lg",
                                "{format_money(fannie_mae_income())}"
                            }
                        }
                    }
                }

                div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                    h4 { class: "font-bold text-yellow-800 mb-2", "Fannie Mae Pension Guidelines:" }
                    ul { class: "text-sm text-gray-900 dark:text-gray-100 space-y-1",
                        li { "• 70% of the first $1,000 of monthly pension income" }
                        li { "• 85% of amounts over $1,000" }
                        li { "• Use the lower of Fannie Mae calculation or vested amount" }
                        li { "• Taxable portion only (non-taxable portions excluded)" }
                        li { "• Must be documented and consistent for 12-24 months" }
                    }
                }
            }

            // Final Selection Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "✓" }
                    "Qualifying Pension Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Pension Type Selected"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold",
                                {
                                    match pension_type().as_str() {
                                        "defined_benefit" => "Defined Benefit Pension",
                                        "defined_contribution" => "Defined Contribution (401k/403b)",
                                        "military" => "Military Pension",
                                        "government" => "Government Pension",
                                        _ => "Not Selected",
                                    }
                                }
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Qualifying Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right text-xl",
                                "{format_money(qualifying_income() / 12.0)}"
                            }
                        }
                    }

                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        h4 { class: "font-bold text-blue-800 mb-2", "Documentation Required:" }
                        ul { class: "text-sm text-gray-900 dark:text-gray-100 space-y-1",
                            li { "• Pension award letter or benefit statement" }
                            li { "• Tax returns showing pension income reporting" }
                            li { "• Pension plan documents or summary plan description" }
                            li { "• Payment history (minimum 12-24 months)" }
                            li { "• Vesting and service documentation" }
                        }
                    }
                }
            }
        }
    }
}   	 