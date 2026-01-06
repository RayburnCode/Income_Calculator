use dioxus::prelude::*;
use dioxus_router::Link;
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
        client::Client::new().await.ok()
    });

    // State for clients - will be loaded from database
    let clients = use_signal(|| Vec::<Client>::new());
    let is_loading = use_signal(|| true);
    let error_message = use_signal(|| None::<String>);

    // Load clients on mount
    let reload_clients = {
        let client_resource = client_resource.clone();
        let mut clients = clients.clone();
        let mut error_message = error_message.clone();
        let mut is_loading = is_loading.clone();
        move || {
            spawn(async move {
                if let Some(Some(db_client)) = client_resource.read().as_ref() {
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
                            is_loading.set(false);
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Error loading borrowers: {}", e)));
                            is_loading.set(false);
                        }
                    }
                } else {
                    // Failed to connect to database
                    error_message.set(Some("Failed to connect to database. Please check the database setup.".to_string()));
                    is_loading.set(false);
                }
            });
        }
    };

    use_effect(move || {
        reload_clients();
    });

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
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Dashboard" }
                    p { class: "text-gray-600 mt-2", "Welcome to your Income Calculator Dashboard" }
                }

                // Error message
                if let Some(error) = error_message() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                        "{error}"
                    }
                }

                // Stats Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-8",
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Total Clients" }
                        if is_loading() {
                            p { class: "text-3xl font-bold text-blue-600 mt-2", "..." }
                        } else {
                            p { class: "text-3xl font-bold text-blue-600 mt-2", "{total_clients}" }
                        }
                    }
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Active Clients" }
                        if is_loading() {
                            p { class: "text-3xl font-bold text-green-600 mt-2", "..." }
                        } else {
                            p { class: "text-3xl font-bold text-green-600 mt-2", "{active_clients}" }
                        }
                    }
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Average Income" }
                        if is_loading() {
                            p { class: "text-3xl font-bold text-purple-600 mt-2",
                                "..."
                            }
                        } else {
                            p { class: "text-3xl font-bold text-purple-600 mt-2",
                                "{average_income_str}"
                            }
                        }
                    }
                }

                // Clients Table
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    div { class: "flex justify-between items-center mb-4",
                        h2 { class: "text-xl font-semibold text-gray-800", "Clients" }
                        if client_resource.read().as_ref().is_some() {
                            AddClientModal { on_client_added: move |_| reload_clients() }
                        } else {
                            button { class: "bg-gray-500 text-white font-bold py-2 px-4 rounded",
                                "Loading..."
                            }
                        }
                    }
                    if is_loading() {
                        div { class: "text-center py-8", "Loading clients..." }
                    } else {
                        div { class: "overflow-x-auto bg-white shadow-md rounded-lg",
                            table { class: "min-w-full table-auto",
                                thead { class: "bg-gray-50",
                                    tr {
                                        for header in &headers {
                                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                "{header}"
                                            }
                                        }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Actions"
                                        }
                                    }
                                }
                                tbody { class: "bg-white divide-y divide-gray-200",
                                    for client in clients() {
                                        tr { class: "hover:bg-gray-50",
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                "{client.id}"
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                "{client.name}"
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                "{client.email}"
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                "$"
                                                {format!("{:.2}", client.income)}
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                "{client.status}"
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                Link { to: format!("/dashboard/client/{}", client.id),
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
}
