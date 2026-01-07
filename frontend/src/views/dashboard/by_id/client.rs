use dioxus::prelude::*;
use crate::components::tab::{Tab, TabItem};
use crate::views::dashboard::by_id::income_worksheet::Worksheet;
use crate::views::dashboard::by_id::options_template::OptionsTemplate;
use crate::views::dashboard::by_id::income_worksheet::W2Jobs;

#[component]
pub fn ClientDetails(id: i32) -> Element {
    let mut active_tab = use_signal(|| 0);

    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        client::Client::new().await
    });

    // State for borrower data
    let borrower = use_signal(|| None::<shared::models::Borrower>);
    let error_message = use_signal(|| None::<String>);

    // Load borrower when the resource is ready
    use_effect(move || {
        let resource_value = client_resource.read().clone();
        let mut borrower = borrower.clone();
        let mut error_message = error_message.clone();
        let client_id = id;
        
        spawn(async move {
            match resource_value.as_ref() {
                Some(Ok(db_client)) => {
                    // Load borrower from database
                    match db_client.get_borrower(client_id).await {
                        Ok(Some(borrower_data)) => {
                            borrower.set(Some(borrower_data));
                            error_message.set(None);
                        }
                        Ok(None) => {
                            error_message.set(Some(format!("Borrower with ID {} not found", client_id)));
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Error loading borrower: {}", e)));
                        }
                    }
                }
                Some(Err(e)) => {
                    // Database connection failed
                    error_message.set(Some(e.clone()));
                }
                None => {
                    // Still loading
                }
            }
        });
    });

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
                        let content: Element = rsx! {
                            // Error message
                            if let Some(error) = error_message() {
                                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                                    "{error}"
                                }
                            }

                            // Client Info Card

            

                            div { class: "bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Client Information" }
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Name" }
                                        p { class: "mt-1 text-sm text-gray-900",
                                            if let Some(borrower_data) = borrower() {
                                                "{borrower_data.name}"
                                            } else {
                                                "Loading..."
                                            }
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Employer" }
                                        p { class: "mt-1 text-sm text-gray-900",
                                            if let Some(borrower_data) = borrower() {
                                                "{borrower_data.employer_name.as_deref().unwrap_or(\"N/A\")}"
                                            } else {
                                                "Loading..."
                                            }
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Income Type" }
                                        p { class: "mt-1 text-sm text-gray-900",
                                            if let Some(borrower_data) = borrower() {
                                                "{borrower_data.income_type.as_deref().unwrap_or(\"N/A\")}"
                                            } else {
                                                "Loading..."
                                            }
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700", "Loan Number" }
                                        p { class: "mt-1 text-sm text-gray-900",
                                            if let Some(borrower_data) = borrower() {
                                                "{borrower_data.loan_number.as_deref().unwrap_or(\"N/A\")}"
                                            } else {
                                                "Loading..."
                                            }
                                        }
                                    }
                                }
                            }
            
                            div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Income Information" }
                                W2Jobs {}
                            }
            
                            div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Loan Information" }
                                p { class: "text-gray-600", "Loan details will be displayed here." }
                            }
                        };
                        content
                    }
                    1 => {
                        let content: Element = rsx! {
                            Worksheet { id }
                        };
                        content
                    }
                    2 => {
                        let content: Element = rsx! {
                            OptionsTemplate { id }
                        };
                        content
                    }
                    _ => {
                        let content: Element = rsx! { "Invalid tab" };
                        content
                    }
                }
            }
        }
    }
}
