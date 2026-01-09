use dioxus::prelude::*;
use shared::models::{Campaign, CampaignStatus, CampaignType, OutreachTemplate};
use crate::components::input::Input;

#[component]
pub fn CampaignManager() -> Element {
    let mut campaigns = use_resource(|| async {
        let repo = crate::get_repository();
        repo.get_all_campaigns().await.unwrap_or_default()
    });

    let templates = use_resource(|| async {
        let repo = crate::get_repository();
        repo.get_all_outreach_templates().await.unwrap_or_default()
    });

    let mut selected_campaign = use_signal(|| None::<Campaign>);
    let mut show_create_form = use_signal(|| false);
    let mut new_campaign_name = use_signal(|| String::new());
    let mut new_campaign_description = use_signal(|| String::new());
    let mut new_campaign_type = use_signal(|| CampaignType::Email);
    let mut selected_template_id = use_signal(|| None::<i32>);
    let mut segment_criteria = use_signal(|| String::new());

    let create_campaign = move |_| {
        spawn(async move {
            if let (repo, Some(template_id)) = (crate::get_repository(), selected_template_id()) {
                let campaign = Campaign {
                    id: 0, // Will be set by database
                    name: new_campaign_name(),
                    description: if new_campaign_description().is_empty() { None } else { Some(new_campaign_description()) },
                    campaign_type: new_campaign_type(),
                    template_id,
                    segment_criteria: serde_json::json!({
                        "criteria": segment_criteria()
                    }),
                    status: CampaignStatus::Draft,
                    scheduled_date: None,
                    completed_date: None,
                    target_audience_count: None,
                    sent_count: 0,
                    opened_count: 0,
                    clicked_count: 0,
                    converted_count: 0,
                    created_by: "user".to_string(), // TODO: Get from auth context
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                if let Err(e) = repo.save_campaign(campaign).await {
                    log::error!("Failed to save campaign: {:?}", e);
                } else {
                    // Reset form
                    new_campaign_name.set(String::new());
                    new_campaign_description.set(String::new());
                    selected_template_id.set(None);
                    segment_criteria.set(String::new());
                    show_create_form.set(false);
                    // Refresh campaigns
                    campaigns.restart();
                }
            }
        });
    };

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "flex justify-between items-center",
                h1 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Campaign Manager" }
                button {
                    class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                    onclick: move |_| show_create_form.set(true),
                    "Create Campaign"
                }
            }

            // Create Campaign Form
            if show_create_form() {
                div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                        "Create New Campaign"
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Campaign Name"
                            }
                            Input {
                                placeholder: "e.g., Rate Lock Reminders",
                                value: new_campaign_name(),
                                oninput: move |e: FormEvent| new_campaign_name.set(e.value()),
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Campaign Type"
                            }
                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white",
                                value: "{new_campaign_type():?}",
                                onchange: move |e| {
                                    let value = e.value();
                                    let campaign_type = match value.as_str() {
                                        "Email" => CampaignType::Email,
                                        "SMS" => CampaignType::SMS,
                                        "DirectMail" => CampaignType::DirectMail,
                                        _ => CampaignType::Email,
                                    };
                                    new_campaign_type.set(campaign_type);
                                },
                                option { value: "Email", "Email" }
                                option { value: "SMS", "SMS" }
                                option { value: "DirectMail", "Direct Mail" }
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Description"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white",
                            placeholder: "Brief description of this campaign",
                            rows: 2,
                            value: "{new_campaign_description()}",
                            oninput: move |e| new_campaign_description.set(e.value()),
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Template"
                        }
                        select {
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white",
                            onchange: move |e| {
                                if let Ok(id) = e.value().parse::<i32>() {
                                    selected_template_id.set(Some(id));
                                } else {
                                    selected_template_id.set(None);
                                }
                            },
                            option { value: "", "Select a template..." }
                            match &*templates.value().read() {
                                Some(template_list) => {
                                    rsx! {
                                        for template in template_list.iter() {
                                            option {
                                                value: "{template.id}",
                                                selected: selected_template_id().map_or(false, |id| id == template.id),
                                                "{template.name} - {template.template_type:?}"
                                            }
                                        }
                                    }
                                }
                                None => rsx! {
                                    option { "Loading templates..." }
                                },
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Segmentation Criteria"
                        }
                        Input {
                            placeholder: "e.g., status:active,loan_type:refinance",
                            value: segment_criteria(),
                            oninput: move |e: FormEvent| segment_criteria.set(e.value()),
                        }
                        p { class: "text-xs text-gray-500 mt-1",
                            "Use format: key:value,key:value (e.g., status:active,loan_type:refinance)"
                        }
                    }

                    div { class: "flex justify-end space-x-3",
                        button {
                            class: "px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 transition-colors",
                            onclick: move |_| show_create_form.set(false),
                            "Cancel"
                        }
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                            disabled: selected_template_id().is_none() || new_campaign_name().is_empty(),
                            onclick: create_campaign,
                            "Create Campaign"
                        }
                    }
                }
            }

            // Campaigns List
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md",
                {
                    let campaigns_value = campaigns.value();
                    let campaigns_read = campaigns_value.read();
                    match &*campaigns_read {
                        Some(campaign_list) => {
                            let campaign_list = campaign_list.clone();
                            rsx! {
                                if campaign_list.is_empty() {
                                    div { class: "p-8 text-center text-gray-500 dark:text-gray-400",
                                        "No campaigns found. Create your first campaign to get started."
                                    }
                                } else {
                                    for campaign in campaign_list.iter() {
                                div { class: "divide-y divide-gray-200 dark:divide-gray-700",
                                    for campaign in campaign_list.iter() {
                                        div { class: "p-4 hover:bg-gray-50 dark:hover:bg-gray-700",
                                            div { class: "flex justify-between items-start",
                                                div { class: "flex-1",
                                                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                                                        "{campaign.name}"
                                                    }
                                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                                        "{campaign.description.as_deref().unwrap_or(\"\")}"
                                                    }
                                                    div { class: "flex items-center space-x-4 mt-2",
                                                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                                            "{campaign.campaign_type:?}"
                                                        }
                                                        span {
                                                            class: format!(

                                // Analytics Summary

                
                

                                                                "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}",
                                                                match campaign.status {
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
                                                                },
                                                            ),
                                                            "{campaign.status:?}"
                                                        }
                                                    }
                
                                                    if campaign.sent_count > 0 {
                                                        div { class: "mt-3 grid grid-cols-2 md:grid-cols-4 gap-4 text-sm",
                                                            div { class: "text-center",
                                                                div { class: "font-semibold text-gray-900 dark:text-white",
                                                                    "{campaign.sent_count}"
                                                                }
                                                                div { class: "text-gray-600 dark:text-gray-400", "Sent" }
                                                            }
                                                            div { class: "text-center",
                                                                div { class: "font-semibold text-gray-900 dark:text-white",
                                                                    "{(campaign.opened_count as f64 / campaign.sent_count as f64 * 100.0).round()}%"
                                                                }
                                                                div { class: "text-gray-600 dark:text-gray-400", "Open Rate" }
                                                            }
                                                            div { class: "text-center",
                                                                div { class: "font-semibold text-gray-900 dark:text-white",
                                                                    "{(campaign.clicked_count as f64 / campaign.sent_count as f64 * 100.0).round()}%"
                                                                }
                                                                div { class: "text-gray-600 dark:text-gray-400", "Click Rate" }
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
                                                    button {
                                                        class: "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 text-sm font-medium",
                                                        onclick: move |_| selected_campaign.set(Some(campaign.clone())),
                                                        "View Details"
                                                    }
                                                    if campaign.status == CampaignStatus::Draft {
                                                        button {
                                                            class: "text-green-600 hover:text-green-800 dark:text-green-400 dark:hover:text-green-200 text-sm font-medium",
                                                            onclick: move |_| {
                                                                spawn(async move {
                                                                    let repo = crate::get_repository();
                                                                    let mut updated_campaign = campaign.clone();
                                                                    updated_campaign.status = CampaignStatus::Scheduled;
                                                                    updated_campaign.scheduled_date = Some(chrono::Utc::now());
                                                                    if let Err(e) = repo.update_campaign(updated_campaign).await {
                                                                        log::error!("Failed to schedule campaign: {:?}", e);
                                                                    } else {
                                                                        campaigns.restart();
                                                                    }
                                                                });
                                                            },
                                                            "Schedule"
                                                        }
                                                    }
                                                    if matches!(campaign.status, CampaignStatus::Scheduled | CampaignStatus::Paused) {
                                                        button {
                                                            class: "text-green-600 hover:text-green-800 dark:text-green-400 dark:hover:text-green-200 text-sm font-medium",
                                                            onclick: move |_| {
                                                                spawn(async move {
                                                                    let repo = crate::get_repository();
                                                                    let mut updated_campaign = campaign.clone();
                                                                    updated_campaign.status = CampaignStatus::Running;
                                                                    if let Err(e) = repo.update_campaign(updated_campaign).await {
                                                                        log::error!("Failed to start campaign: {:?}", e);
                                                                    } else {
                                                                        campaigns.restart();
                                                                    }
                                                                });
                                                            },
                                                            "Start"
                                                        }
                                                    }
                                                    if campaign.status == CampaignStatus::Running {
                                                        button {
                                                            class: "text-orange-600 hover:text-orange-800 dark:text-orange-400 dark:hover:text-orange-200 text-sm font-medium",
                                                            onclick: move |_| {
                                                                spawn(async move {
                                                                    let repo = crate::get_repository();
                                                                    let mut updated_campaign = campaign.clone();
                                                                    updated_campaign.status = CampaignStatus::Paused;
                                                                    if let Err(e) = repo.update_campaign(updated_campaign).await {
                                                                        log::error!("Failed to pause campaign: {:?}", e);
                                                                    } else {
                                                                        campaigns.restart();
                                                                    }
                                                                });
                                                            },
                                                            "Pause"
                                                        }
                                                    }
                                                    button {
                                                        class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-200 text-sm font-medium",
                                                        onclick: move |_| {
                                                            spawn(async move {
                                                                let repo = crate::get_repository();
                                                                if let Err(e) = repo.delete_campaign(campaign.id).await {
                                                                    log::error!("Failed to delete campaign: {:?}", e);
                                                                } else {
                                                                    campaigns.restart();
                                                                }
                                                            });
                                                        },
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
                    None => {
                        rsx! {
                            div { class: "p-8 text-center",
                                div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4" }
                                p { class: "text-gray-600 dark:text-gray-400", "Loading campaigns..." }
                            }
                        }
                    }
                }
            }

            // Campaign Details Modal
            if let Some(campaign) = selected_campaign() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                        div { class: "p-6",
                            div { class: "flex justify-between items-start mb-4",
                                div {
                                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                                        "{campaign.name}"
                                    }
                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                        "{campaign.description.as_deref().unwrap_or(\"\")}"
                                    }
                                }
                                button {
                                    class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-200",
                                    onclick: move |_| selected_campaign.set(None),
                                    svg {
                                        class: "w-6 h-6",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M6 18L18 6M6 6l12 12",
                                        }
                                    }
                                }
                            }

                            // Campaign Stats
                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 mb-6",
                                div { class: "bg-gray-50 dark:bg-gray-700 p-4 rounded-lg text-center",
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white",
                                        "{campaign.sent_count}"
                                    }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "Emails Sent"
                                    }
                                }
                                div { class: "bg-gray-50 dark:bg-gray-700 p-4 rounded-lg text-center",
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white",
                                        "{campaign.opened_count}"
                                    }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "Opens"
                                    }
                                }
                                div { class: "bg-gray-50 dark:bg-gray-700 p-4 rounded-lg text-center",
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white",
                                        "{campaign.clicked_count}"
                                    }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "Clicks"
                                    }
                                }
                                div { class: "bg-gray-50 dark:bg-gray-700 p-4 rounded-lg text-center",
                                    div { class: "text-2xl font-bold text-gray-900 dark:text-white",
                                        "{campaign.converted_count}"
                                    }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "Conversions"
                                    }
                                }
                            }

                            // Performance Metrics
                            if campaign.sent_count > 0 {
                                div { class: "mb-6",
                                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-3",
                                        "Performance Metrics"
                                    }
                                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                        div { class: "bg-blue-50 dark:bg-blue-900 p-4 rounded-lg",
                                            div { class: "text-xl font-bold text-blue-600 dark:text-blue-400",
                                                "{(campaign.opened_count as f64 / campaign.sent_count as f64 * 100.0).round()}%"
                                            }
                                            div { class: "text-sm text-blue-600 dark:text-blue-400",
                                                "Open Rate"
                                            }
                                        }
                                        div { class: "bg-green-50 dark:bg-green-900 p-4 rounded-lg",
                                            div { class: "text-xl font-bold text-green-600 dark:text-green-400",
                                                "{(campaign.clicked_count as f64 / campaign.sent_count as f64 * 100.0).round()}%"
                                            }
                                            div { class: "text-sm text-green-600 dark:text-green-400",
                                                "Click Rate"
                                            }
                                        }
                                        div { class: "bg-purple-50 dark:bg-purple-900 p-4 rounded-lg",
                                            div { class: "text-xl font-bold text-purple-600 dark:text-purple-400",
                                                "{(campaign.converted_count as f64 / campaign.sent_count as f64 * 100.0).round()}%"
                                            }
                                            div { class: "text-sm text-purple-600 dark:text-purple-400",
                                                "Conversion Rate"
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex justify-end",
                                button {
                                    class: "bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                                    onclick: move |_| selected_campaign.set(None),
                                    "Close"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
