use dioxus::prelude::*;
use crate::components::tab::{Tab, TabItem};
use crate::views::dashboard::by_id::income_worksheet::Worksheet;
use crate::views::dashboard::by_id::options_template::OptionsTemplate;
use crate::views::dashboard::by_id::income_worksheet::W2Jobs;
use chrono::Utc;
use shared::models::Status;

#[component]
pub fn ClientDetails(id: i32) -> Element {
    let mut active_tab = use_signal(|| 0);

    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        repository::Repository::new().await
    });

    // State for borrower data
    let borrower = use_signal(|| None::<shared::models::Borrower>);
    let error_message = use_signal(|| None::<String>);
    let mut is_editing = use_signal(|| false);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_status = use_signal(|| Status::Active);
    let mut edit_email = use_signal(|| String::new());
    let mut edit_phone = use_signal(|| String::new());

    // Load borrower when the resource is ready
    use_effect(move || {
        let resource_value = client_resource.read().clone();
        let mut borrower = borrower.clone();
        let mut error_message = error_message.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_phone = edit_phone.clone();
        let client_id = id;
        
        spawn(async move {
            match resource_value.as_ref() {
                Some(Ok(db_client)) => {
                    // Load borrower from database
                    match db_client.get_borrower(client_id).await {
                        Ok(Some(borrower_data)) => {
                            borrower.set(Some(borrower_data.clone()));
                            // Populate edit fields
                            edit_name.set(borrower_data.name.clone());
                            edit_status.set(borrower_data.status.unwrap_or(Status::Active));
                            edit_email.set(borrower_data.email.unwrap_or_default());
                            edit_phone.set(borrower_data.phone_number.unwrap_or_default());
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

    // Save borrower changes
    let save_changes = {
        let client_resource = client_resource.clone();
        let borrower = borrower.clone();
        let mut error_message = error_message.clone();
        let mut is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_status = edit_status.clone();
        let edit_email = edit_email.clone();
        let edit_phone = edit_phone.clone();
        
        move || {
            let mut borrower_clone = borrower.clone();
            spawn(async move {
                if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                    if let Some(mut borrower_data) = borrower_clone() {
                        // Update the borrower data
                        borrower_data.name = edit_name();
                        borrower_data.status = Some(edit_status());
                        borrower_data.email = if edit_email().is_empty() { None } else { Some(edit_email()) };
                        borrower_data.phone_number = if edit_phone().is_empty() { None } else { Some(edit_phone()) };
                        borrower_data.updated_at = Utc::now();
                        
                        match db_client.update_borrower(borrower_data.clone()).await {
                            Ok(_) => {
                                borrower_clone.set(Some(borrower_data));
                                error_message.set(None);
                                is_editing.set(false);
                            }
                            Err(e) => {
                                error_message.set(Some(format!("Error updating borrower: {}", e)));
                            }
                        }
                    }
                }
            });
        }
    };

    // Cancel editing
    let mut cancel_edit = {
        let mut is_editing = is_editing.clone();
        let borrower = borrower.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_phone = edit_phone.clone();
        
        move || {
            // Reset edit fields to current borrower data
            if let Some(borrower_data) = borrower() {
                edit_name.set(borrower_data.name.clone());
                edit_status.set(borrower_data.status.unwrap_or(Status::Active));
                edit_email.set(borrower_data.email.unwrap_or_default());
                edit_phone.set(borrower_data.phone_number.unwrap_or_default());
            }
            is_editing.set(false);
        }
    };

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
                        let status_text = if let Some(borrower_data) = borrower() {
                            if let Some(status) = borrower_data.status {
                                status.to_string()
                            } else {
                                "N/A".to_string()
                            }
                        } else {
                            "Loading...".to_string()
                        };
                        let content: Element = rsx! {
                            // Error message
                            if let Some(error) = error_message() {
                                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                                    "{error}"
                                }
                            }

                            // Client Info Card

                            // Edit mode
                            // Display mode

            
            

                            div { class: "bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Client Information" }
                                div { class: "mb-4 flex justify-end",
                                    if *is_editing.read() {
                                        div { class: "space-x-2",
                                            button {
                                                class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                                                onclick: move |_| save_changes(),
                                                "Save"
                                            }
                                            button {
                                                class: "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                                                onclick: move |_| cancel_edit(),
                                                "Cancel"
                                            }
                                        }
                                    } else {
                                        button {
                                            class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                                            onclick: move |_| is_editing.set(true),
                                            "Edit"
                                        }
                                    }
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    if *is_editing.read() {
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Full Name" }
                                            input {
                                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                                value: "{edit_name()}",
                                                oninput: move |evt| edit_name.set(evt.value()),
                                            }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Status" }
                                            select {
                                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                                value: "{edit_status().to_string()}",
                                                onchange: move |evt| {
                                                    let value = evt.value();
                                                    let status = match value.as_str() {
                                                        "Inactive" => Status::Inactive,
                                                        "Pending" => Status::Pending,
                                                        "Approved" => Status::Approved,
                                                        "Rejected" => Status::Rejected,
                                                        "Closed" => Status::Closed,
                                                        _ => Status::Active,
                                                    };
                                                    edit_status.set(status);
                                                },
                                                option { value: "Active", "Active" }
                                                option { value: "Inactive", "Inactive" }
                                                option { value: "Pending", "Pending" }
                                                option { value: "Approved", "Approved" }
                                                option { value: "Rejected", "Rejected" }
                                                option { value: "Closed", "Closed" }
                                            }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Email" }
                                            input {
                                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                                value: "{edit_email()}",
                                                oninput: move |evt| edit_email.set(evt.value()),
                                            }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Phone Number" }
                                            input {
                                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                                value: "{edit_phone()}",
                                                oninput: move |evt| edit_phone.set(evt.value()),
                                            }
                                        }
                                    } else {
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Full Name" }
                                            p { class: "mt-1 text-sm text-gray-900",
                                                if let Some(borrower_data) = borrower() {
                                                    "{borrower_data.name}"
                                                } else {
                                                    "Loading..."
                                                }
                                            }
                                        }
            
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Status" }
                                            p { class: "mt-1 text-sm text-gray-900", "{status_text}" }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Email" }
                                            p { class: "mt-1 text-sm text-gray-900",
                                                if let Some(borrower_data) = borrower() {
                                                    "{borrower_data.email.as_deref().unwrap_or(\"N/A\")}"
                                                } else {
                                                    "Loading..."
                                                }
                                            }
                                        }
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700", "Phone Number" }
                                            p { class: "mt-1 text-sm text-gray-900",
                                                if let Some(borrower_data) = borrower() {
                                                    "{borrower_data.phone_number.as_deref().unwrap_or(\"N/A\")}"
                                                } else {
                                                    "Loading..."
                                                }
                                            }
                                        }
                                    }
                                }
                            }
            
                            div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
                                h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Income Information" }
                                W2Jobs { borrower_id: id }
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
