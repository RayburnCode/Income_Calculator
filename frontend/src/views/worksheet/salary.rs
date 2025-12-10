use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn Salary() -> Element {
    // State management
    let mut salary_amount = use_signal(|| String::new());
    let mut pay_frequency = use_signal(|| String::from("monthly"));
    let mut ytd_salary = use_signal(|| String::new());
    let mut ytd_months = use_signal(|| String::from("12"));
    let mut w2_year1 = use_signal(|| String::new());
    let mut w2_year1_months = use_signal(|| String::from("12"));
    let mut w2_year2 = use_signal(|| String::new());
    let mut w2_year2_months = use_signal(|| String::from("12"));
    let mut selected_base = use_signal(|| String::from("none"));
    
    // Calculate income based on pay frequency
    let frequency_multiplier = use_memo(move || {
        match pay_frequency().as_str() {
            "monthly" => 1.0,
            "bi-weekly" => 26.0 / 12.0,
            "semi-monthly" => 24.0 / 12.0,
            "weekly" => 52.0 / 12.0,
            _ => 1.0,
        }
    });
    
    let calculated_income = use_memo(move || {
        if let Ok(amount) = salary_amount().parse::<f64>() {
            amount * frequency_multiplier()
        } else {
            0.0
        }
    });
    
    // Calculate YTD Monthly Average
    let ytd_monthly_avg = use_memo(move || {
        if let Ok(ytd) = ytd_salary().parse::<f64>() {
            if let Ok(months) = ytd_months().parse::<f64>() {
                if months > 0.0 {
                    ytd / months
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
    
    // Calculate W2 Year 1 Income
    let w2_year1_income = use_memo(move || {
        if let Ok(w2) = w2_year1().parse::<f64>() {
            if let Ok(months) = w2_year1_months().parse::<f64>() {
                if months > 0.0 {
                    w2 / months
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
    
    // Calculate W2 Year 2 Income
    let w2_year2_income = use_memo(move || {
        if let Ok(w2) = w2_year2().parse::<f64>() {
            if let Ok(months) = w2_year2_months().parse::<f64>() {
                if months > 0.0 {
                    w2 / months
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
    
    // Determine base used to qualify
    let base_used_to_qualify = use_memo(move || {
        match selected_base().as_str() {
            "current" => calculated_income(),
            "ytd" => ytd_monthly_avg(),
            "w2_year1" => w2_year1_income(),
            "w2_year2" => w2_year2_income(),
            _ => 0.0,
        }
    });
    
    // Helper to format money
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    
    rsx! {
        div { class: "space-y-8",
            // Current Salary Section
            div { class: "bg-gradient-to-br from-purple-50 to-pink-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "💰" }
                    "Current Salary"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Salary Amount",
                                r#type: "number",
                                name: "salary_amount",
                                value: "{salary_amount}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| salary_amount.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Pay Frequency"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{pay_frequency}",
                                onchange: move |evt: Event<FormData>| pay_frequency.set(evt.value()),
                                option { value: "monthly", "Monthly (×1)" }
                                option { value: "bi-weekly", "Bi-Weekly (×26/12)" }
                                option { value: "semi-monthly", "Semi-Monthly (×24/12)" }
                                option { value: "weekly", "Weekly (×52/12)" }
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-purple-200 border-2 border-purple-400 rounded-lg text-gray-900 font-bold text-right text-lg",
                                "{format_money(calculated_income())}"
                            }
                        }
                    }
                }
            }
            // Historical Income Section
            div { class: "bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "📊" }
                    "Historical Income"
                }
                div { class: "space-y-4",
                    // YTD Salary
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "YTD Salary (Paystub)",
                                r#type: "number",
                                name: "ytd_salary",
                                value: "{ytd_salary}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| ytd_salary.set(evt.value()),
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
                                "Monthly Average"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(ytd_monthly_avg())}"
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
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
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
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Monthly Income"
                            }
                            div { class: "px-4 py-3 bg-white border-2 border-gray-300 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(w2_year2_income())}"
                            }
                        }
                    }
                }
            }
            // Base Used to Qualify Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "✓" }
                    "Base Used to Qualify"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Select Salary to Use"
                            }
                            select {
                                class: "bg-white border-2 border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 font-semibold",
                                value: "{selected_base}",
                                onchange: move |evt: Event<FormData>| selected_base.set(evt.value()),
                                option { value: "none", "-- Select Salary --" }
                                option { value: "current",
                                    "Current Salary ({format_money(calculated_income())})"
                                }
                                option { value: "ytd", "YTD Average ({format_money(ytd_monthly_avg())})" }
                                option { value: "w2_year1",
                                    "W2 Year 1 ({format_money(w2_year1_income())})"
                                }
                                option { value: "w2_year2",
                                    "W2 Year 2 ({format_money(w2_year2_income())})"
                                }
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Income"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right text-xl",
                                "{format_money(base_used_to_qualify())}"
                            }
                        }
                    }
                    // Note about requirements
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-yellow-700", "⚠️ Note: " }
                            "If YTD or past year is lower, confirm why. Otherwise, lower of YTD and W2 required."
                        }
                    }
                }
            }
        }
    }
}	