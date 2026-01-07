use dioxus::prelude::*;
use crate::components::AnalyticsCard;

/// Analytics page component showcasing various analytics cards
#[component]
pub fn Analytics() -> Element {
    // Mock data - in a real app, this would come from your database/API
    let total_clients = 125;
    let active_clients = 98;
    let total_income = 2456789.50;
    let average_income = total_income / total_clients as f64;
    let total_loans = 87;
    let pending_applications = 12;
    let monthly_growth = 8.5;
    let conversion_rate = 67.3;

    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Analytics Dashboard" }
                    p { class: "text-gray-600 mt-2", "Key metrics and insights for your business" }
                }

                // Analytics Cards Grid
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mb-8",
                    AnalyticsCard {
                        title: "Total Clients".to_string(),
                        value: total_clients.to_string(),
                        subtitle: Some("All registered clients".to_string()),
                        icon: Some("👥".to_string()),
                        color: "blue".to_string(),
                        trend: Some("+12%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Active Clients".to_string(),
                        value: active_clients.to_string(),
                        subtitle: Some("Currently active".to_string()),
                        icon: Some("✅".to_string()),
                        color: "green".to_string(),
                        trend: Some("+5%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Total Income".to_string(),
                        value: format!("${:.0}", total_income),
                        subtitle: Some("Combined client income".to_string()),
                        icon: Some("💰".to_string()),
                        color: "purple".to_string(),
                        trend: Some("+15%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Average Income".to_string(),
                        value: format!("${:.0}", average_income),
                        subtitle: Some("Per client".to_string()),
                        icon: Some("📊".to_string()),
                        color: "indigo".to_string(),
                        trend: Some("+3%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Total Loans".to_string(),
                        value: total_loans.to_string(),
                        subtitle: Some("Approved loans".to_string()),
                        icon: Some("🏠".to_string()),
                        color: "blue".to_string(),
                        trend: Some("+8%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Pending Applications".to_string(),
                        value: pending_applications.to_string(),
                        subtitle: Some("Awaiting approval".to_string()),
                        icon: Some("⏳".to_string()),
                        color: "yellow".to_string(),
                        trend: Some("-2".to_string()),
                        trend_positive: false,
                    }

                    AnalyticsCard {
                        title: "Monthly Growth".to_string(),
                        value: format!("{}%", monthly_growth),
                        subtitle: Some("Client acquisition".to_string()),
                        icon: Some("📈".to_string()),
                        color: "green".to_string(),
                        trend: Some("+2.1%".to_string()),
                        trend_positive: true,
                    }

                    AnalyticsCard {
                        title: "Conversion Rate".to_string(),
                        value: format!("{}%", conversion_rate),
                        subtitle: Some("Applications to loans".to_string()),
                        icon: Some("🎯".to_string()),
                        color: "red".to_string(),
                        trend: Some("+1.5%".to_string()),
                        trend_positive: true,
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