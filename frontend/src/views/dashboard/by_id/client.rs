use dioxus::prelude::*;

#[component]
pub fn ClientDetails(id: i32) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-4xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Client Details" }
                    p { class: "text-gray-600 mt-2", "Details for client ID: {id}" }
                }

                // Client Info Card
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Client Information" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Name"
                            }
                            p { class: "mt-1 text-sm text-gray-900", "John Doe" } // Placeholder
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Email"
                            }
                            p { class: "mt-1 text-sm text-gray-900", "john@example.com" } // Placeholder
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Income"
                            }
                            p { class: "mt-1 text-sm text-gray-900", "$50,000.00" } // Placeholder
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Status"
                            }
                            p { class: "mt-1 text-sm text-gray-900", "Active" } // Placeholder
                        }
                    }
                }

                // Additional sections can be added here
                div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Income Information" }
                    p { class: "text-gray-600", "Income details will be displayed here." }
                }

                div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Loan Information" }
                    p { class: "text-gray-600", "Loan details will be displayed here." }
                }
            }
        }
    }
}
