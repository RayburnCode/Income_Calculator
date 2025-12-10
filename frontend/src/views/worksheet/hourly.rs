use dioxus::prelude::*;
use crate::components::{Input, Checkbox};
use chrono::DateTime;

#[component]
pub fn Hourly() -> Element {
    // State management
    let mut per_hour = use_signal(|| String::new());
    let mut hours_worked = use_signal(|| String::from("0"));
    let mut ytd_earnings = use_signal(|| String::new());
    let mut ytd_months = use_signal(|| String::from("0"));
    let mut w2_year1 = use_signal(|| String::new());
    let mut w2_year1_months = use_signal(|| String::from("0"));
    let mut w2_year2 = use_signal(|| String::new());
    let mut w2_year2_months = use_signal(|| String::from("0"));
    let mut check_date = use_signal(||DateTime::default());
    let mut check_date_month = check_date.clone();
    let mut ytd_months_calc = use_signal(|| String::from("0"));
    let mut days_in_month = use_signal(|| String::from("31"));
    
    // Checkbox states for income selection
    let mut use_per_hour = use_signal(|| false);
    let mut use_ytd = use_signal(|| false);
    let mut use_w2_year1 = use_signal(|| false);
    let mut use_w2_year2 = use_signal(|| false);
    let mut use_ytd_avg = use_signal(|| false);
    let mut use_ytd_plus_1w2 = use_signal(|| false);
    let mut use_ytd_plus_2w2 = use_signal(|| false);

    // Calculate per hour income
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

    // Calculate YTD income
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

    // Helper to format money
    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }

    rsx! {
        div { class: "space-y-6 p-6 bg-white rounded-lg shadow",
            h2 { class: "text-2xl font-bold text-gray-800 mb-4", "Hourly Income Calculator" }
            // Input Section

            div { class: "grid grid-cols-12 gap-4 items-center mb-4",
                div { class: "col-span-3",
                    Input {
                        label: "Check Date",
                        r#type: "date",
                        name: "check_date",
                        value: "{check_date}",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(dt) = evt.value().parse::<DateTime<chrono::Utc>>() {
                                check_date.set(dt);
                            }
                        },
                    }
                }
                div { class: "col-span-2",
                    Input {
                        label: "# YTD Months",
                        r#type: "number",
                        name: "ytd_months_calc",
                        value: "{ytd_months_calc}",
                        oninput: move |evt: Event<FormData>| ytd_months_calc.set(evt.value()),
                    }
                }
                div { class: "col-span-2",
                    Input {
                        label: "Days in Month",
                        r#type: "number",
                        name: "days_in_month",
                        value: "{days_in_month}",
                        oninput: move |evt: Event<FormData>| days_in_month.set(evt.value()),
                    }
                }
            }


            div { class: "space-y-4",
                // Per Hour Input
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        Input {
                            label: "Per Hour",
                            r#type: "number",
                            name: "per_hour",
                            value: "{per_hour}",
                            placeholder: "0.00",
                            oninput: move |evt: Event<FormData>| per_hour.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2",
                        Input {
                            label: "# of Hours",
                            r#type: "number",
                            name: "hours",
                            value: "{hours_worked}",
                            placeholder: "0",
                            oninput: move |evt: Event<FormData>| hours_worked.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2 flex items-end pb-2",
                        span { class: "text-sm text-gray-600", "X 52/12" }
                    }
                    div { class: "col-span-3 flex items-end pb-2",
                        span { class: "text-lg font-semibold text-green-600",
                            "{format_money(per_hour_income())}"
                        }
                    }
                    div { class: "col-span-2 flex items-end pb-2",
                        Checkbox {
                            label: "Use",
                            checked: use_per_hour(),
                            onchange: move |_| use_per_hour.set(!use_per_hour()),
                        }
                    }
                }

                // YTD Earnings Input
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        Input {
                            label: "YTD Earnings",
                            r#type: "number",
                            name: "ytd_earnings",
                            value: "{ytd_earnings}",
                            placeholder: "0.00",
                            oninput: move |evt: Event<FormData>| ytd_earnings.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2",
                        Input {
                            label: "# Months",
                            r#type: "number",
                            name: "ytd_months",
                            value: "{ytd_months}",
                            placeholder: "0",
                            oninput: move |evt: Event<FormData>| ytd_months.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2" }
                    div { class: "col-span-3 flex items-end pb-2",
                        span { class: "text-lg font-semibold text-green-600",
                            "{format_money(ytd_income())}"
                        }
                    }
                    div { class: "col-span-2 flex items-end pb-2",
                        Checkbox {
                            label: "Use",
                            checked: use_ytd(),
                            onchange: move |_| use_ytd.set(!use_ytd()),
                        }
                    }
                }

                // W2 Year 1 Input
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        Input {
                            label: "W2 Tax Year 1",
                            r#type: "number",
                            name: "w2_year1",
                            value: "{w2_year1}",
                            placeholder: "0.00",
                            oninput: move |evt: Event<FormData>| w2_year1.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2",
                        Input {
                            label: "# Months",
                            r#type: "number",
                            name: "w2_year1_months",
                            value: "{w2_year1_months}",
                            placeholder: "0",
                            oninput: move |evt: Event<FormData>| w2_year1_months.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2" }
                    div { class: "col-span-3 flex items-end pb-2",
                        span { class: "text-lg font-semibold text-green-600",
                            "{format_money(w2_year1_income())}"
                        }
                    }
                    div { class: "col-span-2 flex items-end pb-2",
                        Checkbox {
                            label: "Use",
                            checked: use_w2_year1(),
                            onchange: move |_| use_w2_year1.set(!use_w2_year1()),
                        }
                    }
                }

                // W2 Year 2 Input
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        Input {
                            label: "W2 Tax Year 2",
                            r#type: "number",
                            name: "w2_year2",
                            value: "{w2_year2}",
                            placeholder: "0.00",
                            oninput: move |evt: Event<FormData>| w2_year2.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2",
                        Input {
                            label: "# Months",
                            r#type: "number",
                            name: "w2_year2_months",
                            value: "{w2_year2_months}",
                            placeholder: "0",
                            oninput: move |evt: Event<FormData>| w2_year2_months.set(evt.value()),
                        }
                    }
                    div { class: "col-span-2" }
                    div { class: "col-span-3 flex items-end pb-2",
                        span { class: "text-lg font-semibold text-green-600",
                            "{format_money(w2_year2_income())}"
                        }
                    }
                    div { class: "col-span-2 flex items-end pb-2",
                        Checkbox {
                            label: "Use",
                            checked: use_w2_year2(),
                            onchange: move |_| use_w2_year2.set(!use_w2_year2()),
                        }
                    }
                }
            }

            // Separator
            div { class: "border-t border-gray-300 my-6" }

            // Average Calculations Section
            div { class: "space-y-4",
                h3 { class: "text-xl font-semibold text-gray-800 mb-3", "Average Calculations" }
                // Additional inputs

                // YTD Average
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        span { class: "text-sm font-medium text-gray-700", "YTD Avg" }
                    }
                    div { class: "col-span-6 flex items-center",
                        span { class: "text-lg font-semibold text-blue-600", "{format_money(ytd_avg())}" }
                    }
                    div { class: "col-span-3",
                        Checkbox {
                            label: "Use",
                            checked: use_ytd_avg(),
                            onchange: move |_| use_ytd_avg.set(!use_ytd_avg()),
                        }
                    }
                }

                // YTD + 1 W2 Average
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        span { class: "text-sm font-medium text-gray-700", "YTD + 1 W2 Avg" }
                    }
                    div { class: "col-span-6 flex items-center",
                        span { class: "text-lg font-semibold text-blue-600",
                            "{format_money(ytd_plus_1w2_avg())}"
                        }
                    }
                    div { class: "col-span-3",
                        Checkbox {
                            label: "Use",
                            checked: use_ytd_plus_1w2(),
                            onchange: move |_| use_ytd_plus_1w2.set(!use_ytd_plus_1w2()),
                        }
                    }
                }

                // YTD + 2 Yr W2 Average
                div { class: "grid grid-cols-12 gap-4 items-center",
                    div { class: "col-span-3",
                        span { class: "text-sm font-medium text-gray-700", "YTD + 2 Yr W2 Avg" }
                    }
                    div { class: "col-span-6 flex items-center",
                        span { class: "text-lg font-semibold text-blue-600",
                            "{format_money(ytd_plus_2w2_avg())}"
                        }
                    }
                    div { class: "col-span-3",
                        Checkbox {
                            label: "Use",
                            checked: use_ytd_plus_2w2(),
                            onchange: move |_| use_ytd_plus_2w2.set(!use_ytd_plus_2w2()),
                        }
                    }
                }
            }

            // Separator
            div { class: "border-t border-gray-300 my-6" }

            // Final Selection
            div { class: "bg-gray-50 p-4 rounded-lg",
                div { class: "flex justify-between items-center",
                    span { class: "text-lg font-semibold text-gray-700", "Use Lowest Income" }
                    span { class: "text-sm text-gray-600", "or check the income you wish to use above" }
                }
            }
        }
    }
}   	