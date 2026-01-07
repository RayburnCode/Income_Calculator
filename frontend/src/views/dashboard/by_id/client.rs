use dioxus::prelude::*;
use crate::components::tab::{Tab, TabItem};
use crate::views::dashboard::by_id::income_worksheet::Worksheet;
use crate::views::dashboard::by_id::options_template::OptionsTemplate;

#[component]
pub fn ClientDetails(id: i32) -> Element {
    let mut active_tab = use_signal(|| 0);

    let tabs = vec![
        TabItem {
            label: "Overview".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
        TabItem {
            label: "Income Worksheet".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
        TabItem {
            label: "Options Template".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "mx-auto",
                // Header and Tabs on same line
                div { class: "mb-8 flex justify-between items-start",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Client Details" }
                        p { class: "text-gray-600 mt-2", "Details for client ID: {id}" }
                    }
                    div {
                        Tab {
                            tabs,
                            active_tab: *active_tab.read(),
                            on_tab_change: Some(
                                EventHandler::new(move |index: usize| {
                                    active_tab.set(index);
                                }),
                            ),
                        }
                    }
                }

                // Content based on active tab
                match *active_tab.read() {
                    0 => {
                        rsx! {
                            // Client Info Card
                            div { class: "bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Client Information" }
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {

                            // Additional sections can be added here

            

                                        label { class: "block text-sm font-medium text-gray-700", "Name" }
                                        p { class: "mt-1 text-sm text-gray-900", "John Doe" } // Placeholder
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Email" }
                                        p { class: "mt-1 text-sm text-gray-900", "john@example.com" } // Placeholder
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Income" }
                                        p { class: "mt-1 text-sm text-gray-900", "$50,000.00" } // Placeholder
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Status" }
                                        p { class: "mt-1 text-sm text-gray-900", "Active" } // Placeholder
                                    }
                                }
                            }
            
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
                    1 => {
                        rsx! {
                            Worksheet { id }
                        }
                    }
                    2 => {
                        rsx! {
                            OptionsTemplate { id }
                        }
                    }
                    _ => rsx! { "Invalid tab" },
                }
            }
        }
    }
}
