use dioxus::prelude::*;

#[component]
pub fn Reports() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Reports" }
                    p { class: "text-gray-600 mt-2", "Generate and view detailed financial reports" }
                }

                // Placeholder content
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Available Reports" }
                    div { class: "space-y-4",
                        div { class: "p-4 border border-gray-200 rounded-lg",
                            h3 { class: "font-semibold text-gray-800", "Income Analysis Report" }
                            p { class: "text-gray-600 mt-1",
                                "Comprehensive analysis of client income data"
                            }
                            button { class: "mt-2 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
                                "Generate Report"
                            }
                        }
                        div { class: "p-4 border border-gray-200 rounded-lg",
                            h3 { class: "font-semibold text-gray-800", "Loan Performance Report" }
                            p { class: "text-gray-600 mt-1", "Track loan performance and metrics" }
                            button { class: "mt-2 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
                                "Generate Report"
                            }
                        }
                        div { class: "p-4 border border-gray-200 rounded-lg",
                            h3 { class: "font-semibold text-gray-800", "Client Summary Report" }
                            p { class: "text-gray-600 mt-1",
                                "Overview of all client data and statistics"
                            }
                            button { class: "mt-2 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
                                "Generate Report"
                            }
                        }
                    }
                }
            }
        }
    }
}