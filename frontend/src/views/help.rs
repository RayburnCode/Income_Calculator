use dioxus::prelude::*;


#[component]
pub fn Help() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto p-6 space-y-6",
            // Page header
            div { class: "text-center",
                h1 { class: "text-3xl font-bold", "Help & Documentation" }
                p { class: "text-gray-200 mt-2",
                    "Guides, FAQs and examples for both consumers and industry professionals. This page does not provide rate or investment advice."
                }
            }

            // Main grid
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                // Left / main column
                div { class: "lg:col-span-2 space-y-6",

                    // Quick Start
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: "text-2xl text-gray-900 font-semibold mb-2", "Quick Start" }
                        p { class: "text-gray-900 mb-3",
                            "A fast walkthrough to get you calculating your Debt-to-Income (DTI) ratio."
                        }
                        ul { class: "text-gray-900",
                            li { class: "mb-1",
                                "1. Add one or more monthly incomes under the Income section."
                            }
                            li { class: "mb-1",
                                "2. Add recurring monthly debts under the Debts section (loans, credit cards, etc.)."
                            }
                            li { class: "mb-1",
                                "3. Enter housing-related payments in Housing Payment (principal, taxes, insurance, HOA)."
                            }
                            li { class: "mb-1", "4. Review the Results card for your calculated DTI." }
                        }
                        p { class: "text-sm text-gray-700 mt-3",
                            "Tip: Use the Add / Edit buttons to manage multiple incomes or debts."
                        }
                    }

                    // Inputs & Fields details
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: "text-gray-900 text-2xl font-semibold mb-2",
                            "Inputs & Field Guide"
                        }
                        p { class: "text-gray-900 mb-2", "What to enter in each field:" }
                        dl {
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Description" }
                                dd { class: "text-gray-700",
                                    "A short label for the income or debt (e.g., 'Salary', 'Car loan')."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Type" }
                                dd { class: "text-gray-700",
                                    "Optional class (W2, 1099, Auto, Student, etc.)."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Monthly Amount" }
                                dd { class: "text-gray-700",
                                    "Enter the recurring monthly value. Use numbers only (decimals allowed)."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Housing fields" }
                                dd { class: "text-gray-700",
                                    "Principal & Interest, Property Taxes, Homeowners Insurance, HOA Fees — all monthly amounts."
                                }
                            }
                        }
                    }

                    // Examples
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: " text-gray-900 text-2xl font-semibold mb-2", "Examples" }
                        p { class: "text-gray-900 mb-2", "Example (consumer):" }
                        pre { class: "bg-gray-100 p-3 rounded text-sm text-gray-900",
                            "Income: $5,000\nMortgage: $1,500\nOther debts: $300\n→ DTI = (1,500 + 300) / 5,000 = 36%"
                        }
                        p { class: "text-gray-900 mt-3",
                            "Example (multiple incomes): combine monthly income totals before dividing."
                        }
                        p { class: "text-sm text-gray-700 mt-2",
                            "Note: These are illustrative examples only and not financial advice."
                        }
                    }
                }

                // Right column / sidebar
                div { class: "space-y-6",

                    // FAQ
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: " text-gray-900 text-xl font-semibold mb-2", "FAQ" }
                        dl {
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900", "What counts as debt?" }
                                dd { class: "text-gray-700",
                                    "Recurring monthly obligations such as loan payments, minimum credit card payments, alimony, etc."
                                }
                            }
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900", "Should I include utilities?" }
                                dd { class: "text-gray-700",
                                    "Typically utilities are not included in DTI calculations unless they are a contractual monthly obligation."
                                }
                            }
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900",
                                    "How are multiple incomes handled?"
                                }
                                dd { class: "text-gray-700",
                                    "Add each income separately; the app sums them for the total monthly income used in the DTI calculation."
                                }
                            }
                        }
                    }

                    // Troubleshooting
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: " text-gray-900 text-xl font-semibold mb-2", "Troubleshooting" }
                        ul { class: "text-gray-900",
                            li { class: "mb-2",
                                "If an input doesn't update, try clicking the field and retyping the value (some platforms require focus to trigger updates)."
                            }
                            li { class: "mb-2",
                                "Make sure numeric values use a dot for decimals (e.g., 1234.56)."
                            }
                            li { class: "mb-2",
                                "If the app doesn't reload after changes, restart the dev server: `cargo run` / frontend dev command."
                            }
                        }
                    }

                    // Contact / feedback
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: "text-gray-900 text-xl font-semibold mb-2", "Contact & Feedback" }
                        p { class: "text-gray-900 mb-2",
                            "Bugs, feature requests, or professional feedback — open an issue on the repo or email:"
                        }
                        p { class: "text-sm text-blue-600",
                            a {
                                href: "mailto:hello@example.com",
                                class: "hover:underline",
                                "hello@example.com"
                            }
                        }
                    }
                }
            }

            // Footer card: privacy / version
            div { class: "bg-white rounded-lg shadow-md p-4 border border-gray-200 flex flex-col md:flex-row md:justify-between items-start md:items-center",
                p { class: "text-sm text-gray-600",
                    "Privacy: This app runs locally in the browser / desktop shell. No personal financial data is sent to external services."
                }
                p { class: "text-sm text-gray-600 mt-3 md:mt-0", "Version: dev" }
            }
        }
    }
}


