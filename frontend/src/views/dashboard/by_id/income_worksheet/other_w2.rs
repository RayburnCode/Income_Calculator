use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn OtherW2() -> Element {
    // State management for various W2 income types
    let mut tips_monthly = use_signal(|| String::new());
    let mut shift_differential_monthly = use_signal(|| String::new());
    let mut on_call_pay_monthly = use_signal(|| String::new());
    let mut call_back_pay_monthly = use_signal(|| String::new());
    let mut hazard_pay_monthly = use_signal(|| String::new());
    let mut profit_sharing_monthly = use_signal(|| String::new());
    let mut sick_pay_monthly = use_signal(|| String::new());
    let mut holiday_vacation_pay_monthly = use_signal(|| String::new());

    // Calculate qualifying amounts based on Fannie Mae/Freddie Mac guidelines
    let qualifying_tips = use_memo(move || {
        // Tips are 100% qualifying if documented and consistent
        tips_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_shift_differential = use_memo(move || {
        // Shift differentials are 100% qualifying if regular
        shift_differential_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_on_call_pay = use_memo(move || {
        // On-call pay may be considered if regular and documented
        on_call_pay_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_call_back_pay = use_memo(move || {
        // Call-back pay may be considered if regular
        call_back_pay_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_hazard_pay = use_memo(move || {
        // Hazard pay is 100% qualifying if regular
        hazard_pay_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_profit_sharing = use_memo(move || {
        // Profit sharing may be considered if consistent (usually requires 2+ year history)
        profit_sharing_monthly().parse::<f64>().unwrap_or(0.0)
    });

    let qualifying_sick_pay = use_memo(move || {
        // Sick pay is generally not considered qualifying income
        0.0
    });

    let qualifying_holiday_vacation_pay = use_memo(move || {
        // Holiday/vacation pay may be considered if paid out separately and consistent
        holiday_vacation_pay_monthly().parse::<f64>().unwrap_or(0.0)
    });

    // Calculate total qualifying other W2 income
    let total_qualifying_other_w2 = use_memo(move || {
        qualifying_tips() + qualifying_shift_differential() + qualifying_on_call_pay() +
        qualifying_call_back_pay() + qualifying_hazard_pay() + qualifying_profit_sharing() +
        qualifying_sick_pay() + qualifying_holiday_vacation_pay()
    });

    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }

    rsx! {
        div { class: "space-y-8",
            // Tips & Gratuities Section
            div { class: "bg-gradient-to-br from-yellow-50 to-amber-50 p-6 rounded-xl shadow-md border-2 border-yellow-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-yellow-600", "💰" }
                    "Tips & Gratuities"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Tips & Gratuities",
                                r#type: "number",
                                name: "tips_monthly",
                                value: "{tips_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| tips_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount (100%)"
                            }
                            div { class: "px-4 py-3 bg-yellow-200 border-2 border-yellow-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_tips())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-blue-700", "📋 Fannie Mae/Freddie Mac: " }
                            "100% of tips and gratuities are qualifying if documented with W2 forms, tip reporting forms, or employer verification. Must show consistency for 12-24 months."
                        }
                    }
                }
            }

            // Shift Differentials & Premium Pay Section
            div { class: "bg-gradient-to-br from-purple-50 to-violet-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "🕐" }
                    "Shift Differentials & Premium Pay"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Shift Differential",
                                r#type: "number",
                                name: "shift_differential_monthly",
                                value: "{shift_differential_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| shift_differential_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount (100%)"
                            }
                            div { class: "px-4 py-3 bg-purple-200 border-2 border-purple-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_shift_differential())}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-12 gap-4 items-end mt-4",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Hazard Pay",
                                r#type: "number",
                                name: "hazard_pay_monthly",
                                value: "{hazard_pay_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| hazard_pay_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount (100%)"
                            }
                            div { class: "px-4 py-3 bg-purple-200 border-2 border-purple-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_hazard_pay())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-blue-700", "📋 Fannie Mae/Freddie Mac: " }
                            "100% of shift differentials and hazard pay are qualifying if regular and documented on pay stubs or W2 forms."
                        }
                    }
                }
            }

            // On-Call & Call-Back Pay Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "📞" }
                    "On-Call & Call-Back Pay"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly On-Call/Standby Pay",
                                r#type: "number",
                                name: "on_call_pay_monthly",
                                value: "{on_call_pay_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| on_call_pay_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_on_call_pay())}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-12 gap-4 items-end mt-4",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Call-Back Pay",
                                r#type: "number",
                                name: "call_back_pay_monthly",
                                value: "{call_back_pay_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| call_back_pay_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_call_back_pay())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-yellow-700", "⚠️ Note: " }
                            "On-call and call-back pay may be considered if regular and documented. Requires employer verification of frequency and likelihood of continuation."
                        }
                    }
                }
            }

            // Profit Sharing & Other Compensation Section
            div { class: "bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "📊" }
                    "Profit Sharing & Other Compensation"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Profit Sharing",
                                r#type: "number",
                                name: "profit_sharing_monthly",
                                value: "{profit_sharing_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| profit_sharing_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_profit_sharing())}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-12 gap-4 items-end mt-4",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Holiday/Vacation Pay",
                                r#type: "number",
                                name: "holiday_vacation_pay_monthly",
                                value: "{holiday_vacation_pay_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| holiday_vacation_pay_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_holiday_vacation_pay())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Fannie Mae/Freddie Mac: " }
                            "Profit sharing requires 2+ years of consistent payments. Holiday/vacation pay may be considered if paid out separately and documented."
                        }
                    }
                }
            }

            // Non-Qualifying Income Section
            div { class: "bg-gradient-to-br from-red-50 to-rose-50 p-6 rounded-xl shadow-md border-2 border-red-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-red-600", "❌" }
                    "Generally Non-Qualifying Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Sick Pay",
                                r#type: "number",
                                name: "sick_pay_monthly",
                                value: "{sick_pay_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| sick_pay_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-red-200 border-2 border-red-400 rounded-lg text-gray-900 font-bold text-right",
                                "{format_money(qualifying_sick_pay())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-red-50 border-2 border-red-300 rounded-lg",
                        h4 { class: "font-bold text-red-800 mb-2",
                            "Generally NOT Considered Qualifying:"
                        }
                        ul { class: "text-sm text-gray-900 space-y-1",
                            li { "• Sick pay (unless expected to continue indefinitely)" }
                            li { "• Severance pay (one-time payment)" }
                            li { "• Sign-on or relocation bonuses (one-time)" }
                            li { "• Most retention bonuses (unless recurring)" }
                            li { "• Unemployment benefits" }
                            li { "• Workers compensation (unless long-term)" }
                        }
                    }
                }
            }

            // Total Other W2 Income Summary
            div { class: "bg-gradient-to-br from-gray-50 to-slate-50 p-6 rounded-xl shadow-md border-2 border-gray-200",
                h3 { class: "text-xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                    span { class: "text-gray-600", "📈" }
                    "Total Qualifying Other W2 Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-gray-200 border-2 border-gray-400 rounded-lg text-gray-900 font-semibold text-center min-w-[200px]",
                                "Monthly Qualifying Other W2 Income"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-gray-300 border-2 border-gray-500 rounded-lg text-gray-900 font-bold text-right text-xl",
                                "{format_money(total_qualifying_other_w2())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        h4 { class: "font-bold text-blue-800 mb-2", "Documentation Requirements:" }
                        ul { class: "text-sm text-gray-900 space-y-1",
                            li { "• Recent pay stubs showing all income components" }
                            li { "• W2 forms for the past 2 years" }
                            li { "• Employer verification of additional pay components" }
                            li { "• Profit sharing plan documents (if applicable)" }
                            li { "• Tax returns showing consistent reporting" }
                        }
                    }
                }
            }
        }
    }
}
 