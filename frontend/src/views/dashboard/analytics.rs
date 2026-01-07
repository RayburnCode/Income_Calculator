use dioxus::prelude::*;
use crate::components::AnalyticsCard;

/// Analytics data structure
#[derive(Clone, Debug)]
struct AnalyticsData {
    total_clients: usize,
    active_clients: usize,
    total_income: f64,
    average_income: f64,
    total_loans: usize,
    pending_applications: usize,
    monthly_growth: f64,
    conversion_rate: f64,
}

/// Analytics page component showcasing various analytics cards
#[component]
pub fn Analytics() -> Element {
    let client = use_context::<client::Client>();
    let mut analytics_data = use_signal(|| None::<AnalyticsData>);
    let mut is_loading = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);

    // Load analytics data on component mount
    use_effect(move || {
        let client_clone = client.clone();
        spawn(async move {
            match load_analytics_data(&client_clone).await {
                Ok(data) => {
                    analytics_data.set(Some(data));
                    is_loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to load analytics: {:?}", e)));
                    is_loading.set(false);
                }
            }
        });
    });

    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Analytics Dashboard" }
                    p { class: "text-gray-600 mt-2", "Key metrics and insights for your business" }
                }

                // Loading state
                if is_loading() {
                    div { class: "flex justify-center items-center h-64",
                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500" }
                        span { class: "ml-3 text-gray-600", "Loading analytics..." }
                    }
                } else if let Some(error) = error_message() {
                    div { class: "bg-red-50 border border-red-200 rounded-lg p-4 mb-8",
                        div { class: "flex",
                            div { class: "flex-shrink-0", "⚠️" }
                            div { class: "ml-3",
                                h3 { class: "text-sm font-medium text-red-800",
                                    "Error Loading Analytics"
                                }
                                p { class: "mt-2 text-sm text-red-700", "{error}" }
                            }
                        }
                    }
                } else if let Some(data) = analytics_data() {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mb-8",
                        AnalyticsCard {
                            title: "Total Clients".to_string(),
                            value: data.total_clients.to_string(),
                            subtitle: Some("All registered clients".to_string()),
                            icon: Some("👥".to_string()),
                            color: "blue".to_string(),
                            trend: Some("+12%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Active Clients".to_string(),
                            value: data.active_clients.to_string(),
                            subtitle: Some("Currently active".to_string()),
                            icon: Some("✅".to_string()),
                            color: "green".to_string(),
                            trend: Some("+5%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Total Income".to_string(),
                            value: format!("${:.0}", data.total_income),
                            subtitle: Some("Combined client income".to_string()),
                            icon: Some("💰".to_string()),
                            color: "purple".to_string(),
                            trend: Some("+15%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Average Income".to_string(),
                            value: format!("${:.0}", data.average_income),
                            subtitle: Some("Per client".to_string()),
                            icon: Some("📊".to_string()),
                            color: "indigo".to_string(),
                            trend: Some("+3%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Total Loans".to_string(),
                            value: data.total_loans.to_string(),
                            subtitle: Some("Approved loans".to_string()),
                            icon: Some("🏠".to_string()),
                            color: "blue".to_string(),
                            trend: Some("+8%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Pending Applications".to_string(),
                            value: data.pending_applications.to_string(),
                            subtitle: Some("Awaiting approval".to_string()),
                            icon: Some("⏳".to_string()),
                            color: "yellow".to_string(),
                            trend: Some("-2".to_string()),
                            trend_positive: false,
                        }

                        AnalyticsCard {
                            title: "Monthly Growth".to_string(),
                            value: format!("{}%", data.monthly_growth),
                            subtitle: Some("Client acquisition".to_string()),
                            icon: Some("📈".to_string()),
                            color: "green".to_string(),
                            trend: Some("+2.1%".to_string()),
                            trend_positive: true,
                        }

                        AnalyticsCard {
                            title: "Conversion Rate".to_string(),
                            value: format!("{}%", data.conversion_rate),
                            subtitle: Some("Applications to loans".to_string()),
                            icon: Some("🎯".to_string()),
                            color: "red".to_string(),
                            trend: Some("+1.5%".to_string()),
                            trend_positive: true,
                        }
                    }
                }

                // Additional sections can be added here
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Detailed Analytics" }
                    p { class: "text-gray-600", "More detailed charts and graphs would go here." }
                }
            }
        }
    }
}

/// Load analytics data from the database
async fn load_analytics_data(client: &client::Client) -> Result<AnalyticsData, Box<dyn std::error::Error>> {
    // Get total clients count
    let total_clients = client.get_total_clients_count().await? as usize;

    // For now, assume all borrowers are active (we can enhance this later with status tracking)
    let active_clients = total_clients;

    // Get total income sum
    let total_income = client.get_total_income_sum().await?;

    let average_income = if total_clients > 0 {
        total_income / total_clients as f64
    } else {
        0.0
    };

    // Get total loans count
    let total_loans = client.get_total_loans_count().await? as usize;

    // For now, assume no pending applications (we can enhance this later)
    let pending_applications = 0;

    // Mock growth and conversion rates (we can calculate these from historical data later)
    let monthly_growth = 8.5;
    let conversion_rate = 67.3;

    Ok(AnalyticsData {
        total_clients,
        active_clients,
        total_income,
        average_income,
        total_loans,
        pending_applications,
        monthly_growth,
        conversion_rate,
    })
}