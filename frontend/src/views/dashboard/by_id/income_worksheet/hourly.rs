use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn Hourly() -> Element {
    // State management
    let mut per_hour = use_signal(|| String::new());
    let mut hours_worked = use_signal(|| String::from("40"));
    let mut ytd_earnings = use_signal(|| String::new());
    let mut ytd_months = use_signal(|| String::from("12"));
    let mut w2_year1 = use_signal(|| String::new());
    let mut w2_year1_months = use_signal(|| String::from("12"));
    let mut w2_year2 = use_signal(|| String::new());
    let mut w2_year2_months = use_signal(|| String::from("12"));
    let mut selected_calculation = use_signal(|| String::from("none"));
    
    // Calculate per hour income (hourly rate × hours × 52 weeks / 12 months)
    let per_hour_income = use_memo(move || {
        if let Ok(rate) = per_hour().parse::<f64>() {
            if let Ok(hours) = hours_worked().parse::<f64>() {
                rate * hours * 52.0 / 12.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    });

    // Calculate YTD income (YTD earnings / months × 12)
    let ytd_income = use_memo(move || {
        if let Ok(ytd) = ytd_earnings().parse::<f64>() {
            if let Ok(months) = ytd_months().parse::<f64>() {
                if months > 0.0 {
                    ytd / months * 12.0
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

    // Calculate W2 Year 1 income
    let w2_year1_income = use_memo(move || {
        if let Ok(w2) = w2_year1().parse::<f64>() {
            if let Ok(months) = w2_year1_months().parse::<f64>() {
                if months > 0.0 {
                    w2 / months * 12.0
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

    // Calculate W2 Year 2 income
    let w2_year2_income = use_memo(move || {
        if let Ok(w2) = w2_year2().parse::<f64>() {
            if let Ok(months) = w2_year2_months().parse::<f64>() {
                if months > 0.0 {
                    w2 / months * 12.0
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

    // Calculate averages
    let ytd_avg = use_memo(move || {
        ytd_income()
    });

    let ytd_plus_1w2_avg = use_memo(move || {
        (ytd_income() + w2_year1_income()) / 2.0
    });

    let ytd_plus_2w2_avg = use_memo(move || {
        (ytd_income() + w2_year1_income() + w2_year2_income()) / 3.0
    });

    // Find lowest calculation
    let lowest_calculation = use_memo(move || {
        let mut values = vec![ytd_avg(), ytd_plus_1w2_avg(), ytd_plus_2w2_avg()];
        values.retain(|&x| x > 0.0);
        values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    });

    // Determine final income based on selection
    let final_income = use_memo(move || {
        match selected_calculation().as_str() {
            "current" => per_hour_income(),
            "ytd" => ytd_avg(),
            "ytd_1w2" => ytd_plus_1w2_avg(),
            "ytd_2w2" => ytd_plus_2w2_avg(),
            "lowest" => if lowest_calculation().is_finite() { lowest_calculation() } else { 0.0 },
            _ => 0.0,
        }
    });

    // Helper to format money
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    rsx! {
        div { class: "space-y-8",
            // Current Hourly Rate Section
            div { class: "bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "⏰" }
                    "Current Hourly Rate"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Hourly Rate",
                                r#type: "number",
                                name: "per_hour",
                                value: "{per_hour}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| per_hour.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4",
                            Input {
                                label: "Hours per Week",
                                r#type: "number",
                                name: "hours_worked",
                                value: "{hours_worked}",
                                placeholder: "40",
                                oninput: move |evt: Event<FormData>| hours_worked.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(per_hour_income())}"
                            }
                        }
                    }
                    // Calculation note
                    div { class: "mt-4 p-3 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-blue-700", "Formula: " }
                            "Hourly Rate × Hours per Week × 52 weeks ÷ 12 months"
                        }
                    }
                }
            }

            // Historical Income Section
            div { class: "bg-gradient-to-br from-purple-50 to-pink-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "📊" }
                    "Historical Income"
                }
                div { class: "space-y-4",
                    // YTD Earnings
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "YTD Earnings (Paystub)",
                                r#type: "number",
                                name: "ytd_earnings",
                                value: "{ytd_earnings}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| ytd_earnings.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "ytd_months",
                                value: "{ytd_months}",
                                oninput: move |evt: Event<FormData>| ytd_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-5 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(ytd_income())}"
                            }
                        }
                    }

                    // W2 Year 1
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "W2 Income (Year 1)",
                                r#type: "number",
                                name: "w2_year1",
                                value: "{w2_year1}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| w2_year1.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "w2_year1_months",
                                value: "{w2_year1_months}",
                                oninput: move |evt: Event<FormData>| w2_year1_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-5 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(w2_year1_income())}"
                            }
                        }
                    }

                    // W2 Year 2
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "W2 Income (Year 2)",
                                r#type: "number",
                                name: "w2_year2",
                                value: "{w2_year2}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| w2_year2.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "w2_year2_months",
                                value: "{w2_year2_months}",
                                oninput: move |evt: Event<FormData>| w2_year2_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-5 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(w2_year2_income())}"
                            }
                        }
                    }
                }
            }

            // Calculation Methods Section
            div { class: "bg-gradient-to-br from-orange-50 to-amber-50 p-6 rounded-xl shadow-md border-2 border-orange-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-orange-600", "🧮" }
                    "Calculation Methods"
                }
                div { class: "space-y-4",
                    // Current Hourly
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Current Hourly Rate"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(per_hour_income())}"
                            }
                        }
                    }

                    // YTD Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "YTD Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(ytd_avg())}"
                            }
                        }
                    }

                    // YTD + 1 W2 Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "YTD + 1 W2 Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(ytd_plus_1w2_avg())}"
                            }
                        }
                    }

                    // YTD + 2 W2 Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-orange-100 border-2 border-orange-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "YTD + 2 W2 Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(ytd_plus_2w2_avg())}"
                            }
                        }
                    }

                    // Lowest Calculation
                    div { class: "grid grid-cols-12 gap-4 items-center pt-4 border-t-2 border-orange-300",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-red-200 border-2 border-red-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-center min-w-[200px]",
                                "Lowest of Calculations"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            {
                                let lowest_val = if lowest_calculation().is_finite() {
                                    lowest_calculation()
                                } else {
                                    0.0
                                };
                                rsx! {
                                    div { class: "px-4 py-3 bg-red-100 border-2 border-red-300 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right text-lg",
                                        "{format_money(lowest_val)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Final Selection Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "✓" }
                    "Income to Use for Qualifying"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Select Calculation Method"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 dark:text-gray-100 dark:text-gray-100 text-sm rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{selected_calculation}",
                                onchange: move |evt: Event<FormData>| selected_calculation.set(evt.value()),
                                option { value: "none", "-- Select Method --" }
                                option { value: "current",
                                    "Current Hourly ({format_money(per_hour_income())})"
                                }
                                option { value: "ytd", "YTD Average ({format_money(ytd_avg())})" }
                                option { value: "ytd_1w2",
                                    "YTD + 1 W2 Avg ({format_money(ytd_plus_1w2_avg())})"
                                }
                                option { value: "ytd_2w2",
                                    "YTD + 2 W2 Avg ({format_money(ytd_plus_2w2_avg())})"
                                }
                                {
                                    let lowest_val = if lowest_calculation().is_finite() {
                                        lowest_calculation()
                                    } else {
                                        0.0
                                    };
                                    rsx! {
                                        option { value: "lowest", "Use Lowest ({format_money(lowest_val)})" }
                                    }
                                }
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Income"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right text-xl",
                                "{format_money(final_income())}"
                            }
                        }
                    }

                    // Note about requirements
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "⚠️ Note: " }
                            "If YTD or past year is lower, confirm why. Otherwise, lower of YTD and W2 required."
                        }
                    }
                }
            }
        }
    }
}   	