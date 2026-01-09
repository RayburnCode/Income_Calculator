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
                div { class: "bg-theme-surface-100 dark:bg-theme-surface-800 rounded-lg p-4 border border-theme-border-300 dark:border-theme-border-600",
                    h4 { class: "text-sm font-semibold text-theme-text-700 dark:text-theme-text-200 mb-3",
                        "Calculation Breakdown"
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-3 text-sm",
                        div { class: "flex justify-between items-center p-2 bg-theme-surface-50 dark:bg-theme-surface-900 rounded border border-theme-border-200 dark:border-theme-border-700",
                            span { class: "text-theme-text-600 dark:text-theme-text-300",
                                "Monthly Income:"
                            }
                            span { class: "font-semibold text-theme-success-600 dark:text-theme-success-400",
                                "{format_money(total_income())}"
                            }
                        }
                        div { class: "flex justify-between items-center p-2 bg-theme-surface-50 dark:bg-theme-surface-900 rounded border border-theme-border-200 dark:border-theme-border-700",
                            span { class: "text-theme-text-600 dark:text-theme-text-300",
                                "Housing Payment:"
                            }
                            span { class: "font-semibold text-theme-primary-600 dark:text-theme-primary-400",
                                "{format_money(total_housing())}"
                            }
                        }
                        div { class: "flex justify-between items-center p-2 bg-theme-surface-50 dark:bg-theme-surface-900 rounded border border-theme-border-200 dark:border-theme-border-700",
                            span { class: "text-theme-text-600 dark:text-theme-text-300",
                                "Other Debts:"
                            }
                            span { class: "font-semibold text-theme-error-600 dark:text-theme-error-400",
                                "{format_money(total_debt())}"
                            }
                        }
                    }
                }
                // DTI Cards
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Frontend DTI Card
                    div { class: "bg-gradient-to-br from-theme-primary-50 dark:from-theme-primary-900/20 to-theme-primary-100 dark:to-theme-primary-800/20 rounded-lg shadow-lg p-6 border-2 border-theme-primary-300 dark:border-theme-primary-600",
                        h3 { class: "text-xl font-bold text-theme-primary-900 dark:text-theme-primary-100 mb-2",
                            "Front-End DTI"
                        }
                        p { class: "text-sm text-theme-primary-700 dark:text-theme-primary-300 mb-4",
                            "Housing expenses only"
                        }
                        div { class: "text-center",
                            p { class: "text-5xl font-bold text-theme-primary-600 dark:text-theme-primary-400 mb-2",
                                "{frontend_dti():.2}%"
                            }
                            p { class: "text-xs text-theme-primary-600 dark:text-theme-primary-400 mt-2",
                                "Mortgage PITI ÷ Gross Income"
                            }
                        }
                        div { class: "mt-4 pt-4 border-t border-theme-primary-300 dark:border-theme-primary-600",
                            if total_income() == 0.0 {
                                p { class: "text-sm text-theme-text-600 dark:text-theme-text-300 italic",
                                    "Please enter income for more accurate result"
                                }
                            } else if frontend_dti() <= 28.0 {
                                p { class: "text-sm text-theme-success-700 dark:text-theme-success-300 font-semibold",
                                    "✓ Excellent - Within recommended range"
                                }
                            } else if frontend_dti() <= 40.0 {
                                p { class: "text-sm text-theme-success-700 dark:text-theme-success-300 font-semibold",
                                    "✓ Acceptable - Within recommended range"
                                }
                            } else if frontend_dti() <= 45.0 {
                                p { class: "text-sm text-theme-warning-700 dark:text-theme-warning-300 font-semibold",
                                    "⚠ Caution - Above ideal range, may require additional review"
                                }
                            } else {
                                p { class: "text-sm text-theme-error-700 dark:text-theme-error-300 font-semibold",
                                    "⚠ High - Consider Reducing Housing Costs or paying Down Debt"
                                }
                            }
                        }
                    }
                    // Backend DTI Card
                    div { class: "bg-gradient-to-br from-theme-success-50 dark:from-theme-success-900/20 to-theme-success-100 dark:to-theme-success-800/20 rounded-lg shadow-lg p-6 border-2 border-theme-success-300 dark:border-theme-success-600",
                        h3 { class: "text-xl font-bold text-theme-success-900 dark:text-theme-success-100 mb-2",
                            "Back-End DTI"
                        }
                        p { class: "text-sm text-theme-success-700 dark:text-theme-success-300 mb-4",
                            "All monthly debt payments"
                        }
                        div { class: "text-center",
                            p { class: "text-5xl font-bold text-theme-success-600 dark:text-theme-success-400 mb-2",
                                "{backend_dti():.2}%"
                            }
                            p { class: "text-xs text-theme-success-600 dark:text-theme-success-400 mt-2",
                                "(Mortgage PITI + Debts) ÷ Gross Income"
                            }
                        }
                        div { class: "mt-4 pt-4 border-t border-theme-success-300 dark:border-theme-success-600",
                            if total_income() == 0.0 {
                                p { class: "text-sm text-theme-text-600 dark:text-theme-text-300 italic",
                                    "Please enter income for more accurate result"
                                }
                            } else if backend_dti() <= 28.0 {
                                p { class: "text-sm text-theme-success-700 dark:text-theme-success-300 font-semibold",
                                    "✓ Excellent - Within recommended range"
                                }
                            } else if backend_dti() <= 40.0 {
                                p { class: "text-sm text-theme-success-700 dark:text-theme-success-300 font-semibold",
                                    "✓ Acceptable - Within recommended range"
                                }
                            } else if backend_dti() <= 45.0 {
                                p { class: "text-sm text-theme-warning-700 dark:text-theme-warning-300 font-semibold",
                                    "⚠ Caution - Above ideal range, may require additional review"
                                }
                            } else {
                                p { class: "text-sm text-theme-error-700 dark:text-theme-error-300 font-semibold",
                                    "⚠ High - May impact approval"
                                }
                            }
                        }
                    }
                }
            }
        } else {
            div { class: "bg-theme-surface-100 dark:bg-theme-surface-800 rounded-lg shadow-md p-8 text-center border border-theme-border-200 dark:border-theme-border-700",
                p { class: "text-theme-text-600 dark:text-theme-text-300 text-lg",
                    "Enter your income, housing payment, and debts above to see your DTI ratios"
                }
            }
        }
    }
}