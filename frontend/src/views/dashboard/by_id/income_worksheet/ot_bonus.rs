use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn OTBonus() -> Element {
    // State management
    let mut ytd_ot_bonus = use_signal(|| String::new());
    let mut ytd_months = use_signal(|| String::from("12"));
    let mut past_year_ot = use_signal(|| String::new());
    let mut past_year_months = use_signal(|| String::from("12"));
    let mut additional_year_ot = use_signal(|| String::new());
    let mut additional_year_months = use_signal(|| String::from("12"));
    let mut selected_calculation = use_signal(|| String::from("none"));
    
    // Calculate YTD OT/Bonus Income
    let ytd_income = use_memo(move || {
        if let Ok(ytd) = ytd_ot_bonus().parse::<f64>() {
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
    
    // Calculate Past Year OT Income
    let past_year_income = use_memo(move || {
        if let Ok(past) = past_year_ot().parse::<f64>() {
            if let Ok(months) = past_year_months().parse::<f64>() {
                if months > 0.0 {
                    past / months * 12.0
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
    
    // Calculate Additional Year OT Income
    let additional_year_income = use_memo(move || {
        if let Ok(additional) = additional_year_ot().parse::<f64>() {
            if let Ok(months) = additional_year_months().parse::<f64>() {
                if months > 0.0 {
                    additional / months * 12.0
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
    
    let ytd_plus_1year_avg = use_memo(move || {
        (ytd_income() + past_year_income()) / 2.0
    });
    
    let ytd_plus_2year_avg = use_memo(move || {
        (ytd_income() + past_year_income() + additional_year_income()) / 3.0
    });
    
    // Find lowest calculation
    let lowest_calculation = use_memo(move || {
        let mut values = vec![ytd_avg(), ytd_plus_1year_avg(), ytd_plus_2year_avg()];
        values.retain(|&x| x > 0.0);
        values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    });
    
    // Determine final income based on selection
    let final_income = use_memo(move || {
        match selected_calculation().as_str() {
            "ytd" => ytd_avg(),
            "ytd_1year" => ytd_plus_1year_avg(),
            "ytd_2year" => ytd_plus_2year_avg(),
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
            // Income Inputs Section
            div { class: "bg-gradient-to-br from-orange-50 to-amber-50 p-6 rounded-xl shadow-md border-2 border-orange-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-orange-600", "💼" }
                    "Overtime & Bonus Income"
                }
                div { class: "space-y-4",
                    // YTD OT/Bonus
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "YTD Overtime / Bonus*",
                                r#type: "number",
                                name: "ytd_ot_bonus",
                                value: "{ytd_ot_bonus}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| ytd_ot_bonus.set(evt.value()),
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
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(ytd_income())}"
                            }
                        }
                    }
                    // Past Year OT
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Past Year OT Breakout",
                                r#type: "number",
                                name: "past_year_ot",
                                value: "{past_year_ot}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| past_year_ot.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "past_year_months",
                                value: "{past_year_months}",
                                oninput: move |evt: Event<FormData>| past_year_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-5 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(past_year_income())}"
                            }
                        }
                    }
                    // Additional Year OT
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Additional Year OT / Bonus",
                                r#type: "number",
                                name: "additional_year_ot",
                                value: "{additional_year_ot}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| additional_year_ot.set(evt.value()),
                            }
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "# Months",
                                r#type: "number",
                                name: "additional_year_months",
                                value: "{additional_year_months}",
                                oninput: move |evt: Event<FormData>| additional_year_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-5 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Annual Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(additional_year_income())}"
                            }
                        }
                    }
                    // Important Note
                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-blue-700", "ℹ️ Note: " }
                            "*If DU requires only a YTD paystub, OT/Bonus must be annualized. Divide YTD OT/Bonus by 12 months; for qualifying purposes, typically a two year history of receipt is required."
                        }
                    }
                }
            }
            // Calculations Section
            div { class: "bg-gradient-to-br from-purple-50 to-violet-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "📊" }
                    "Calculation Methods"
                }
                div { class: "space-y-4",
                    // YTD Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-purple-100 border-2 border-purple-300 rounded-lg text-gray-900 font-semibold text-center min-w-[200px]",
                                "YTD Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(ytd_avg())}"
                            }
                        }
                    }
                    // YTD + 1 Year Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-purple-100 border-2 border-purple-300 rounded-lg text-gray-900 font-semibold text-center min-w-[200px]",
                                "YTD + 1 Year Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(ytd_plus_1year_avg())}"
                            }
                        }
                    }
                    // YTD + 2 Year Average
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-purple-100 border-2 border-purple-300 rounded-lg text-gray-900 font-semibold text-center min-w-[200px]",
                                "YTD + 2 Year Average"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(ytd_plus_2year_avg())}"
                            }
                        }
                    }
                    // Lowest Calculation
                    div { class: "grid grid-cols-12 gap-4 items-center pt-4 border-t-2 border-purple-300",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-indigo-200 border-2 border-indigo-400 rounded-lg text-gray-900 font-bold text-center min-w-[200px]",
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
                                    div { class: "px-4 py-3 bg-indigo-100 border-2 border-indigo-300 rounded-lg text-gray-900 font-bold text-right text-lg",
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
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "✓" }
                    "Income to Use for Qualifying"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Select Calculation Method"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{selected_calculation}",
                                onchange: move |evt: Event<FormData>| selected_calculation.set(evt.value()),
                                option { value: "none", "-- Select Method --" }
                                option { value: "ytd", "YTD Average ({format_money(ytd_avg())})" }
                                option { value: "ytd_1year",
                                    "YTD + 1 Year Avg ({format_money(ytd_plus_1year_avg())})"
                                }
                                option { value: "ytd_2year",
                                    "YTD + 2 Year Avg ({format_money(ytd_plus_2year_avg())})"
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
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Income"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right text-xl",
                                "{format_money(final_income())}"
                            }
                        }
                    }
                }
            }
        }
    }
}   
									
									