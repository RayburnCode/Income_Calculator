use dioxus::prelude::*;
use crate::components::Input;
use shared::models::Borrower;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Information() -> Element {
    // Get the database client
    let client_resource = use_resource(|| async {
        client::Client::new().await.ok()
    });

    // State for borrower information
    let borrower = use_signal(|| Borrower::default());
    let is_loading = use_signal(|| true);
    let is_saving = use_signal(|| false);
    let save_message = use_signal(|| None::<String>);

    // Load borrower data on mount
    use_effect(move || {
        let mut borrower = borrower;
        let mut is_loading = is_loading;
        spawn(async move {
            if let Some(Some(_db_client)) = client_resource.read().as_ref() {
                // For now, create a default borrower or load existing one
                // In a real app, you'd get the borrower ID from context/route
                let default_borrower = Borrower::default();
                borrower.set(default_borrower);
                is_loading.set(false);
            }
        });
    });

    // Save borrower data
    let save_borrower = move |_| {
        let borrower = borrower;
        let mut is_saving = is_saving;
        let mut save_message = save_message;
        spawn(async move {
            if let Some(Some(db_client)) = client_resource.read().as_ref() {
                let borrower_data = borrower();
                is_saving.set(true);
                match db_client.save_borrower(borrower_data).await {
                    Ok(_) => {
                        save_message.set(Some("Borrower information saved successfully!".to_string()));
                    }
                    Err(e) => {
                        save_message.set(Some(format!("Error saving borrower: {}", e)));
                    }
                }
                is_saving.set(false);
            }
        });
    };

    rsx! {
        if is_loading() {
            div { class: "text-center py-8", "Loading borrower information..." }
        } else {
            div { class: "space-y-4",
                Input {
                    placeholder: "Borrower's Name",
                    label: "Borrower's Name",
                    value: borrower().name.clone(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.name = evt.value();
                        borrower.set(b);
                    },
                }
                Input {
                    placeholder: "Employer Name",
                    label: "Employer Name",
                    value: borrower().employer_name.clone().unwrap_or_default(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.employer_name = if evt.value().is_empty() { None } else { Some(evt.value()) };
                        borrower.set(b);
                    },
                }
                Input {
                    placeholder: "Income Type",
                    label: "Income Type",
                    value: borrower().income_type.clone().unwrap_or_default(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.income_type = if evt.value().is_empty() { None } else { Some(evt.value()) };
                        borrower.set(b);
                    },
                }
                Input {
                    placeholder: "Loan Number",
                    label: "Loan Number",
                    value: borrower().loan_number.clone().unwrap_or_default(),
                    oninput: move |evt: FormEvent| {
                        let mut borrower = borrower;
                        let mut b = borrower();
                        b.loan_number = if evt.value().is_empty() { None } else { Some(evt.value()) };
                        borrower.set(b);
                    },
                }
                div { class: "flex items-center space-x-4",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50",
                        disabled: is_saving(),
                        onclick: save_borrower,
                        if is_saving() {
                            "Saving..."
                        } else {
                            "Save Borrower Info"
                        }
                    }
                    if let Some(message) = save_message() {
                        span { class: "text-green-600", "{message}" }
                    }
                }
            }
        }
    }
}
 