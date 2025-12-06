//Description, Type, Monthly Amount


use dioxus::prelude::*;
use crate::components::layout::layout::TotalIncome;

#[derive(Clone, PartialEq)]
struct IncomeItem {
    id: usize,
    description: String,
    income_type: String,
    monthly_amount: f64,
}

#[component]
pub fn Income() -> Element {
    let mut description = use_signal(|| String::new());
    let mut income_type = use_signal(|| String::new());
    let mut monthly_amount = use_signal(|| String::new());
    let mut incomes_list = use_signal(|| Vec::<IncomeItem>::new());
    let mut next_id = use_signal(|| 0usize);
    let mut editing_id = use_signal(|| None::<usize>);
    
    // Calculate total income
    let total_income = use_memo(move || {
        incomes_list.read().iter().map(|i| i.monthly_amount).sum::<f64>()
    });
    
    // Update global total_income context
    let TotalIncome(mut total_income_ctx) = use_context::<TotalIncome>();
    use_effect(use_reactive!(|(total_income,)| {
        total_income_ctx.set(total_income());
    }));
    
    // Listen for global reset signal and clear local state when it increments
    let reset = use_context::<Signal<usize>>();
    use_effect(move || {
        if reset() > 0 {
            incomes_list.set(Vec::new());
            next_id.set(0usize);
            editing_id.set(None);
            description.set(String::new());
            income_type.set(String::new());
            monthly_amount.set(String::new());
        }
    });
    
    // Helper for formatting money with commas
    fn format_money(amount: f64) -> String {
        let abs_amount = amount.abs();
        let formatted = format!("{:.2}", abs_amount);
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"00");
        
        let mut result = String::new();
        for (i, ch) in integer_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(',');
            }
            result.push(ch);
        }
        let formatted_int = result.chars().rev().collect::<String>();
        
        if amount < 0.0 && amount < -0.001 {
            format!("-${}.{}", formatted_int, decimal_part)
        } else {
            format!("${}.{}", formatted_int, decimal_part)
        }
    }
    
    // Helper to format input display with commas
    fn format_currency_input(value: &str) -> String {
        // Remove non-numeric characters except decimal point
        let cleaned: String = value.chars().filter(|c| c.is_numeric() || *c == '.').collect();
        if cleaned.is_empty() {
            return String::new();
        }
        
        // Don't format while typing - just return cleaned input
        // Only validate it's a valid number format
        if cleaned.contains('.') {
            let parts: Vec<&str> = cleaned.split('.').collect();
            if parts.len() == 2 {
                let integer_part = parts[0];
                let decimal_part = parts[1];
                
                // Add commas to integer part
                let mut result = String::new();
                for (i, ch) in integer_part.chars().rev().enumerate() {
                    if i > 0 && i % 3 == 0 {
                        result.push(',');
                    }
                    result.push(ch);
                }
                let formatted_int = result.chars().rev().collect::<String>();
                
                // Limit decimal to 2 places
                let limited_decimal = if decimal_part.len() > 2 {
                    &decimal_part[..2]
                } else {
                    decimal_part
                };
                
                format!("{}.{}", formatted_int, limited_decimal)
            } else {
                cleaned
            }
        } else {
            // Add commas to integer part only
            let mut result = String::new();
            for (i, ch) in cleaned.chars().rev().enumerate() {
                if i > 0 && i % 3 == 0 {
                    result.push(',');
                }
                result.push(ch);
            }
            result.chars().rev().collect::<String>()
        }
    }
    
    // Helper to parse currency input (removes commas)
    fn parse_currency(value: &str) -> Result<f64, std::num::ParseFloatError> {
        let cleaned: String = value.chars().filter(|c| c.is_numeric() || *c == '.').collect();
        cleaned.parse::<f64>()
    }
    
    rsx! {
        div { class: "space-y-4",
            // Input grid
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                div { class: "flex flex-col",
                    label {
                        r#for: "income_description",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Description"
                    }
                    input {
                        r#type: "text",
                        id: "income_description",
                        name: "income_description",
                        placeholder: "e.g., Company Name",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{description}",
                        oninput: move |evt| {
                            description.set(evt.value());
                        },
                    }
                }
                div { class: "flex flex-col",
                    label {
                        r#for: "income_type",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Type"
                    }
                    input {
                        r#type: "text",
                        id: "income_type",
                        name: "income_type",
                        placeholder: "e.g., W2, 1099",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{income_type}",
                        oninput: move |evt| {
                            income_type.set(evt.value());
                        },
                    }
                }
                div { class: "flex flex-col",
                    label {
                        r#for: "monthly_amount",
                        class: "text-sm font-medium text-gray-700 mb-1",
                        "Monthly Amount"
                    }
                    input {
                        r#type: "text",
                        id: "monthly_amount",
                        name: "monthly_amount",
                        placeholder: "0.00",
                        inputmode: "decimal",
                        class: "px-3 py-2 border border-gray-300 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition",
                        value: "{monthly_amount}",
                        oninput: move |evt| {
                            let formatted = format_currency_input(&evt.value());
                            monthly_amount.set(formatted);
                        },
                    }
                }
            }
            // Add/Update button
            button {
                class: "cursor-pointer px-4 py-2 bg-green-600 hover:bg-green-700 text-white font-medium rounded-lg transition duration-200 shadow-sm hover:shadow-md",
                onclick: move |_| {
                    let desc = description.read().clone();
                    let itype = income_type.read().clone();
                    let amount_str = monthly_amount.read().clone();
                    if !desc.is_empty() && !itype.is_empty() && !amount_str.is_empty() {
                        if let Ok(amount) = parse_currency(&amount_str) {
                            if let Some(edit_id) = editing_id() {
                                incomes_list
                                    .write()
                                    .iter_mut()
                                    .for_each(|income| {
                                        if income.id == edit_id {
                                            income.description = desc.clone();
                                            income.income_type = itype.clone();
                                            income.monthly_amount = amount;
                                        }
                                    });
                                editing_id.set(None);
                            } else {
                                let new_income = IncomeItem {
                                    id: next_id(),
                                    description: desc,
                                    income_type: itype,
                                    monthly_amount: amount,
                                };
                                incomes_list.write().push(new_income);
                                next_id.set(next_id() + 1);
                            }
                            description.set(String::new());
                            income_type.set(String::new());
                            monthly_amount.set(String::new());
                        }
                    }
                },
                if editing_id().is_some() {
                    "Update Income"
                } else {
                    "+ Add Income"
                }
            }
            // Incomes list
            if !incomes_list.read().is_empty() {
                div { class: "mt-6 space-y-2",
                    // Table header
                    div { class: "grid grid-cols-12 gap-2 px-3 py-2 bg-gray-100 rounded-lg font-semibold text-sm text-gray-700",
                        div { class: "col-span-4", "Description" }
                        div { class: "col-span-3", "Type" }
                        div { class: "col-span-3 text-right", "Monthly Amount" }
                        div { class: "col-span-2 text-center", "Actions" }
                    }
                    // Income rows
                    {
                        incomes_list
                            .read()
                            .iter()
                            .map(|income| {
                                let income_id = income.id;
                                let income_desc = income.description.clone();
                                let income_type_val = income.income_type.clone();
                                let income_amount = income.monthly_amount;
                                rsx! {
                                    div {
                                        key: "{income_id}",
                                        class: "grid grid-cols-12 gap-2 px-3 py-3 bg-gray-50 rounded-lg items-center hover:bg-gray-100 transition",
                                        div { class: "col-span-4 text-gray-800", "{income_desc}" }
                                        div { class: "col-span-3 text-gray-800", "{income_type_val}" }
                                        div { class: "col-span-3 text-right font-semibold text-gray-800 px-4",
                                            "{format_money(income_amount)}"
                                        }
                                        div { class: "col-span-2 flex gap-2 justify-center",
                                            // Edit button
                                            button {
                                                class: "cursor-pointer px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium rounded transition",
                                                onclick: move |_| {
                                                    if let Some(i) = incomes_list.read().iter().find(|i| i.id == income_id) {
                                                        description.set(i.description.clone());
                                                        income_type.set(i.income_type.clone());
                                                        monthly_amount.set(i.monthly_amount.to_string());
                                                        editing_id.set(Some(income_id));
                                                    }
                                                },
                                                "Edit"
                                            }
                                            // Delete button
                                            button {
                                                class: "cursor-pointer px-3 py-1 bg-red-500 hover:bg-red-600 text-white text-xs font-medium rounded transition",
                                                onclick: move |_| {
                                                    incomes_list.write().retain(|i| i.id != income_id);
                                                    if editing_id() == Some(income_id) {
                                                        editing_id.set(None);
                                                        description.set(String::new());
                                                        income_type.set(String::new());
                                                        monthly_amount.set(String::new());
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
            // Total display
            div { class: "mt-6 pt-4 border-t border-gray-200",
                div { class: "flex justify-between items-center",
                    span { class: "text-lg font-semibold text-gray-700", "Total Monthly Income:" }
                    span { class: "text-2xl font-bold text-green-600", "{format_money(total_income())}" }
                }
            }
        }
    }
}