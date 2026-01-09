use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn Commission() -> Element {
    // State management for inputs
    let mut ytd_commission = use_signal(|| String::new());
    let mut ytd_expenses = use_signal(|| String::new());
    let mut past_year_commission = use_signal(|| String::new());
    let mut past_year_expenses = use_signal(|| String::new());
    let mut additional_year_commission = use_signal(|| String::new());
    let mut additional_year_expenses = use_signal(|| String::new());
    let mut ytd_months = use_signal(|| String::from("12"));
    let mut past_year_months = use_signal(|| String::from("12"));
    let mut additional_year_months = use_signal(|| String::from("12"));
    let mut selected_income = use_signal(|| String::from("ytd"));

    // Calculations
    let ytd_net_income = use_memo(move || {
        let comm = ytd_commission().parse::<f64>().unwrap_or(0.0);
        let exp = ytd_expenses().parse::<f64>().unwrap_or(0.0);
        comm - exp
    });

    let ytd_monthly_income = use_memo(move || {
        let months = ytd_months().parse::<f64>().unwrap_or(12.0);
        if months > 0.0 {
            ytd_net_income() / months
        } else {
            0.0
        }
    });

    let past_year_net_income = use_memo(move || {
        let comm = past_year_commission().parse::<f64>().unwrap_or(0.0);
        let exp = past_year_expenses().parse::<f64>().unwrap_or(0.0);
        comm - exp
    });

    let past_year_monthly_income = use_memo(move || {
        let months = past_year_months().parse::<f64>().unwrap_or(12.0);
        if months > 0.0 {
            past_year_net_income() / months
        } else {
            0.0
        }
    });

    let additional_year_net_income = use_memo(move || {
        let comm = additional_year_commission().parse::<f64>().unwrap_or(0.0);
        let exp = additional_year_expenses().parse::<f64>().unwrap_or(0.0);
        comm - exp
    });

    let additional_year_monthly_income = use_memo(move || {
        let months = additional_year_months().parse::<f64>().unwrap_or(12.0);
        if months > 0.0 {
            additional_year_net_income() / months
        } else {
            0.0
        }
    });

    // Averages and comparisons (placeholders based on comments)
    let ytd_avg_net = use_memo(move || ytd_monthly_income());
    let ytd_plus_1_year = use_memo(move || (ytd_monthly_income() + past_year_monthly_income()) / 2.0);
    let ytd_plus_2_year = use_memo(move || (ytd_monthly_income() + past_year_monthly_income() + additional_year_monthly_income()) / 3.0);

    // Selected income
    let selected_monthly_income = use_memo(move || {
        match selected_income().as_str() {
            "ytd" => ytd_monthly_income(),
            "past_year" => past_year_monthly_income(),
            "additional_year" => additional_year_monthly_income(),
            "ytd_avg" => ytd_avg_net(),
            "ytd_plus_1" => ytd_plus_1_year(),
            "ytd_plus_2" => ytd_plus_2_year(),
            _ => 0.0,
        }
    });

    // Helper to format money
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }

    rsx! {
        div { class: "space-y-8",
            // Commission Income Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 p-6 rounded-xl shadow-md border-2 border-green-200 dark:border-green-700",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "💼" }
                    "Commission Income"
                }
                div { class: "space-y-6",
                    // YTD Commission
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-3",
                            Input {
                                label: "YTD Commission",
                                r#type: "number",
                                name: "ytd_commission",
                                value: "{ytd_commission}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| ytd_commission.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "minus"
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "Expenses",
                                r#type: "number",
                                name: "ytd_expenses",
                                value: "{ytd_expenses}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| ytd_expenses.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "="
                        }
                        div { class: "col-span-2",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(ytd_net_income())}"
                            }
                        }
                        div { class: "col-span-1",
                            Input {
                                label: "# months",
                                r#type: "number",
                                name: "ytd_months",
                                value: "{ytd_months}",
                                placeholder: "12",
                                oninput: move |evt: Event<FormData>| ytd_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(ytd_monthly_income())}"
                            }
                        }
                    }
                    // Past Year Commission
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-3",
                            Input {
                                label: "Past Year Commission",
                                r#type: "number",
                                name: "past_year_commission",
                                value: "{past_year_commission}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| past_year_commission.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "minus"
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "Expenses",
                                r#type: "number",
                                name: "past_year_expenses",
                                value: "{past_year_expenses}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| past_year_expenses.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "="
                        }
                        div { class: "col-span-2",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(past_year_net_income())}"
                            }
                        }
                        div { class: "col-span-1",
                            Input {
                                label: "# months",
                                r#type: "number",
                                name: "past_year_months",
                                value: "{past_year_months}",
                                placeholder: "12",
                                oninput: move |evt: Event<FormData>| past_year_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(past_year_monthly_income())}"
                            }
                        }
                    }
                    // Additional Year Commission
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-3",
                            Input {
                                label: "Additional Year Commission",
                                r#type: "number",
                                name: "additional_year_commission",
                                value: "{additional_year_commission}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| additional_year_commission.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "minus"
                        }
                        div { class: "col-span-3",
                            Input {
                                label: "Expenses",
                                r#type: "number",
                                name: "additional_year_expenses",
                                value: "{additional_year_expenses}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| additional_year_expenses.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1 text-center text-gray-500 dark:text-gray-400 font-semibold",
                            "="
                        }
                        div { class: "col-span-2",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(additional_year_net_income())}"
                            }
                        }
                        div { class: "col-span-1",
                            Input {
                                label: "# months",
                                r#type: "number",
                                name: "additional_year_months",
                                value: "{additional_year_months}",
                                placeholder: "12",
                                oninput: move |evt: Event<FormData>| additional_year_months.set(evt.value()),
                            }
                        }
                        div { class: "col-span-1",
                            div { class: "px-4 py-3 bg-green-200 dark:bg-green-800 border-2 border-green-400 dark:border-green-600 rounded-lg text-gray-900 dark:text-white font-bold text-right",
                                "{format_money(additional_year_monthly_income())}"
                            }
                        }
                    }
                }
                p { class: "text-sm text-gray-600 dark:text-gray-300 mt-4",
                    "*Follow investor guidelines for unreimbursed expense policy."
                }
            }

            // Averages and Comparisons Section
            div { class: "bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 p-6 rounded-xl shadow-md border-2 border-blue-200 dark:border-blue-700",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "📊" }
                    "Averages and Comparisons"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "flex items-center justify-between p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            span { class: "font-semibold", "YTD Avg using net income" }
                            span { class: "font-bold text-blue-800 dark:text-blue-300",
                                "{format_money(ytd_avg_net())}"
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            span { class: "font-semibold", "2106 YTD Expense Estimate" }
                            span { class: "font-bold text-blue-800 dark:text-blue-300",
                                "FALSE"
                            } // Placeholder
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "flex items-center justify-between p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            span { class: "font-semibold", "YTD + 1 year using Net Income" }
                            span { class: "font-bold text-blue-800 dark:text-blue-300",
                                "{format_money(ytd_plus_1_year())}"
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            span { class: "font-semibold", "2 year Commission Vs 2106 Expenses" }
                            span { class: "font-bold text-blue-800 dark:text-blue-300",
                                "FALSE"
                            } // Placeholder
                        }
                    }
                    div { class: "grid grid-cols-3 gap-4",
                        div { class: "flex items-center justify-between p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            span { class: "font-semibold", "YTD + 2 year using net income" }
                            span { class: "font-bold text-blue-800 dark:text-blue-300",
                                "{format_money(ytd_plus_2_year())}"
                            }
                        }
                        div {
                            class: "p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            div { class: "font-semibold mb-2", "Commission" }
                            div { class: "font-bold text-blue-800 dark:text-blue-300",
                                "$ -"
                            } // Placeholder
                        }
                        div {
                            class: "p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                            div { class: "font-semibold mb-2", "Expenses" }
                            div { class: "font-bold text-blue-800 dark:text-blue-300",
                                "$ -"
                            } // Placeholder
                        }
                    }
                    div { class: "p-4 bg-blue-100 dark:bg-blue-900/50 rounded-lg",
                        div { class: "font-semibold mb-2", "Expense factor" }
                        div { class: "font-bold text-blue-800 dark:text-blue-300", "$ -" } // Placeholder
                    }
                }
            }

            // Income Selection Section
            div { class: "bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-900/20 dark:to-pink-900/20 p-6 rounded-xl shadow-md border-2 border-purple-200 dark:border-purple-700",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "✅" }
                    "Use Lower of Calculations"
                }
                div { class: "space-y-4",
                    div { class: "flex flex-col",
                        label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100",
                            "Select Income to Use"
                        }
                        select {
                            class: "bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 text-sm rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 block w-full px-4 py-3 shadow-sm transition-all duration-200 hover:border-gray-400 dark:hover:border-gray-500 font-semibold",
                            value: "{selected_income}",
                            onchange: move |evt: Event<FormData>| selected_income.set(evt.value()),
                            option { value: "ytd", "YTD Net Income" }
                            option { value: "past_year", "Past Year Net Income" }
                            option { value: "additional_year", "Additional Year Net Income" }
                            option { value: "ytd_avg", "YTD Avg using net income" }
                            option { value: "ytd_plus_1", "YTD + 1 year using Net Income" }
                            option { value: "ytd_plus_2", "YTD + 2 year using net income" }
                        }
                    }
                    div { class: "flex items-center justify-between p-4 bg-purple-100 dark:bg-purple-900/50 rounded-lg",
                        span { class: "font-semibold", "Selected Monthly Income" }
                        span { class: "font-bold text-purple-800 dark:text-purple-300 text-xl",
                            "{format_money(selected_monthly_income())}"
                        }
                    }
                }
            }
        }
    }
}
										