use dioxus::prelude::*;
use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Welcome() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            // Hero Section
            div { class: "container mx-auto px-6 py-16",
                div { class: "text-center",
                    h1 { class: "text-5xl font-bold text-gray-900 dark:text-gray-100 mb-6",
                        "Income Calculator"
                    }
                    p { class: "text-xl text-gray-600 dark:text-gray-300 mb-8 max-w-2xl mx-auto",
                        "Take control of your financial future with our comprehensive income calculation tools. Calculate mortgages, analyze debt-to-income ratios, and plan your financial goals with confidence."
                    }
                    div { class: "flex flex-col sm:flex-row gap-4 justify-center",
                        Link {
                            to: Route::MainDashboard {},
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-8 rounded-lg transition duration-300 shadow-lg hover:shadow-xl",
                            "Get Started"
                        }
                        button {
                            class: "bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 font-semibold py-3 px-8 rounded-lg border border-gray-300 dark:border-gray-600 transition duration-300 shadow-lg hover:shadow-xl",
                            onclick: |_| {},
                            "Learn More"
                        }
                    }
                }
            }

            // Features Section
            div { class: "container mx-auto px-6 py-16",
                div { class: "text-center mb-12",
                    h2 { class: "text-3xl font-bold text-gray-900 dark:text-gray-100 mb-4",
                        "Powerful Financial Tools"
                    }
                    p { class: "text-gray-600 dark:text-gray-300 max-w-2xl mx-auto",
                        "Our suite of calculators helps you make informed financial decisions with accurate, real-time calculations."
                    }
                }

                div { class: "grid md:grid-cols-3 gap-8",
                    // Feature 1
                    div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg hover:shadow-xl transition duration-300",
                        div { class: "w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4",
                            svg {
                                class: "w-6 h-6 text-blue-600",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" }
                            }
                        }
                        h3 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2",
                            "Mortgage Calculator"
                        }
                        p { class: "text-gray-600 dark:text-gray-300",
                            "Calculate monthly payments, total interest, and amortization schedules for your mortgage."
                        }
                    }

                    // Feature 2
                    div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg hover:shadow-xl transition duration-300",
                        div { class: "w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4",
                            svg {
                                class: "w-6 h-6 text-green-600",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                            }
                        }
                        h3 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2",
                            "Income Analysis"
                        }
                        p { class: "text-gray-600 dark:text-gray-300",
                            "Analyze your income sources, calculate net income, and understand your financial position."
                        }
                    }

                    // Feature 3
                    div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg hover:shadow-xl transition duration-300",
                        div { class: "w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4",
                            svg {
                                class: "w-6 h-6 text-purple-600",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path { d: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" }
                            }
                        }
                        h3 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2",
                            "Debt-to-Income Ratio"
                        }
                        p { class: "text-gray-600 dark:text-gray-300",
                            "Calculate your DTI ratio and understand how lenders view your financial health."
                        }
                    }
                }
            }

            // Stats Section
            div { class: "bg-white dark:bg-gray-800 py-16",
                div { class: "container mx-auto px-6",
                    div { class: "grid md:grid-cols-4 gap-8 text-center",
                        div {
                            div { class: "text-3xl font-bold text-blue-600 mb-2", "10K+" }
                            p { class: "text-gray-600 dark:text-gray-300", "Calculations Made" }
                        }
                        div {
                            div { class: "text-3xl font-bold text-green-600 mb-2", "95%" }
                            p { class: "text-gray-600 dark:text-gray-300", "Accuracy Rate" }
                        }
                        div {
                            div { class: "text-3xl font-bold text-purple-600 mb-2",
                                "24/7"
                            }
                            p { class: "text-gray-600 dark:text-gray-300", "Available" }
                        }
                        div {
                            div { class: "text-3xl font-bold text-orange-600 mb-2",
                                "Free"
                            }
                            p { class: "text-gray-600 dark:text-gray-300", "No Hidden Fees" }
                        }
                    }
                }
            }

            // CTA Section
            div { class: "bg-blue-600 py-16",
                div { class: "container mx-auto px-6 text-center",
                    h2 { class: "text-3xl font-bold text-white mb-4",
                        "Ready to Take Control of Your Finances?"
                    }
                    p { class: "text-blue-100 dark:text-blue-200 mb-8 max-w-2xl mx-auto",
                        "Start calculating today and make informed decisions about your financial future."
                    }
                    Link {
                        to: Route::MainDashboard {},
                        class: "bg-white dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700 text-blue-600 dark:text-blue-400 font-semibold py-3 px-8 rounded-lg transition duration-300 shadow-lg hover:shadow-xl",
                        "Start Calculating"
                    }
                }
            }
        }
    }
}
