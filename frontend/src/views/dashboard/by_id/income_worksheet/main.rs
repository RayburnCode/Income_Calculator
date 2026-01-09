use dioxus::prelude::*;
use crate::components::{IncomeAccordion, IncomeAccordionItem};
use crate::views::dashboard::by_id::income_worksheet::{Hourly, Salary, OTBonus, Commission, OtherW2, SocialSecurity, Pension, IRA, OtherIncome};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Worksheet(id: i32) -> Element {
    // Track which sections are displayed and included
    let mut displayed_sections = use_signal(|| Vec::<String>::new());

    let mut handle_display_change = move |(id, displayed): (String, bool)| {
        let mut sections = displayed_sections();
        if displayed {
            if !sections.contains(&id) {
                sections.push(id);
            }
        } else {
            sections.retain(|s| s != &id);
        }
        displayed_sections.set(sections);
    };

    // Build pay type items
    let mut pay_type_items = vec![];
    if displayed_sections.read().contains(&"hourly".to_string()) {
        pay_type_items.push(IncomeAccordionItem {
            id: "hourly".to_string(),
            title: "Hourly Pay".to_string(),
            content: rsx! {
                Hourly {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"salary".to_string()) {
        pay_type_items.push(IncomeAccordionItem {
            id: "salary".to_string(),
            title: "Salary".to_string(),
            content: rsx! {
                Salary {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"ot_bonus".to_string()) {
        pay_type_items.push(IncomeAccordionItem {
            id: "ot_bonus".to_string(),
            title: "Overtime / Bonus".to_string(),
            content: rsx! {
                OTBonus {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"commission".to_string()) {
        pay_type_items.push(IncomeAccordionItem {
            id: "commission".to_string(),
            title: "Commission".to_string(),
            content: rsx! {
                Commission {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"other_w2".to_string()) {
        pay_type_items.push(IncomeAccordionItem {
            id: "other_w2".to_string(),
            title: "Other W2 Income".to_string(),
            content: rsx! {
                OtherW2 {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }

    // Build other income items
    let mut other_income_items = vec![];
    if displayed_sections.read().contains(&"social_security".to_string()) {
        other_income_items.push(IncomeAccordionItem {
            id: "social_security".to_string(),
            title: "Social Security".to_string(),
            content: rsx! {
                SocialSecurity {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"pension".to_string()) {
        other_income_items.push(IncomeAccordionItem {
            id: "pension".to_string(),
            title: "Pension".to_string(),
            content: rsx! {
                Pension {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"ira".to_string()) {
        other_income_items.push(IncomeAccordionItem {
            id: "ira".to_string(),
            title: "IRA / 401(k)".to_string(),
            content: rsx! {
                IRA {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }
    if displayed_sections.read().contains(&"other_income".to_string()) {
        other_income_items.push(IncomeAccordionItem {
            id: "other_income".to_string(),
            title: "Other Income".to_string(),
            content: rsx! {
                OtherIncome {}
            },
            initially_open: false,
            include_in_calc: true,
        });
    }

    rsx! {
        div { class: "space-y-8 p-8 bg-gradient-to-br from-gray-50 to-gray-100 min-h-screen",

            // Income Type Selection
            div { class: "bg-white p-8 rounded-xl shadow-lg border border-gray-200",
                h2 { class: "text-3xl font-bold text-gray-900 mb-6 border-b-2 border-green-500 pb-3",
                    "Select Income Types to Include"
                }
                p { class: "text-gray-600 mb-4",
                    "Choose which income types apply to this borrower. Only selected types will be displayed and included in calculations."
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "hourly",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"hourly".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"hourly".to_string());
                                handle_display_change(("hourly".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "hourly",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Hourly Pay"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "salary",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"salary".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"salary".to_string());
                                handle_display_change(("salary".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "salary",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Salary"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "ot_bonus",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"ot_bonus".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"ot_bonus".to_string());
                                handle_display_change(("ot_bonus".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "ot_bonus",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Overtime / Bonus"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "commission",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"commission".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"commission".to_string());
                                handle_display_change(("commission".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "commission",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Commission"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "other_w2",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"other_w2".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"other_w2".to_string());
                                handle_display_change(("other_w2".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "other_w2",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Other W2 Income"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "social_security",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"social_security".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections
                                    .read()
                                    .contains(&"social_security".to_string());
                                handle_display_change(("social_security".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "social_security",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Social Security"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "pension",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"pension".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"pension".to_string());
                                handle_display_change(("pension".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "pension",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Pension"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "ira",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"ira".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"ira".to_string());
                                handle_display_change(("ira".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "ira",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "IRA / 401(k)"
                        }
                    }
                    div { class: "flex items-center space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors",
                        input {
                            r#type: "checkbox",
                            id: "other_income",
                            class: "w-5 h-5 text-green-600 border-gray-400 rounded focus:ring-2 focus:ring-green-500 cursor-pointer",
                            checked: displayed_sections.read().contains(&"other_income".to_string()),
                            onchange: move |_| {
                                let is_checked = displayed_sections.read().contains(&"other_income".to_string());
                                handle_display_change(("other_income".to_string(), !is_checked));
                            },
                        }
                        label {
                            r#for: "other_income",
                            class: "text-gray-900 font-medium cursor-pointer",
                            "Other Income"
                        }
                    }
                }
            }

            // Display selected income sections
            // Pay Type Section
            if !pay_type_items.is_empty() {
                h2 { class: "text-3xl font-bold text-gray-900 mt-10 mb-6 flex items-center gap-3",
                    span { class: "text-blue-600", "💼" }
                    "Pay Type"
                }
                IncomeAccordion { items: pay_type_items, on_include_change: None }
            }

            // Other Income Section
            if !other_income_items.is_empty() {
                h2 { class: "text-3xl font-bold text-gray-900 mt-10 mb-6 flex items-center gap-3",
                    span { class: "text-green-600", "📊" }
                    "Other Taxable and Nontaxable Income"
                }
                IncomeAccordion { items: other_income_items, on_include_change: None }
            }

            // Summary section - show if any items are selected
            if !displayed_sections.read().is_empty() {
                div { class: "bg-gradient-to-r from-blue-50 to-indigo-50 p-8 rounded-xl shadow-lg border-2 border-blue-200 mt-10",
                    h3 { class: "text-2xl font-bold text-gray-900 mb-6 flex items-center gap-2",
                        span { class: "text-blue-600", "✓" }
                        "Included in Calculations"
                    }
                    div { class: "flex flex-wrap gap-2",
                        for section_id in displayed_sections.read().iter() {
                            span {
                                key: "{section_id}",
                                class: if section_id == "hourly" || section_id == "salary" || section_id == "ot_bonus"
    || section_id == "commission" || section_id == "other_w2" { "px-4 py-2 bg-blue-600 text-white rounded-full text-sm font-semibold shadow-md hover:bg-blue-700 transition-colors" } else { "px-4 py-2 bg-green-600 text-white rounded-full text-sm font-semibold shadow-md hover:bg-green-700 transition-colors" },
                                {
                                    if section_id == "hourly" {
                                        "Hourly Pay"
                                    } else if section_id == "salary" {
                                        "Salary"
                                    } else if section_id == "ot_bonus" {
                                        "OT/Bonus"
                                    } else if section_id == "commission" {
                                        "Commission"
                                    } else if section_id == "other_w2" {
                                        "Other W2"
                                    } else if section_id == "social_security" {
                                        "Social Security"
                                    } else if section_id == "pension" {
                                        "Pension"
                                    } else if section_id == "ira" {
                                        "IRA/401(k)"
                                    } else if section_id == "other_income" {
                                        "Other Income"
                                    } else {
                                        section_id.as_str()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}