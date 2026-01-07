use dioxus::prelude::*;
use crate::views::dashboard::add_client_modal::AddClientModal;

#[derive(Clone, PartialEq)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub income: f64,
    pub status: String,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name) // For simplicity, but table will display fields separately
    }
}

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn MainDashboard() -> Element {
    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        client::Client::new().await
    });

    // State for clients - will be loaded from database
    let clients = use_signal(|| Vec::<Client>::new());
    let error_message = use_signal(|| None::<String>);

    // Load clients when the resource is ready
    use_effect(move || {
        let resource_value = client_resource.read().clone();
        let mut clients = clients.clone();
        let mut error_message = error_message.clone();
        
        spawn(async move {
            match resource_value.as_ref() {
                Some(Ok(db_client)) => {
                    // Load borrowers from database
                    match db_client.get_all_borrowers().await {
                        Ok(borrowers) => {
                            // Convert borrowers to clients for display
                            let client_list: Vec<Client> = borrowers.into_iter().enumerate().map(|(i, b)| {
                                Client {
                                    id: (i as i32 + 1),
                                    name: b.name,
                                    email: b.employer_name.unwrap_or_else(|| "N/A".to_string()),
                                    income: 0.0, // We don't have income in borrower table yet
                                    status: "Active".to_string(),
                                }
                            }).collect();
                            clients.set(client_list);
                            error_message.set(None);
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Error loading borrowers: {}", e)));
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

    // Function to reload clients (for use by add client modal)
    let reload_clients = {
        let client_resource = client_resource.clone();
        let mut clients = clients.clone();
        let mut error_message = error_message.clone();
        move || {
            spawn(async move {
                if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                    match db_client.get_all_borrowers().await {
                        Ok(borrowers) => {
                            let client_list: Vec<Client> = borrowers.into_iter().enumerate().map(|(i, b)| {
                                Client {
                                    id: (i as i32 + 1),
                                    name: b.name,
                                    email: b.employer_name.unwrap_or_else(|| "N/A".to_string()),
                                    income: 0.0,
                                    status: "Active".to_string(),
                                }
                            }).collect();
                            clients.set(client_list);
                            error_message.set(None);
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Error loading borrowers: {}", e)));
                        }
                    }
                }
            });
        }
    };

    let headers = vec!["ID".to_string(), "Name".to_string(), "Email".to_string(), "Income".to_string(), "Status".to_string()];

    let total_clients = clients().len();
    let active_clients = clients().iter().filter(|c| c.status == "Active").count();
    let average_income = if total_clients > 0 {
        clients().iter().map(|c| c.income).sum::<f64>() / total_clients as f64
    } else {
        0.0
    };
    let average_income_str = format!("${:.2}", average_income);

    rsx! {
        div { class: "min-h-screen bg-gray-100 p-3 sm:p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-4 sm:mb-8",
                    h1 { class: "text-2xl sm:text-3xl font-bold text-gray-900", "Dashboard" }
                    p { class: "text-sm sm:text-base text-gray-600 mt-2", "Welcome to your Income Calculator Dashboard" }
                }

                // Error message
                if let Some(error) = error_message() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                        "{error}"
                    }
                }

                // Stats Cards
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 sm:gap-6 mb-4 sm:mb-8",
                    div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md",
                        h3 { class: "text-sm sm:text-lg font-semibold text-gray-700", "Total Clients" }
                        p { class: "text-2xl sm:text-3xl font-bold text-blue-600 mt-2", "{total_clients}" }
                    }
                    div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md",
                        h3 { class: "text-sm sm:text-lg font-semibold text-gray-700", "Active Clients" }
                        p { class: "text-2xl sm:text-3xl font-bold text-green-600 mt-2", "{active_clients}" }
                    }
                    div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md",
                        h3 { class: "text-sm sm:text-lg font-semibold text-gray-700", "Average Income" }
                        p { class: "text-2xl sm:text-3xl font-bold text-purple-600 mt-2", "{average_income_str}" }
                    }
                }

                // Clients Table
                div { class: "bg-white p-3 sm:p-6 rounded-lg shadow-md",
                    div { class: "flex flex-col sm:flex-row justify-between items-start sm:items-center mb-4 gap-3",
                        h2 { class: "text-lg sm:text-xl font-semibold text-gray-800", "Clients" }
                        if let Some(Ok(_)) = client_resource.read().as_ref() {
                            AddClientModal { on_client_added: move |_| reload_clients() }
                        } else {
                            button {
                                class: "bg-gray-500 text-white font-bold py-2 px-4 rounded cursor-not-allowed text-sm sm:text-base w-full sm:w-auto",
                                disabled: true,
                                "Database Not Connected"
                            }
                        }
                    }
                    div { class: "overflow-x-auto bg-white shadow-md rounded-lg -mx-3 sm:mx-0",
                        table { class: "min-w-full table-auto",
                            thead { class: "bg-gray-50",
                                tr {
                                    for header in &headers {
                                        th { class: "px-3 sm:px-6 py-2 sm:py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "{header}"
                                        }
                                    }
                                    th { class: "px-3 sm:px-6 py-2 sm:py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Actions"
                                    }
                                }
                            }
                            tbody { class: "bg-white divide-y divide-gray-200",
                                for client in clients() {
                                    tr { class: "hover:bg-gray-50",
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            "{client.id}"
                                        }
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            "{client.name}"
                                        }
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            "{client.email}"
                                        }
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            "$"
                                            {format!("{:.2}", client.income)}
                                        }
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            "{client.status}"
                                        }
                                        td { class: "px-3 sm:px-6 py-3 sm:py-4 whitespace-nowrap text-xs sm:text-sm text-gray-900",
                                            Link {
                                                to: format!("/dashboard/client/{}", client.id),
                                                class: "text-blue-600 hover:text-blue-800 font-medium",
                                                "View"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
