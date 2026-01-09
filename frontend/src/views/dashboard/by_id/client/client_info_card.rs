use dioxus::prelude::*;
use shared::models::Status;

#[component]
pub fn ClientInfoCard(
    borrower: Signal<Option<shared::models::Borrower>>,
    error_message: Signal<Option<String>>,
    is_editing: Signal<bool>,
    edit_name: Signal<String>,
    edit_status: Signal<Status>,
    edit_email: Signal<String>,
    edit_phone: Signal<String>,
    edit_date_of_birth: Signal<String>,
    edit_social_security_number: Signal<String>,
    edit_address: Signal<String>,
    edit_city: Signal<String>,
    edit_state: Signal<String>,
    edit_zip_code: Signal<String>,
    edit_mailing_address_different: Signal<bool>,
    save_changes: Callback<()>,
    cancel_edit: Callback<()>,
    format_phone_number: fn(&str) -> String,
) -> Element {
    let status_text = if let Some(borrower_data) = borrower() {
        if let Some(status) = borrower_data.status {
            status.to_string()
        } else {
            "N/A".to_string()
        }
    } else {
        "Loading...".to_string()
    };

    rsx! {
        // Error message
        if let Some(error) = error_message() {
            div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                "{error}"
            }
        }

        div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold text-gray-800 dark:text-gray-200 mb-4",
                "Client Information"
            }
            div { class: "mb-4 flex justify-end",
                if *is_editing.read() {
                    div { class: "space-x-2",
                        button {
                            class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| save_changes.call(()),
                            "Save"
                        }
                        button {
                            class: "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| cancel_edit.call(()),
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
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Full Name"
                        }
                        input {
                            class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            value: "{edit_name()}",
                            oninput: move |evt| edit_name.set(evt.value()),
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Status"
                        }
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
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Email"
                        }
                        input {
                            class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            value: "{edit_email()}",
                            oninput: move |evt| edit_email.set(evt.value()),
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Phone Number"
                        }
                        input {
                            class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            value: "{edit_phone()}",
                            oninput: move |evt| edit_phone.set(evt.value()),
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Date of Birth"
                        }
                        input {
                            r#type: "date",
                            class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            value: "{edit_date_of_birth()}",
                            oninput: move |evt| edit_date_of_birth.set(evt.value()),
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            "Social Security Number"
                        }
                        input {
                            class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                            value: "{edit_social_security_number()}",
                            oninput: move |evt| edit_social_security_number.set(evt.value()),
                        }
                    }
                }
            }
            // Address section
            div { class: "mt-6",
                h3 { class: "text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4",
                    "Address Information"
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    if *is_editing.read() {
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Address"
                            }
                            input {
                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                value: "{edit_address()}",
                                oninput: move |evt| edit_address.set(evt.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "City"
                            }
                            input {
                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                value: "{edit_city()}",
                                oninput: move |evt| edit_city.set(evt.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "State"
                            }
                            input {
                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                value: "{edit_state()}",
                                oninput: move |evt| edit_state.set(evt.value()),
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Zip Code"
                            }
                            input {
                                class: "mt-1 text-black block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                value: "{edit_zip_code()}",
                                oninput: move |evt| edit_zip_code.set(evt.value()),
                            }
                        }
                        div { class: "md:col-span-2",
                            label { class: "flex items-center",
                                input {
                                    r#type: "checkbox",
                                    class: "mr-2",
                                    checked: "{edit_mailing_address_different()}",
                                    onchange: move |evt| edit_mailing_address_different.set(evt.checked()),
                                }
                                span { class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "Different mailing address"
                                }
                            }
                        }
                    } else {
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Address"
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-gray-100",
                                if let Some(borrower_data) = borrower() {
                                    "{borrower_data.address.as_deref().unwrap_or(\"N/A\")}"
                                } else {
                                    "Loading..."
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "City"
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-gray-100",
                                if let Some(borrower_data) = borrower() {
                                    "{borrower_data.city.as_deref().unwrap_or(\"N/A\")}"
                                } else {
                                    "Loading..."
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "State"
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-gray-100",
                                if let Some(borrower_data) = borrower() {
                                    "{borrower_data.state.as_deref().unwrap_or(\"N/A\")}"
                                } else {
                                    "Loading..."
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Zip Code"
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-gray-100",
                                if let Some(borrower_data) = borrower() {
                                    "{borrower_data.zip_code.as_deref().unwrap_or(\"N/A\")}"
                                } else {
                                    "Loading..."
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Mailing Address Different"
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-gray-100",
                                if let Some(borrower_data) = borrower() {
                                    if borrower_data.mailing_address_different.unwrap_or(false) {
                                        "Yes"
                                    } else {
                                        "No"
                                    }
                                } else {
                                    "Loading..."
                                }
                            }
                        }
                    }
                }
            
            }
        }
    }
}