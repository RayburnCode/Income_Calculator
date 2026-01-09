use dioxus::prelude::*;
use crate::components::Input;

#[component]
pub fn OtherIncome() -> Element {
    // State management for various income types
    let mut alimony_monthly = use_signal(|| String::new());
    let mut child_support_monthly = use_signal(|| String::new());
    let mut rental_gross_monthly = use_signal(|| String::new());
    let mut rental_expenses_monthly = use_signal(|| String::new());
    let mut interest_dividends_monthly = use_signal(|| String::new());
    let mut disability_monthly = use_signal(|| String::new());
    let mut workers_comp_monthly = use_signal(|| String::new());
    let mut other_income_monthly = use_signal(|| String::new());
    let mut other_income_description = use_signal(|| String::new());

    // Calculate qualifying alimony (75% of gross)
    let qualifying_alimony = use_memo(move || {
        alimony_monthly().parse::<f64>().unwrap_or(0.0) * 0.75
    });

    // Child support is typically not countable as borrower income
    let qualifying_child_support = use_memo(move || {
        0.0 // Child support is generally not considered qualifying income
    });

    // Calculate qualifying rental income (75% of net rental income)
    let qualifying_rental = use_memo(move || {
        let gross = rental_gross_monthly().parse::<f64>().unwrap_or(0.0);
        let expenses = rental_expenses_monthly().parse::<f64>().unwrap_or(0.0);
        let net = gross - expenses;
        if net > 0.0 {
            net * 0.75
        } else {
            0.0
        }
    });

    // Interest and dividends (100% of taxable portion)
    let qualifying_interest_dividends = use_memo(move || {
        interest_dividends_monthly().parse::<f64>().unwrap_or(0.0)
    });

    // Disability income (may be considered if expected to continue)
    let qualifying_disability = use_memo(move || {
        disability_monthly().parse::<f64>().unwrap_or(0.0)
    });

    // Workers compensation (may be considered if expected to continue)
    let qualifying_workers_comp = use_memo(move || {
        workers_comp_monthly().parse::<f64>().unwrap_or(0.0)
    });

    // Other income (case-by-case basis)
    let qualifying_other = use_memo(move || {
        other_income_monthly().parse::<f64>().unwrap_or(0.0)
    });

    // Calculate total qualifying other income
    let total_qualifying_other_income = use_memo(move || {
        qualifying_alimony() + qualifying_child_support() + qualifying_rental() +
        qualifying_interest_dividends() + qualifying_disability() + qualifying_workers_comp() +
        qualifying_other()
    });

    fn format_money(amount: f64) -> String {
        format!("${:.2}", amount)
    }

    rsx! {
        div { class: "space-y-8",
            // Alimony Section
            div { class: "bg-gradient-to-br from-rose-50 to-pink-50 p-6 rounded-xl shadow-md border-2 border-rose-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-rose-600", "💔" }
                    "Alimony Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Alimony Received",
                                r#type: "number",
                                name: "alimony_monthly",
                                value: "{alimony_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| alimony_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount (75%)"
                            }
                            div { class: "px-4 py-3 bg-rose-200 border-2 border-rose-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_alimony())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Fannie Mae: " }
                            "75% of gross alimony received is considered qualifying income. Must be documented with court order and payment history."
                        }
                    }
                }
            }

            // Child Support Section
            div { class: "bg-gradient-to-br from-purple-50 to-violet-50 p-6 rounded-xl shadow-md border-2 border-purple-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-purple-600", "👨‍👩‍👧‍👦" }
                    "Child Support Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Child Support Received",
                                r#type: "number",
                                name: "child_support_monthly",
                                value: "{child_support_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| child_support_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-purple-200 border-2 border-purple-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_child_support())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-red-50 border-2 border-red-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-red-700", "⚠️ Note: " }
                            "Child support is generally NOT considered qualifying income for mortgage purposes as it is intended for child support, not borrower income."
                        }
                    }
                }
            }

            // Rental Income Section
            div { class: "bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-xl shadow-md border-2 border-blue-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-blue-600", "🏠" }
                    "Rental Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Monthly Gross Rental Income",
                                r#type: "number",
                                name: "rental_gross_monthly",
                                value: "{rental_gross_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| rental_gross_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4",
                            Input {
                                label: "Monthly Rental Expenses",
                                r#type: "number",
                                name: "rental_expenses_monthly",
                                value: "{rental_expenses_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| rental_expenses_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount (75% of Net)"
                            }
                            div { class: "px-4 py-3 bg-blue-200 border-2 border-blue-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_rental())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Fannie Mae: " }
                            "75% of net rental income (gross minus expenses) is considered qualifying. Expenses must be documented and reasonable."
                        }
                    }
                }
            }

            // Investment Income Section
            div { class: "bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-green-600", "📈" }
                    "Investment Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Interest & Dividends (Taxable)",
                                r#type: "number",
                                name: "interest_dividends_monthly",
                                value: "{interest_dividends_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| interest_dividends_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount (100%)"
                            }
                            div { class: "px-4 py-3 bg-green-200 border-2 border-green-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_interest_dividends())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Fannie Mae: " }
                            "100% of taxable interest and dividend income is considered qualifying. Must be documented with tax returns."
                        }
                    }
                }
            }

            // Disability & Workers Comp Section
            div { class: "bg-gradient-to-br from-orange-50 to-amber-50 p-6 rounded-xl shadow-md border-2 border-orange-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-orange-600", "🏥" }
                    "Disability & Workers Compensation"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Disability Income",
                                r#type: "number",
                                name: "disability_monthly",
                                value: "{disability_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| disability_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-orange-200 border-2 border-orange-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_disability())}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-12 gap-4 items-end mt-4",
                        div { class: "col-span-6",
                            Input {
                                label: "Monthly Workers Compensation",
                                r#type: "number",
                                name: "workers_comp_monthly",
                                value: "{workers_comp_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| workers_comp_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-orange-200 border-2 border-orange-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_workers_comp())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Fannie Mae: " }
                            "Disability and workers compensation may be considered if expected to continue. Requires documentation of expected duration."
                        }
                    }
                }
            }

            // Other Income Section
            div { class: "bg-gradient-to-br from-indigo-50 to-purple-50 p-6 rounded-xl shadow-md border-2 border-indigo-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-indigo-600", "💡" }
                    "Other Income Sources"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-end",
                        div { class: "col-span-4",
                            Input {
                                label: "Monthly Other Income",
                                r#type: "number",
                                name: "other_income_monthly",
                                value: "{other_income_monthly}",
                                placeholder: "0.00",
                                oninput: move |evt: Event<FormData>| other_income_monthly.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4",
                            Input {
                                label: "Description",
                                r#type: "text",
                                name: "other_income_description",
                                value: "{other_income_description}",
                                placeholder: "e.g., Royalties, Commissions",
                                oninput: move |evt: Event<FormData>| other_income_description.set(evt.value()),
                            }
                        }
                        div { class: "col-span-4 flex flex-col",
                            label { class: "block mb-2.5 text-sm font-semibold text-gray-900 dark:text-gray-100 dark:text-gray-100",
                                "Qualifying Amount"
                            }
                            div { class: "px-4 py-3 bg-indigo-200 border-2 border-indigo-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right",
                                "{format_money(qualifying_other())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-yellow-50 border-2 border-yellow-300 rounded-lg",
                        p { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 font-medium",
                            span { class: "font-bold text-yellow-700", "📋 Note: " }
                            "Other income sources are evaluated on a case-by-case basis. Requires documentation of consistency and likelihood of continuation."
                        }
                    }
                }
            }

            // Total Other Income Summary
            div { class: "bg-gradient-to-br from-gray-50 to-slate-50 p-6 rounded-xl shadow-md border-2 border-gray-200",
                h3 { class: "text-xl font-bold text-gray-900 dark:text-gray-100 dark:text-gray-100 mb-6 flex items-center gap-2",
                    span { class: "text-gray-600", "📊" }
                    "Total Qualifying Other Income"
                }
                div { class: "space-y-4",
                    div { class: "grid grid-cols-12 gap-4 items-center",
                        div { class: "col-span-6 flex items-center gap-3",
                            div { class: "px-4 py-2 bg-gray-200 border-2 border-gray-400 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-semibold text-center min-w-[200px]",
                                "Monthly Qualifying Other Income"
                            }
                        }
                        div { class: "col-span-6 flex flex-col",
                            div { class: "px-4 py-3 bg-gray-300 border-2 border-gray-500 rounded-lg text-gray-900 dark:text-gray-100 dark:text-gray-100 font-bold text-right text-xl",
                                "{format_money(total_qualifying_other_income())}"
                            }
                        }
                    }
                    div { class: "mt-4 p-4 bg-blue-50 border-2 border-blue-300 rounded-lg",
                        h4 { class: "font-bold text-blue-800 mb-2",
                            "General Documentation Requirements:"
                        }
                        ul { class: "text-sm text-gray-900 dark:text-gray-100 dark:text-gray-100 space-y-1",
                            li { "• Tax returns showing income reporting (minimum 2 years)" }
                            li { "• Award letters, court orders, or contracts" }
                            li { "• Payment history (minimum 12-24 months)" }
                            li { "• Documentation of expected continuation" }
                            li { "• Expense documentation for rental income" }
                        }
                    }
                }
            }
        }
    }
} 