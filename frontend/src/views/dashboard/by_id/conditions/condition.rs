use dioxus::prelude::*;
use crate::get_repository;
use shared::models::condition::{CreateConditionRequest, UpdateConditionRequest, ConditionType, ConditionSeverity, ConditionStatus};

#[component]
pub fn ClientConditions(id: i32) -> Element {
    let _client_data = use_resource(move || async move {
        let repo = get_repository();
        repo.get_borrower(id).await
    });
    let mut conditions = use_resource(move || async move {
        let repo = get_repository();
        repo.get_conditions_by_client(id).await.map_err(|e| e.to_string())
    });

    // Form state
    let mut show_form = use_signal(|| false);
    let mut form_title = use_signal(|| String::new());
    let mut form_description = use_signal(|| String::new());
    let mut form_type = use_signal(|| ConditionType::Other);
    let mut form_severity = use_signal(|| ConditionSeverity::Medium);
    let mut form_status = use_signal(|| ConditionStatus::Active);

    // Messages
    let mut error_message = use_signal(|| String::new());
    let mut success_message = use_signal(|| String::new());

    // Event handlers
    let handle_create_condition = move |_| {
        spawn(async move {
            error_message.set(String::new());
            success_message.set(String::new());

            let title = form_title.read().clone();
            let description = form_description.read().clone();

            if title.trim().is_empty() {
                error_message.set("Condition title cannot be empty".to_string());
                return;
            }

            if description.trim().is_empty() {
                error_message.set("Condition description cannot be empty".to_string());
                return;
            }

            let create_request = CreateConditionRequest {
                client_id: id,
                title,
                description,
                condition_type: form_type(),
                severity: form_severity(),
                status: form_status(),
            };

            let repo = get_repository();
            match repo.create_condition(create_request).await {
                Ok(_) => {
                    success_message.set("Condition added successfully!".to_string());
                    form_title.set(String::new());
                    form_description.set(String::new());
                    form_type.set(ConditionType::Other);
                    form_severity.set(ConditionSeverity::Medium);
                    form_status.set(ConditionStatus::Active);
                    show_form.set(false);
                    conditions.restart();
                }
                Err(e) => {
                    error_message.set(format!("Failed to add condition: {}", e));
                }
            }
        });
    };

    let update_condition_status = move |condition_id: i32, new_status: ConditionStatus| {
        spawn(async move {
            let update_request = UpdateConditionRequest {
                title: None,
                description: None,
                condition_type: None,
                severity: None,
                status: Some(new_status),
            };

            let repo = get_repository();
            match repo.update_condition(condition_id, update_request).await {
                Ok(_) => {
                    conditions.restart();
                }
                Err(e) => {
                    error_message.set(format!("Failed to update condition: {}", e));
                }
            }
        });
    };

    let handle_request = move |condition_id: i32| {
        update_condition_status(condition_id, ConditionStatus::Monitoring);
    };

    let handle_complete = move |condition_id: i32| {
        update_condition_status(condition_id, ConditionStatus::Resolved);
    };

    let handle_delete = move |condition_id: i32| {
        spawn(async move {
            let repo = get_repository();
            if let Ok(_) = repo.delete_condition(condition_id).await {
                conditions.restart();
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 dark:bg-gray-900 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-gray-100",
                        "Conditions"
                    }
                    p { class: "text-gray-600 dark:text-gray-400 mt-2",
                        "Manage and track client conditions and requirements"
                    }
                }

                // Main Content
                div { class: "space-y-6",
                    div { class: "flex justify-between items-center",
                        h2 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100",
                            "Conditions"
                        }
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium shadow-sm hover:shadow-md",
                            onclick: move |_| show_form.set(!show_form()),
                            if show_form() {
                                "Cancel"
                            } else {
                                "Add Condition"
                            }
                        }
                    }

                    // Success/Error Messages
                    if !error_message().is_empty() {
                        div { class: "mb-4 bg-red-50 border border-red-300 text-red-800 px-4 py-3 rounded-base",
                            p { "{error_message}" }
                        }
                    }
                    if !success_message().is_empty() {
                        div { class: "mb-4 bg-green-50 border border-green-300 text-green-800 px-4 py-3 rounded-base",
                            p { "{success_message}" }
                        }
                    }

                    // Add Condition Form
                    if show_form() {
                        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6",
                            h3 { class: "text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100",
                                "Add New Condition"
                            }

                            div { class: "space-y-4",
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                        "Title"
                                    }
                                    input {
                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                        placeholder: "Condition title...",
                                        value: "{form_title}",
                                        oninput: move |evt| form_title.set(evt.value()),
                                    }
                                }

                                div {
                                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                        "Description"
                                    }
                                    textarea {
                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 min-h-[80px]",
                                        placeholder: "Condition description...",
                                        value: "{form_description}",
                                        oninput: move |evt| form_description.set(evt.value()),
                                    }
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Type"
                                        }
                                        select {
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                            value: "{form_type():?}",
                                            onchange: move |evt| {
                                                let value = evt.value();
                                                let condition_type = match value.as_str() {
                                                    "Medical" => ConditionType::Medical,
                                                    "Financial" => ConditionType::Financial,
                                                    "Legal" => ConditionType::Legal,
                                                    "Employment" => ConditionType::Employment,
                                                    "Housing" => ConditionType::Housing,
                                                    _ => ConditionType::Other,
                                                };
                                                form_type.set(condition_type);
                                            },
                                            option { value: "Medical", "Medical" }
                                            option { value: "Financial", "Financial" }
                                            option { value: "Legal", "Legal" }
                                            option { value: "Employment", "Employment" }
                                            option { value: "Housing", "Housing" }
                                            option { value: "Other", "Other" }
                                        }
                                    }

                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Severity"
                                        }
                                        select {
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                            value: "{form_severity():?}",
                                            onchange: move |evt| {
                                                let value = evt.value();
                                                let severity = match value.as_str() {
                                                    "Low" => ConditionSeverity::Low,
                                                    "High" => ConditionSeverity::High,
                                                    "Critical" => ConditionSeverity::Critical,
                                                    _ => ConditionSeverity::Medium,
                                                };
                                                form_severity.set(severity);
                                            },
                                            option { value: "Low", "Low" }
                                            option { value: "Medium", "Medium" }
                                            option { value: "High", "High" }
                                            option { value: "Critical", "Critical" }
                                        }
                                    }

                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Status"
                                        }
                                        select {
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                            value: "{form_status():?}",
                                            onchange: move |evt| {
                                                let value = evt.value();
                                                let status = match value.as_str() {
                                                    "Resolved" => ConditionStatus::Resolved,
                                                    "Monitoring" => ConditionStatus::Monitoring,
                                                    "Inactive" => ConditionStatus::Inactive,
                                                    _ => ConditionStatus::Active,
                                                };
                                                form_status.set(status);
                                            },
                                            option { value: "Active", "Active" }
                                            option { value: "Resolved", "Resolved" }
                                            option { value: "Monitoring", "Monitoring" }
                                            option { value: "Inactive", "Inactive" }
                                        }
                                    }
                                }

                                div { class: "flex justify-end",
                                    button {
                                        class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium",
                                        onclick: move |_| handle_create_condition(()),
                                        "Save Condition"
                                    }
                                }
                            }
                        }
                    }

                    // Conditions List
                    div {
                        h3 { class: "text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100",
                            "All Conditions"
                        }

                        {
                            // Clone the data outside the rsx! block to avoid lifetime issues
                            let condition_list = if let Some(Ok(list)) = conditions.read().as_ref() {
                                Some(list.clone())
                            } else {
                                None
                            };

                            if let Some(condition_list) = condition_list {
                                if condition_list.is_empty() {
                                    rsx! {
                                        div { class: "text-center py-12 text-gray-600 dark:text-gray-400",
                                            p { "No conditions added yet." }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "space-y-4",
                                            for condition in condition_list.into_iter() {
                                                {
                                                    let condition_id = condition.id;
                                                    rsx! {
                                                        div {
                                                            key: "{condition.id}",
                                                            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 hover:shadow-md transition-shadow",

                
                        
                

                                                            div { class: "flex justify-between items-start mb-3",
                                                                div { class: "flex-1",
                                                                    h4 { class: "font-medium text-gray-900 dark:text-gray-100", "{condition.title}" }
                                                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "{condition.description}" }
                                                                    div { class: "flex gap-2 mt-2",
                                                                        span { class: "px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full",
                                                                            "{condition.condition_type:?}"
                                                                        }
                                                                        span { class: "px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded-full",
                                                                            "{condition.severity:?}"
                                                                        }
                                                                        span { class: "px-2 py-1 text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full",
                                                                            "{condition.status:?}"
                                                                        }
                                                                    }
                                                                }
                                                                div { class: "flex gap-2 ml-4",
                                                                    button {
                                                                        class: "px-3 py-1 text-xs bg-orange-100 dark:bg-orange-900 text-orange-800 dark:text-orange-200 rounded hover:bg-orange-200 dark:hover:bg-orange-800 transition-colors",
                                                                        onclick: move |_| handle_request(condition_id),
                                                                        "Request"
                                                                    }
                                                                    button {
                                                                        class: "px-3 py-1 text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded hover:bg-green-200 dark:hover:bg-green-800 transition-colors",
                                                                        onclick: move |_| handle_complete(condition_id),
                                                                        "Complete"
                                                                    }
                                                                    button {
                                                                        class: "px-3 py-1 text-xs bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded hover:bg-red-200 dark:hover:bg-red-800 transition-colors",
                                                                        onclick: move |_| handle_delete(condition_id),
                                                                        "Delete"
                                                                    }
                                                                }
                                                            }
                        
                                                            div { class: "text-xs text-gray-500 dark:text-gray-400 mt-2",
                                                                {
                                                                    let created = condition.created_at.format("%Y-%m-%d %H:%M").to_string();
                                                                    let updated = condition.updated_at.format("%Y-%m-%d %H:%M").to_string();
                                                                    format!("Created: {} • Updated: {}", created, updated)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if let Some(Err(e)) = conditions.read().as_ref() {
                                rsx! {
                                    div { class: "bg-red-50 border border-red-300 text-red-800 px-4 py-3 rounded-base",
                                        p { "Error loading conditions: {e}" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "text-center py-12",
                                        p { class: "text-gray-600 dark:text-gray-400", "Loading conditions..." }
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
