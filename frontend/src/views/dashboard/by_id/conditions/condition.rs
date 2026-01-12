use dioxus::prelude::*;
use crate::get_repository;
use shared::models::condition::{CreateConditionRequest, UpdateConditionRequest, ConditionType, ConditionSeverity, ConditionStatus};

#[derive(Props, PartialEq, Clone)]
pub struct ConditionFormProps {
    show_form: Signal<bool>,
    form_title: Signal<String>,
    form_description: Signal<String>,
    form_type: Signal<ConditionType>,
    form_severity: Signal<ConditionSeverity>,
    form_status: Signal<ConditionStatus>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn ConditionForm(props: ConditionFormProps) -> Element {
    let ConditionFormProps {
        show_form,
        mut form_title,
        mut form_description,
        mut form_type,
        mut form_severity,
        mut form_status,
        on_submit,
    } = props;

    rsx! {
        div { class: "mb-6 bg-neutral-secondary-soft border border-default rounded-base p-4",
            h3 { class: "text-lg font-semibold mb-3 text-heading", "Add New Condition" }

            div { class: "space-y-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Title" }
                    input {
                        class: "w-full px-3 py-2 border border-default rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white text-gray-900",
                        placeholder: "Condition title...",
                        value: "{form_title}",
                        oninput: move |evt| form_title.set(evt.value()),
                    }
                }

                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Description" }
                    textarea {
                        class: "w-full px-3 py-2 border border-default rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white text-gray-900 min-h-[80px]",
                        placeholder: "Condition description...",
                        value: "{form_description}",
                        oninput: move |evt| form_description.set(evt.value()),
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Type"
                        }
                        select {
                            class: "w-full px-3 py-2 border border-default rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white text-gray-900",
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
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Severity"
                        }
                        select {
                            class: "w-full px-3 py-2 border border-default rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white text-gray-900",
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
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Status"
                        }
                        select {
                            class: "w-full px-3 py-2 border border-default rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white text-gray-900",
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
                        onclick: move |_| on_submit.call(()),
                        "Save Condition"
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ConditionItemProps {
    condition: shared::models::condition::Condition,
    on_request: EventHandler<i32>,
    on_complete: EventHandler<i32>,
    on_delete: EventHandler<i32>,
}

#[component]
pub fn ConditionItem(props: ConditionItemProps) -> Element {
    let ConditionItemProps {
        condition,
        on_request,
        on_complete,
        on_delete,
    } = props;

    rsx! {
        div {
            key: "{condition.id}",
            class: "p-4 bg-white border border-default rounded-base shadow-xs hover:shadow-sm transition-shadow",

            div { class: "flex justify-between items-start mb-3",
                div { class: "flex-1",
                    h4 { class: "font-medium text-heading", "{condition.title}" }
                    p { class: "text-sm text-body mt-1", "{condition.description}" }
                    div { class: "flex gap-2 mt-2",
                        span { class: "px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full",
                            "{condition.condition_type:?}"
                        }
                        span { class: "px-2 py-1 text-xs bg-yellow-100 text-yellow-800 rounded-full",
                            "{condition.severity:?}"
                        }
                        span { class: "px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full",
                            "{condition.status:?}"
                        }
                    }
                }
                div { class: "flex gap-2 ml-4",
                    button {
                        class: "px-3 py-1 text-xs bg-orange-100 text-orange-800 rounded hover:bg-orange-200 transition-colors",
                        onclick: move |_| on_request.call(condition.id),
                        "Request"
                    }
                    button {
                        class: "px-3 py-1 text-xs bg-green-100 text-green-800 rounded hover:bg-green-200 transition-colors",
                        onclick: move |_| on_complete.call(condition.id),
                        "Complete"
                    }
                    button {
                        class: "px-3 py-1 text-xs bg-red-100 text-red-800 rounded hover:bg-red-200 transition-colors",
                        onclick: move |_| on_delete.call(condition.id),
                        "Delete"
                    }
                }
            }

            div { class: "text-xs text-gray-500 mt-2",
                {
                    let created = condition.created_at.format("%Y-%m-%d %H:%M").to_string();
                    let updated = condition.updated_at.format("%Y-%m-%d %H:%M").to_string();
                    format!("Created: {} • Updated: {}", created, updated)
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ConditionListProps {
    conditions: Resource<Result<Vec<shared::models::condition::Condition>, String>>,
    on_request: EventHandler<i32>,
    on_complete: EventHandler<i32>,
    on_delete: EventHandler<i32>,
}

#[component]
pub fn ConditionList(props: ConditionListProps) -> Element {
    let ConditionListProps {
        conditions,
        on_request,
        on_complete,
        on_delete,
    } = props;

    rsx! {
        div {
            h3 { class: "text-lg font-semibold mb-3 text-heading", "All Conditions" }

            if let Some(Ok(condition_list)) = conditions.read().as_ref() {
                if condition_list.is_empty() {
                    div { class: "text-center py-8 text-body",
                        p { "No conditions added yet." }
                    }
                } else {
                    div { class: "space-y-3",
                        for condition in condition_list.iter() {
                            ConditionItem {
                                condition: condition.clone(),
                                on_request: on_request.clone(),
                                on_complete: on_complete.clone(),
                                on_delete: on_delete.clone(),
                            }
                        }
                    }
                }
            } else if let Some(Err(e)) = conditions.read().as_ref() {
                div { class: "bg-red-50 border border-red-300 text-red-800 px-4 py-3 rounded-base",
                    p { "Error loading conditions: {e}" }
                }
            } else {
                div { class: "text-center py-8",
                    p { class: "text-body", "Loading conditions..." }
                }
            }
        }
    }
}

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
        div { class: "bg-neutral-primary-soft border border-default rounded-base p-6 shadow-xs",
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-2xl font-bold text-heading", "Conditions" }
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
                ConditionForm {
                    show_form: show_form.clone(),
                    form_title: form_title.clone(),
                    form_description: form_description.clone(),
                    form_type: form_type.clone(),
                    form_severity: form_severity.clone(),
                    form_status: form_status.clone(),
                    on_submit: EventHandler::new(move |_| handle_create_condition(())),
                }
            }

            // Conditions List
            ConditionList {
                conditions: conditions.clone(),
                on_request: EventHandler::new(handle_request),
                on_complete: EventHandler::new(handle_complete),
                on_delete: EventHandler::new(handle_delete),
            }
        }
    }
}
