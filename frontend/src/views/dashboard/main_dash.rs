use dioxus::prelude::*;
use crate::components::Table;

#[derive(Clone, PartialEq)]
pub struct Client {
    pub id: u32,
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
    // Sample data - in real app, this would come from state or API
    let clients = use_signal(|| vec![
        Client { id: 1, name: "John Doe".to_string(), email: "john@example.com".to_string(), income: 75000.0, status: "Active".to_string() },
        Client { id: 2, name: "Jane Smith".to_string(), email: "jane@example.com".to_string(), income: 85000.0, status: "Active".to_string() },
        Client { id: 3, name: "Bob Johnson".to_string(), email: "bob@example.com".to_string(), income: 65000.0, status: "Inactive".to_string() },
    ]);

    let headers = vec!["ID".to_string(), "Name".to_string(), "Email".to_string(), "Income".to_string(), "Status".to_string()];
    let rows: Vec<Vec<String>> = clients().iter().map(|client| vec![
        client.id.to_string(),
        client.name.clone(),
        client.email.clone(),
        format!("${:.2}", client.income),
        client.status.clone(),
    ]).collect();

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

                // Stats Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-8",
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Total Clients" }
                        p { class: "text-3xl font-bold text-blue-600 mt-2", "{total_clients}" }
                    }
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Active Clients" }
                        p { class: "text-3xl font-bold text-green-600 mt-2", "{active_clients}" }
                    }
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h3 { class: "text-lg font-semibold text-gray-700", "Average Income" }
                        p { class: "text-3xl font-bold text-purple-600 mt-2", "{average_income_str}" }
                    }
                }

                // Clients Table
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Clients" }
                    Table { headers, rows }
                }
            }
        }
    }
}
