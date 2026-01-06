use dioxus::prelude::*;
use chrono::Utc;
use shared::models::Borrower;

#[component]
pub fn AddClientModal(on_client_added: EventHandler<()>) -> Element {
    let client_resource = use_context::<dioxus::prelude::Resource<client::Client>>();
    let mut is_open = use_signal(|| false);
    let mut first_name = use_signal(|| String::new());
    let mut last_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut phone = use_signal(|| String::new());

    let open_modal = move |_| is_open.set(true);
    let close_modal = move |_| {
        is_open.set(false);
        // Reset form
        first_name.set(String::new());
        last_name.set(String::new());
        email.set(String::new());
        phone.set(String::new());
    };

    let submit = move |evt: FormEvent| {
        evt.prevent_default();
        let first = first_name().clone();
        let last = last_name().clone();
        let full_name = format!("{} {}", first, last);
        let email_val = email().clone();
        let phone_val = phone().clone();

        let borrower = Borrower {
            id: 0, // Will be set by DB
            name: full_name,
            employer_name: Some(email_val),
            income_type: Some("employed".to_string()),
            loan_number: Some(phone_val),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let client_res = client_resource.clone();
        let on_added = on_client_added.clone();
        spawn(async move {
            if let Some(c) = client_res.read().as_ref() {
                let result: Result<(), Box<dyn std::error::Error>> = c.clone().save_borrower(borrower).await;
                match result {
                    Ok(_) => {
                        on_added.call(());
                        is_open.set(false);
                        // Reset form
                        first_name.set(String::new());
                        last_name.set(String::new());
                        email.set(String::new());
                        phone.set(String::new());
                    }
                    Err(e) => {
                        // Handle error, maybe show a message
                        eprintln!("Failed to save borrower: {:?}", e);
                    }
                }
            }
        });
    };

    rsx! {
        // Button to open modal
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            onclick: open_modal,
            "Add Client"
        }

        // Modal overlay
        if is_open() {
            div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                div { class: "bg-white p-6 rounded-lg shadow-lg max-w-md w-full mx-4",
                    h2 { class: "text-xl font-bold mb-4", "Add New Client" }
                    form { onsubmit: submit,
                        div { class: "mb-4",
                            label { class: "block text-gray-700 text-sm font-bold mb-2",
                                "First Name:"
                            }
                            input {
                                class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "text",
                                value: first_name(),
                                oninput: move |e| first_name.set(e.value().clone()),
                                required: true,
                            }
                        }
                        div { class: "mb-4",
                            label { class: "block text-gray-700 text-sm font-bold mb-2",
                                "Last Name:"
                            }
                            input {
                                class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "text",
                                value: last_name(),
                                oninput: move |e| last_name.set(e.value().clone()),
                                required: true,
                            }
                        }
                        div { class: "mb-4",
                            label { class: "block text-gray-700 text-sm font-bold mb-2",
                                "Email:"
                            }
                            input {
                                class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "email",
                                value: email(),
                                oninput: move |e| email.set(e.value().clone()),
                                required: true,
                            }
                        }
                        div { class: "mb-6",
                            label { class: "block text-gray-700 text-sm font-bold mb-2",
                                "Phone Number:"
                            }
                            input {
                                class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                r#type: "tel",
                                value: phone(),
                                oninput: move |e| phone.set(e.value().clone()),
                                required: true,
                            }
                        }
                        div { class: "flex items-center justify-between",
                            button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                                r#type: "submit",
                                "Add Client"
                            }
                            button {
                                class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline ml-4",
                                onclick: close_modal,
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}
