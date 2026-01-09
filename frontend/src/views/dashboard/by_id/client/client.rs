use dioxus::prelude::*;
use crate::components::tab::{Tab, TabItem};
use crate::views::dashboard::by_id::income_worksheet::Worksheet;
use crate::views::dashboard::by_id::options_template::OptionsTemplate;
use crate::views::dashboard::by_id::outreach::{OutreachTemplates, Timeline};
use crate::views::dashboard::by_id::{ClientNotes,ClientDocuments, ClientConditions};
use chrono::Utc;
use shared::models::Status;
use crate::views::dashboard::by_id::client::{ClientOverview};

#[component] 
pub fn ClientDetails(id: i32) -> Element {
    let mut active_tab = use_signal(|| 0);

    // Function to format phone number as (111)111-1111
    let format_phone_number = |input: &str| -> String {
        let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
        let len = digits.len();

        match len {
            0 => String::new(),
            1..=3 => format!("({}", digits),
            4..=6 => format!("({}){}", &digits[0..3], &digits[3..]),
            _ => format!("({}){}-{}", &digits[0..3], &digits[3..6], &digits[6..len.min(10)]),
        }
    };

    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        repository::Repository::new().await
    });

    // State for borrower data
    let borrower = use_signal(|| None::<shared::models::Borrower>);
    let error_message = use_signal(|| None::<String>);
    let  is_editing = use_signal(|| false);
    let  edit_name = use_signal(|| String::new());
    let  edit_status = use_signal(|| Status::Active);
    let  edit_email = use_signal(|| String::new());
    let  edit_phone = use_signal(|| String::new());
    let mut edit_date_of_birth = use_signal(|| String::new());
    let mut edit_social_security_number = use_signal(|| String::new());
    let mut edit_address = use_signal(|| String::new());
    let mut edit_city = use_signal(|| String::new());
    let mut edit_state = use_signal(|| String::new());
    let mut edit_zip_code = use_signal(|| String::new());
    let mut edit_mailing_address_different = use_signal(|| false);

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
                            edit_date_of_birth.set(borrower_data.date_of_birth.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default());
                            edit_social_security_number.set(borrower_data.social_security_number.unwrap_or_default());
                            edit_address.set(borrower_data.address.unwrap_or_default());
                            edit_city.set(borrower_data.city.unwrap_or_default());
                            edit_state.set(borrower_data.state.unwrap_or_default());
                            edit_zip_code.set(borrower_data.zip_code.unwrap_or_default());
                            edit_mailing_address_different.set(borrower_data.mailing_address_different.unwrap_or(false));
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
                        borrower_data.date_of_birth = if edit_date_of_birth().is_empty() { None } else { chrono::NaiveDate::parse_from_str(&edit_date_of_birth(), "%Y-%m-%d").ok() };
                        borrower_data.social_security_number = if edit_social_security_number().is_empty() { None } else { Some(edit_social_security_number()) };
                        borrower_data.address = if edit_address().is_empty() { None } else { Some(edit_address()) };
                        borrower_data.city = if edit_city().is_empty() { None } else { Some(edit_city()) };
                        borrower_data.state = if edit_state().is_empty() { None } else { Some(edit_state()) };
                        borrower_data.zip_code = if edit_zip_code().is_empty() { None } else { Some(edit_zip_code()) };
                        borrower_data.mailing_address_different = Some(edit_mailing_address_different());
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
                edit_date_of_birth.set(borrower_data.date_of_birth.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default());
                edit_social_security_number.set(borrower_data.social_security_number.unwrap_or_default());
                edit_address.set(borrower_data.address.unwrap_or_default());
                edit_city.set(borrower_data.city.unwrap_or_default());
                edit_state.set(borrower_data.state.unwrap_or_default());
                edit_zip_code.set(borrower_data.zip_code.unwrap_or_default());
                edit_mailing_address_different.set(borrower_data.mailing_address_different.unwrap_or(false));
            }
            is_editing.set(false);
        }
    };

    // Wrap closures in Callbacks
    let save_changes_cb = Callback::new(move |_| save_changes());
    let cancel_edit_cb = Callback::new(move |_| cancel_edit());

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
                    TabItem {
            label: "Notes".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },            TabItem {
            label: "Documents".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },            TabItem {
            label: "Conditions".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
            TabItem {
            label: "Outreach".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
                    TabItem {
            label: "Timeline".to_string(),
            href: None,
            disabled: false,
            icon: None,
        },
    ];

    rsx! {
        div { class: "min-h-screen bg-theme-bg-primary dark:bg-theme-bg-secondary",
            // Banner background
            div { class: "relative h-32 bg-gradient-to-r from-primary-medium to-accent pt-20",
                // Optional overlay for better text readability if needed
                div { class: "absolute inset-0 bg-theme-bg-primary/20 dark:bg-theme-bg-secondary/30" }

                // Client name, status, and tabs overlay
                div { class: "absolute bottom-4 left-6 right-6 flex flex-col items-start gap-4",
                    // Name and status on the top
                    div { class: "pb-4",
                        h1 { class: "text-2xl font-bold text-theme-text-primary dark:text-theme-text-secondary drop-shadow-lg",
                            {borrower().map(|b| b.name.clone()).unwrap_or_else(|| "Loading...".to_string())}
                        }
                        {
                            borrower()
                                .and_then(|b| b.status)
                                .map(|status| {
                                    let class_name = match status {
                                        Status::Active => "bg-green-500 dark:bg-green-600 text-white",
                                        Status::Inactive => "bg-gray-500 dark:bg-gray-600 text-white",
                                        Status::Pending => {
                                            "bg-yellow-500 dark:bg-yellow-600 text-black dark:text-gray-900"
                                        }
                                        Status::Approved => "bg-blue-500 dark:bg-blue-600 text-white",
                                        Status::Rejected => "bg-red-500 dark:bg-red-600 text-white",
                                        Status::Closed => "bg-purple-500 dark:bg-purple-600 text-white",
                                    };
                                    rsx! {
                                        span { class: "px-3 py-1 rounded-full text-sm font-medium {class_name} w-fit", "{status}" }
                                    }
                                })
                        }
                    }

                    // Tabs below the name
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
            }

            div { class: "dark:bg-theme-bg-secondary relative p-6",
                // Content based on active tab
                match *active_tab.read() {
                    0 => {
                        rsx! {
                            ClientOverview {
                                id,
                                borrower,
                                error_message,
                                is_editing,
                                edit_name,
                                edit_status,
                                edit_email,
                                edit_phone,
                                edit_date_of_birth,
                                edit_social_security_number,
                                edit_address,
                                edit_city,
                                edit_state,
                                edit_zip_code,
                                edit_mailing_address_different,
                                save_changes: save_changes_cb,
                                cancel_edit: cancel_edit_cb,
                                format_phone_number,
                            }
                        }
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
                    3 => {
                        let content: Element = rsx! {
                            ClientNotes { id }
                        };
                        content
                    }
                    4 => {
                        let content: Element = rsx! {
                            ClientDocuments { id }
                        };
                        content
                    }
                    5 => {
                        let content: Element = rsx! {
                            ClientConditions { id }
                        };
                        content
                    }
                    6 => {
                        let content: Element = rsx! {
                            OutreachTemplates { id }
                        };
                        content
                    }
                    7 => {
                        let content: Element = rsx! {
                            Timeline { id }
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
