use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn IRA() -> Element {
    // State management
    let mut monthly_distribution = use_signal(|| String::new());
    let mut distribution_frequency = use_signal(|| String::from("monthly"));
    let mut ira_type = use_signal(|| String::from("qualified"));
    let mut account_balance = use_signal(|| String::new());
    let years_until_depletion = use_signal(|| String::from("10"));
    
    // Calculate annualized income based on distribution frequency
    let annualized_distribution = use_memo(move || {
        if let Ok(monthly) = monthly_distribution().parse::<f64>() {
            match distribution_frequency().as_str() {
                "monthly" => monthly * 12.0,
                "quarterly" => monthly * 4.0,
                "semi-annually" => monthly * 2.0,
                "annually" => monthly,
                _ => monthly * 12.0,
            }
        } else {
            0.0
        }
    });
    
    // Calculate sustainable withdrawal rate (4% rule)
    let sustainable_income = use_memo(move || {
        if let Ok(balance) = account_balance().parse::<f64>() {
            balance * 0.04
        } else {
            0.0
        }
    });
    
    // Calculate depletion-based income
    let depletion_income = use_memo(move || {
        if let Ok(balance) = account_balance().parse::<f64>() {
            if let Ok(years) = years_until_depletion().parse::<f64>() {
                if years > 0.0 {
                    balance / years
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        }
    });
    
    // Calculate qualified plan income (Fannie Mae method)
    let qualified_income = use_memo(move || {
        let dist_annual = annualized_distribution();
        let sustainable = sustainable_income();
        let depletion = depletion_income();
        
        let mut values = vec![dist_annual, sustainable];
        if depletion > 0.0 {
            values.push(depletion);
        }
        values.into_iter().filter(|&x| x > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
    });
    
    // Calculate non-qualified income (Roth IRA)
    let non_qualified_income = use_memo(move || {
        annualized_distribution()
    });
    
    // Determine final qualifying income
    let qualifying_income = use_memo(move || {
        match ira_type().as_str() {
            "qualified" => qualified_income(),
            "non-qualified" => non_qualified_income(),
            _ => 0.0,
        }
    });
    
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    
    rsx! {
        div { class: "space-y-8",
            // IRA Type Selection Section
            div { class: "bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "🏦" }
                    "IRA Account Information"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "IRA Type"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 dark:text-gray-100 text-sm rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{ira_type}",
                                onchange: move |evt: Event<FormData>| ira_type.set(evt.value()),
                                option { value: "qualified", "Qualified IRA (Traditional, SEP, SIMPLE)" }
                                option { value: "non-qualified", "Non-Qualified (Roth IRA)" }
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Current Account Balance"
                            }
                            Input {
                                label: "",
                                r#type: "number",
                                name: "account_balance",
                                value: "{account_balance}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| account_balance.set(evt.value()),
                            }
                        }
                    }
                }
            }

            // Distribution Information Section
            div { class: "bg-gradient-to-br from-purple-50 to-pink-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "💰" }
                    "Distribution Information"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Monthly Distribution Amount",
                                r#type: "number",
                                name: "monthly_distribution",
                                value: "{monthly_distribution}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| monthly_distribution.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Distribution Frequency"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 dark:text-gray-100 text-sm rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{distribution_frequency}",
                                onchange: move |evt: Event<FormData>| distribution_frequency.set(evt.value()),
                                option { value: "monthly", "Monthly" }
                                option { value: "quarterly", "Quarterly" }
                                option { value: "semi-annually", "Semi-Annually" }
                                option { value: "annually", "Annually" }
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "Annualized Distribution"
                            }
                            div { class: "px-4 py-3 bg-purple-200 border-2 border-purple-400 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(annualized_distribution())}"
                            }
                        }
                    }

                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-blue-700", "ℹ️ Note: " }
                            "Enter the gross distribution amount before taxes. For qualified plans, taxes will be deducted. For Roth IRAs, distributions are typically tax-free."
                        }
                    }
                }
            }

            // Fannie Mae Calculation Methods Section
            div { class: "bg-gradient-to-br from-orange-50 to-amber-50 p-6 rounded-xl shadow-md border-2 border-orange-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-orange-600", "📊" }
                    "Fannie Mae Calculation Methods"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Distribution Method"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(annualized_distribution())}"
                            }
                        }
                    }

                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "4% Sustainable Withdrawal"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                "{format_money(sustainable_income())}"
                            }
                        }
                    }

                    if depletion_income() > 0.0 {
                        div { class: "grid grid-cols-12 gap-4 items-center",
                            div { class: "col-span-6 flex items-center gap-3",
                                div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                    "Depletion Method"
                                }
                            }
                            div { class: "col-span-6 flex flex-col",
                                div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right",
                                    "{format_money(depletion_income())}"
                                }
                            }
                        }
                    }

                    div { class: "grid grid-cols-12 gap-4 items-center pt-4 border-t-2 border-orange-300",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-red-200 border-2 border-red-400 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-center min-w-[200px]",
                                "Lowest Qualifying Amount"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-red-100 border-2 border-red-300 rounded-lg text-gray-900 dark:text-gray-100 font-bold text-right text-lg",
                                "{format_money(qualifying_income())}"
                            }
                        }
                    }
                }

                div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                    h4 { class: "font-bold text-yellow-800 mb-2", "Fannie Mae Guidelines:" }
                    ul { class: "text-sm text-gray-900 dark:text-gray-100 space-y-1",
                        li { "• Use the lowest calculated amount for qualifying income" }
                        li { "• Qualified plans: Income is taxable (taxes will be deducted)" }
                        li { "• Roth IRAs: Income is tax-free (no tax deduction needed)" }
                        li { "• Distributions must be documented and consistent" }
                        li { "• 4% rule provides sustainable withdrawal rate" }
                    }
                }
            }

            // Final Selection Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "✓" }
                    "Qualifying IRA Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                                "IRA Type Selected"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 font-semibold",
                                {
                                    match ira_type().as_str() {
                                        "qualified" => "Qualified IRA (Taxable Income)",
                                        "non-qualified" => "Roth IRA (Tax-Free Income)",
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
                            li { "• Most recent account statement showing balance and distributions" }
                            li { "• Tax returns showing IRA contributions/deductions" }
                            li { "• Distribution history (minimum 12-24 months)" }
                            li { "• IRA plan documents or custodian statements" }
                        }
                    }
                }
            }
        }
    }
}