use dioxus::prelude::*;
use crate::components::{IncomeAccordion, IncomeAccordionItem};
use crate::views::worksheet::{Information, Hourly, Salary, OTBonus, Commission, OtherW2, SocialSecurity, Pension, IRA, OtherIncome};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Worksheet() -> Element {
    // Track which sections are included
    let mut included_sections = use_signal(|| Vec::<String>::new());

    let handle_include_change = move |(id, included): (String, bool)| {
        let mut sections = included_sections();
        if included {
            if !sections.contains(&id) {
                sections.push(id);
            }
        } else {
            sections.retain(|s| s != &id);
        }
        included_sections.set(sections);
    };

    rsx! {
        div { class: "space-y-6 p-6 bg-gray-50 rounded-lg",
            // Borrower Information (always shown)
            div { class: "bg-white p-6 rounded-lg shadow",
                h2 { class: "text-2xl font-bold text-gray-800 mb-4", "Borrower Information" }
                Information {}
            }

            // Pay Type Section
            h2 { class: "text-2xl font-bold text-gray-800 mt-8 mb-4", "Pay Type" }
            IncomeAccordion {
                items: vec![
                    IncomeAccordionItem {
                        id: "hourly".to_string(),
                        title: "Hourly Pay".to_string(),
                        content: rsx! { Hourly {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "salary".to_string(),
                        title: "Salary".to_string(),
                        content: rsx! { Salary {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "ot_bonus".to_string(),
                        title: "Overtime / Bonus".to_string(),
                        content: rsx! { OTBonus {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "commission".to_string(),
                        title: "Commission".to_string(),
                        content: rsx! { Commission {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "other_w2".to_string(),
                        title: "Other W2 Income".to_string(),
                        content: rsx! { OtherW2 {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                ],
                on_include_change: handle_include_change,
            }

            // Other Income Section
            h2 { class: "text-2xl font-bold text-gray-800 mt-8 mb-4", "Other Taxable and Nontaxable Income" }
            IncomeAccordion {
                items: vec![
                    IncomeAccordionItem {
                        id: "social_security".to_string(),
                        title: "Social Security".to_string(),
                        content: rsx! { SocialSecurity {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "pension".to_string(),
                        title: "Pension".to_string(),
                        content: rsx! { Pension {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "ira".to_string(),
                        title: "IRA / 401(k)".to_string(),
                        content: rsx! { IRA {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                    IncomeAccordionItem {
                        id: "other_income".to_string(),
                        title: "Other Income".to_string(),
                        content: rsx! { OtherIncome {} },
                        initially_open: false,
                        include_in_calc: false,
                    },
                ],
                on_include_change: handle_include_change,
            }

            // Summary section
            div { class: "bg-blue-50 p-6 rounded-lg shadow mt-8",
                h3 { class: "text-xl font-bold text-gray-800 mb-4", "Included in Calculations" }
                div { class: "flex flex-wrap gap-2",
                    {
                        let sections = included_sections.read();
                        rsx! {
                            {sections.iter().map(|section_id| {
                                let badge_class = match section_id.as_str() {
                                    "hourly" | "salary" | "ot_bonus" | "commission" | "other_w2" => 
                                        "px-3 py-1 bg-blue-500 text-white rounded-full text-sm",
                                    _ => "px-3 py-1 bg-green-500 text-white rounded-full text-sm",
                                };
                                let label = match section_id.as_str() {
                                    "hourly" => "Hourly Pay",
                                    "salary" => "Salary",
                                    "ot_bonus" => "OT/Bonus",
                                    "commission" => "Commission",
                                    "other_w2" => "Other W2",
                                    "social_security" => "Social Security",
                                    "pension" => "Pension",
                                    "ira" => "IRA/401(k)",
                                    "other_income" => "Other Income",
                                    _ => section_id.as_str(),
                                };
                                rsx! {
                                    span { key: "{section_id}", class: "{badge_class}", "{label}" }
                                }
                            })}
                            if sections.is_empty() {
                                span { class: "text-gray-500 italic", "No income sources selected" }
                            }
                        }
                    }
                }
            }
        }
    }
}
