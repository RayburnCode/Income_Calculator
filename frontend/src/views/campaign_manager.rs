use dioxus::prelude::*;
use shared::models::{Campaign, CampaignStatus};

#[component]
pub fn CampaignManager() -> Element {
    let campaigns = use_resource(|| async {
        let repo = crate::get_repository();
        repo.get_all_campaigns().await.unwrap_or_default()
    });

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "flex justify-between items-center",
                h1 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Campaign Manager" }
                button { class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                    "Create Campaign"
                }
            }

            // Campaigns List
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md",
                match campaigns() {
                    Some(campaign_list) => {
                        if campaign_list.is_empty() {
                            rsx! {
                                div { class: "p-8 text-center text-gray-500 dark:text-gray-400",
                                    "No campaigns found. Create your first campaign to get started."
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "divide-y divide-gray-200 dark:divide-gray-700",
                                    for campaign in campaign_list {
                                        {
                                            let status_class = match campaign.status {
                                                CampaignStatus::Draft => {
                                                    "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200"
                                                }
                                                CampaignStatus::Scheduled => {
                                                    "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200"
                                                }
                                                CampaignStatus::Running => {
                                                    "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                                                }
                                                CampaignStatus::Paused => {
                                                    "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200"
                                                }
                                                CampaignStatus::Completed => {
                                                    "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200"
                                                }
                                                CampaignStatus::Cancelled => {
                                                    "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
                                                }
                                            };
                                            rsx! {
                                                div { class: "p-4 hover:bg-gray-50 dark:hover:bg-gray-700",
                                                    div { class: "flex justify-between items-start",
                                                        div { class: "flex-1",
                                                            h3 { class: "text-lg font-medium text-gray-900 dark:text-white", "{campaign.name}" }
                                                            p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                                                "{campaign.description.as_deref().unwrap_or(\"\")}"
                                                            }
                                                            div { class: "flex items-center space-x-4 mt-2",
                                                                span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                                                    "{campaign.campaign_type:?}"
                                                                }
                                                                span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {status_class}",
                                                                    "{campaign.status:?}"
                                                                }
                                                            }
                                                            if campaign.sent_count > 0 {
                                                                div { class: "mt-3 grid grid-cols-4 gap-4 text-sm",
                                                                    div { class: "text-center",
                                                                        div { class: "font-semibold text-gray-900 dark:text-white",
                                                                            "{campaign.sent_count}"
                                                                        }
                                                                        div { class: "text-gray-600 dark:text-gray-400", "Sent" }
                                                                    }
                                                                    div { class: "text-center",
                                                                        div { class: "font-semibold text-gray-900 dark:text-white",
                                                                            "{campaign.opened_count}"
                                                                        }
                                                                        div { class: "text-gray-600 dark:text-gray-400", "Opens" }
                                                                    }
                                                                    div { class: "text-center",
                                                                        div { class: "font-semibold text-gray-900 dark:text-white",
                                                                            "{campaign.clicked_count}"
                                                                        }
                                                                        div { class: "text-gray-600 dark:text-gray-400", "Clicks" }
                                                                    }
                                                                    div { class: "text-center",
                                                                        div { class: "font-semibold text-gray-900 dark:text-white",
                                                                            "{campaign.converted_count}"
                                                                        }
                                                                        div { class: "text-gray-600 dark:text-gray-400", "Conversions" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        div { class: "flex space-x-2",
                                                            button { class: "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 text-sm font-medium",
                                                                "View"
                                                            }
                                                            button { class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-200 text-sm font-medium",
                                                                "Delete"
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
                    None => rsx! {
                        div { class: "p-8 text-center",
                            div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4" }
                            p { class: "text-gray-600 dark:text-gray-400", "Loading campaigns..." }
                        }
                    },
                }
            }
        }
    }
}
