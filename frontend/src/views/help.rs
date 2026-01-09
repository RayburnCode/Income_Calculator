use dioxus::prelude::*;

#[component]
pub fn Help() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto p-6 space-y-6",
            // Page header
            div { class: "text-center",
                h1 { class: "text-3xl font-bold", "Help & Documentation" }
                p { class: "text-gray-200 mt-2",
                    "Complete guide to using the Income Calculator application for mortgage professionals and financial advisors."
                }
            }

            // Main grid
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                // Left / main column
                div { class: "lg:col-span-2 space-y-6",

                    // Getting Started
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: "text-2xl text-gray-900 font-semibold mb-2", "Getting Started" }
                        p { class: "text-gray-900 mb-3",
                            "Welcome to the Income Calculator! This application helps mortgage professionals manage clients, calculate income qualifications, and analyze mortgage refinance options."
                        }
                        ul { class: "text-gray-900",
                            li { class: "mb-1",
                                "1. The Dashboard shows your client overview and key metrics."
                            }
                            li { class: "mb-1",
                                "2. Add clients using the 'Add Client' button to start managing borrowers."
                            }
                            li { class: "mb-1",
                                "3. Use Analytics to view real-time business insights and client metrics."
                            }
                            li { class: "mb-1",
                                "4. Access individual client details to manage income, debts, and loan options."
                            }
                        }
                        p { class: "text-sm text-gray-700 mt-3",
                            "Tip: All data is stored locally in a secure SQLite database."
                        }
                    }

                    // Client Management
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: "text-gray-900 text-2xl font-semibold mb-2", "Client Management" }
                        p { class: "text-gray-900 mb-2", "How to manage your clients:" }
                        dl {
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Adding Clients" }
                                dd { class: "text-gray-700",
                                    "Click 'Add Client' on the dashboard. Enter first name, last name, email, and phone number. Phone numbers are automatically formatted as (XXX)XXX-XXXX."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Client Details" }
                                dd { class: "text-gray-700",
                                    "Click on any client in the table to access their detailed profile, including income worksheets and mortgage options."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Income Worksheets" }
                                dd { class: "text-gray-700",
                                    "Add borrower and coborrower income information, including monthly amounts, ratios, and debt details."
                                }
                            }
                            div { class: "mb-2",
                                dt { class: "font-medium text-gray-900", "Mortgage Options" }
                                dd { class: "text-gray-700",
                                    "Create and compare mortgage refinance options with detailed calculations, pricing, and savings analysis."
                                }
                            }
                        }
                    }

                    // Analytics & Reporting
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h2 { class: " text-gray-900 text-2xl font-semibold mb-2",
                            "Analytics Dashboard"
                        }
                        p { class: "text-gray-900 mb-2", "Understanding your business metrics:" }
                        pre { class: "bg-gray-100 dark:bg-gray-700 p-3 rounded text-sm text-gray-900 dark:text-gray-100",
                            "📊 Total Clients: Current number of active borrowers\n💰 Total Income: Combined monthly income across all clients\n📈 Average Income: Mean income per client\n🏠 Total Loans: Number of loan applications processed"
                        }
                        p { class: "text-gray-900 mt-3",
                            "Analytics automatically update as you add clients and income data. All calculations are performed in real-time from your local database."
                        }
                        p { class: "text-sm text-gray-700 mt-2",
                            "Note: Analytics focus on business metrics, not individual client financial details."
                        }
                    }
                }

                // Right column / sidebar
                div { class: "space-y-6",

                    // FAQ
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: " text-gray-900 text-xl font-semibold mb-2",
                            "Frequently Asked Questions"
                        }
                        dl {
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900", "Is my data secure?" }
                                dd { class: "text-gray-700",
                                    "Yes! All data is stored locally in an encrypted SQLite database. No information is sent to external servers."
                                }
                            }
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900", "Can I backup my data?" }
                                dd { class: "text-gray-700",
                                    "The database file (income_calculator.db) can be copied to create backups. Keep it in a secure location."
                                }
                            }
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900",
                                    "How do I update client information?"
                                }
                                dd { class: "text-gray-700",
                                    "Click on any client in the dashboard table to access their detailed profile and edit information."
                                }
                            }
                            div { class: "mb-3",
                                dt { class: "font-medium text-gray-900",
                                    "What calculations are performed?"
                                }
                                dd { class: "text-gray-700",
                                    "DTI ratios, loan-to-value calculations, payment amounts, savings analysis, and refinance comparisons."
                                }
                            }
                        }
                    }

                    // Data Management
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: " text-gray-900 text-xl font-semibold mb-2", "Data Management" }
                        ul { class: "text-gray-900",
                            li { class: "mb-2",
                                "Client data includes personal information, income details, and loan preferences."
                            }
                            li { class: "mb-2",
                                "Income worksheets track borrower qualifications and debt-to-income ratios."
                            }
                            li { class: "mb-2",
                                "Mortgage options include loan terms, rates, payments, and savings calculations."
                            }
                            li { class: "mb-2",
                                "All data is automatically saved to the local database as you work."
                            }
                        }
                    }

                    // Technical Support
                    div { class: "bg-white rounded-lg shadow-md p-6 border border-gray-200",
                        h3 { class: "text-gray-900 text-xl font-semibold mb-2", "Technical Support" }
                        p { class: "text-gray-900 mb-2",
                            "Need help with the application? Here are some common solutions:"
                        }
                        ul { class: "text-gray-900 text-sm",
                            li { class: "mb-1", "• Restart the application if data doesn't load" }
                            li { class: "mb-1",
                                "• Check that the database file exists in the application directory"
                            }
                            li { class: "mb-1",
                                "• Ensure you have write permissions for data storage"
                            }
                            li { class: "mb-1", "• Clear browser cache if using web version" }
                        }
                        p { class: "text-sm text-blue-600 mt-3",
                            a {
                                href: "mailto:support@incomecalculator.app",
                                class: "hover:underline",
                                "Contact Support"
                            }
                        }
                    }
                }
            }

            // Footer card: privacy / version
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 border border-gray-200 dark:border-gray-600 flex flex-col md:flex-row md:justify-between items-start md:items-center",
                p { class: "text-sm text-gray-600",
                    "Privacy: All client and financial data remains local to your device. No data is transmitted to external services or cloud storage."
                }
                p { class: "text-sm text-gray-600 mt-3 md:mt-0",
                    "Version: 1.0.0 - Desktop Application"
                }
            }
        }
    }
}


