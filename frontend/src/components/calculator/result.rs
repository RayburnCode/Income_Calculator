use dioxus::prelude::*;
use crate::components::layout::layout::{TotalIncome, TotalDebt, TotalHousing};

#[component]
pub fn Result() -> Element {
    // Get totals from context
    let TotalIncome(total_income) = use_context::<TotalIncome>();
    let TotalDebt(total_debt) = use_context::<TotalDebt>();
    let TotalHousing(total_housing) = use_context::<TotalHousing>();
    
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
    
    // Calculate DTI ratios
    let frontend_dti = use_memo(move || {
        let income = total_income();
        if income > 0.0 {
            (total_housing() / income) * 100.0
        } else {
            0.0
        }
    });
    
    let backend_dti = use_memo(move || {
        let income = total_income();
        if income > 0.0 {
            ((total_housing() + total_debt()) / income) * 100.0
        } else {
            0.0
        }
    });
    
    // Check if we have any data entered
    let has_data = use_memo(move || {
        total_income() > 0.0 || total_debt() > 0.0 || total_housing() > 0.0
    });
    
    rsx! {
        if has_data() {
            div { class: "space-y-6",
                // Calculation Breakdown
                div { class: "bg-gray-50 rounded-lg p-4 border border-gray-300",
                    h4 { class: "text-sm font-semibold text-gray-700 mb-3", "Calculation Breakdown" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-3 text-sm",
                        div { class: "flex justify-between items-center p-2 bg-white rounded border border-gray-200",
                            span { class: "text-gray-600", "Monthly Income:" }
                            span { class: "font-semibold text-green-600",
                                "{format_money(total_income())}"
                            }
                        }
                        div { class: "flex justify-between items-center p-2 bg-white rounded border border-gray-200",
                            span { class: "text-gray-600", "Housing Payment:" }
                            span { class: "font-semibold text-blue-600",
                                "{format_money(total_housing())}"
                            }
                        }
                        div { class: "flex justify-between items-center p-2 bg-white rounded border border-gray-200",
                            span { class: "text-gray-600", "Other Debts:" }
                            span { class: "font-semibold text-red-600", "{format_money(total_debt())}" }
                        }
                    }
                }
                // DTI Cards
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Frontend DTI Card
                    div { class: "bg-gradient-to-br from-blue-50 to-blue-100 rounded-lg shadow-lg p-6 border-2 border-blue-300",
                        h3 { class: "text-xl font-bold text-blue-900 mb-2", "Front-End DTI" }
                        p { class: "text-sm text-blue-700 mb-4", "Housing expenses only" }
                        div { class: "text-center",
                            p { class: "text-5xl font-bold text-blue-600 mb-2",
                                "{frontend_dti():.2}%"
                            }
                            p { class: "text-xs text-blue-600 mt-2", "Mortgage PITI ÷ Gross Income" }
                        }
                        div { class: "mt-4 pt-4 border-t border-blue-300",
                            if total_income() == 0.0 {
                                p { class: "text-sm text-gray-600 italic",
                                    "Please enter income for more accurate result"
                                }
                            } else if frontend_dti() <= 28.0 {
                                p { class: "text-sm text-green-700 font-semibold",
                                    "✓ Excellent - Within recommended range"
                                }
                            } else if frontend_dti() <= 40.0 {
                                p { class: "text-sm text-green-700 font-semibold",
                                    "✓ Acceptable - Within recommended range"
                                }
                            } else if frontend_dti() <= 45.0 {
                                p { class: "text-sm text-yellow-700 font-semibold",
                                    "⚠ Caution - Above ideal range, may require additional review"
                                }
                            } else {
                                p { class: "text-sm text-red-700 font-semibold",
                                    "⚠ High - Consider Reducing Housing Costs or paying Down Debt"
                                }
                            }
                        }
                    }
                    // Backend DTI Card
                    div { class: "bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg shadow-lg p-6 border-2 border-purple-300",
                        h3 { class: "text-xl font-bold text-purple-900 mb-2", "Back-End DTI" }
                        p { class: "text-sm text-purple-700 mb-4", "All monthly debt payments" }
                        div { class: "text-center",
                            p { class: "text-5xl font-bold text-purple-600 mb-2",
                                "{backend_dti():.2}%"
                            }
                            p { class: "text-xs text-purple-600 mt-2",
                                "(Mortgage PITI + Debts) ÷ Gross Income"
                            }
                        }
                        div { class: "mt-4 pt-4 border-t border-purple-300",
                            if total_income() == 0.0 {
                                p { class: "text-sm text-gray-600 italic",
                                    "Please enter income for more accurate result"
                                }
                            } else if backend_dti() <= 28.0 {
                                p { class: "text-sm text-green-700 font-semibold",
                                    "✓ Excellent - Within recommended range"
                                }
                            } else if backend_dti() <= 40.0 {
                                p { class: "text-sm text-green-700 font-semibold",
                                    "✓ Acceptable - Within recommended range"
                                }
                            } else if backend_dti() <= 45.0 {
                                p { class: "text-sm text-yellow-700 font-semibold",
                                    "⚠ Caution - Above ideal range, may require additional review"
                                }
                            } else {
                                p { class: "text-sm text-red-700 font-semibold",
                                    "⚠ High - May impact approval"
                                }
                            }
                        }
                    }
                }
            }
        } else {
            div { class: "bg-gray-50 rounded-lg shadow-md p-8 text-center border border-gray-200",
                p { class: "text-gray-600 text-lg",
                    "Enter your income, housing payment, and debts above to see your DTI ratios"
                }
            }
        }
    }
}