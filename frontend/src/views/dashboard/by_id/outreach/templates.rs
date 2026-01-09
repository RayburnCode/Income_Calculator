use dioxus::prelude::*;
use shared::models::{OutreachTemplate, TemplateType};
use crate::components::input::Input;
use crate::components::tab::Tab;
use crate::components::accordion::Accordion;

#[component]
pub fn Templates() -> Element {
    let mut templates = use_resource(|| async {
        let repo = crate::get_repository();
        repo.get_all_outreach_templates().await.unwrap_or_default()
    });

    let mut selected_template = use_signal(|| None::<OutreachTemplate>);
    let mut show_create_form = use_signal(|| false);
    let mut new_template_name = use_signal(|| String::new());
    let mut new_template_type = use_signal(|| TemplateType::Email);
    let mut new_template_subject = use_signal(|| String::new());
    let mut new_template_content = use_signal(|| String::new());
    let mut new_template_description = use_signal(|| String::new());

    let create_template = move |_| {
        spawn(async move {
            let repo = crate::get_repository();
            let template = OutreachTemplate {
                id: 0, // Will be set by database
                name: new_template_name(),
                template_type: new_template_type(),
                subject: if new_template_subject().is_empty() { None } else { Some(new_template_subject()) },
                content: new_template_content(),
                description: if new_template_description().is_empty() { None } else { Some(new_template_description()) },
                is_default: false,
                is_active: true,
                created_by: Some("user".to_string()), // TODO: Get from auth context
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            if let Err(e) = repo.save_outreach_template(template).await {
                log::error!("Failed to save template: {:?}", e);
            } else {
                // Reset form
                new_template_name.set(String::new());
                new_template_subject.set(String::new());
                new_template_content.set(String::new());
                new_template_description.set(String::new());
                show_create_form.set(false);
                // Refresh templates
                templates.restart();
            }
        });
    };

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "flex justify-between items-center",
                h1 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Outreach Templates" }
                button {
                    class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                    onclick: move |_| show_create_form.set(true),
                    "Create Template"
                }
            }

            // Create Template Form
            if show_create_form() {
                div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                        "Create New Template"
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Template Name"
                            }
                            Input {
                                placeholder: "e.g., Welcome Email",
                                value: new_template_name(),
                                oninput: move |e: FormEvent| new_template_name.set(e.value()),
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Template Type"
                            }
                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white",
                                value: "{new_template_type():?}",
                                onchange: move |e| {
                                    let value = e.value();
                                    let template_type = match value.as_str() {
                                        "Email" => TemplateType::Email,
                                        "Letter" => TemplateType::Letter,
                                        "DocumentRequest" => TemplateType::DocumentRequest,
                                        "StatusUpdate" => TemplateType::StatusUpdate,
                                        "WelcomeMessage" => TemplateType::WelcomeMessage,
                                        "FollowUp" => TemplateType::FollowUp,
                                        _ => TemplateType::Other,
                                    };
                                    new_template_type.set(template_type);
                                },
                                option { value: "Email", "Email" }
                                option { value: "Letter", "Letter" }
                                option { value: "DocumentRequest", "Document Request" }
                                option { value: "StatusUpdate", "Status Update" }
                                option { value: "WelcomeMessage", "Welcome Message" }
                                option { value: "FollowUp", "Follow Up" }
                                option { value: "Other", "Other" }
                            }
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Subject (for emails)"
                        }
                        Input {
                            placeholder: "Email subject line",
                            value: new_template_subject(),
                            oninput: move |e: FormEvent| new_template_subject.set(e.value()),
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Description"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white",
                            placeholder: "Brief description of this template",
                            rows: 2,
                            value: "{new_template_description()}",
                            oninput: move |e| new_template_description.set(e.value()),
                        }
                    }

                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Content"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white font-mono",
                            placeholder: "Template content with placeholders like {{borrower_name}}, {{loan_amount}}, etc.",
                            rows: 10,
                            value: "{new_template_content()}",
                            oninput: move |e| new_template_content.set(e.value()),
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
                            onclick: create_template,
                            "Create Template"
                        }
                    }
                }
            }

            // Templates List
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md",
                {
                    let templates_value = templates.value();
                    let templates_read = templates_value.read();
                    match &*templates_read {
                        Some(templates_list) => {
                            let templates_list = templates_list.clone();
                            rsx! {
                                if templates_list.is_empty() {
                                    div { class: "p-8 text-center text-gray-500 dark:text-gray-400",
                                        "No templates found. Create your first template to get started."
                                    }
                                } else {
                                    for template in templates_list.iter() {
                                div { class: "divide-y divide-gray-200 dark:divide-gray-700",
                                    for template in templates_list.iter() {
                                        div { class: "p-4 hover:bg-gray-50 dark:hover:bg-gray-700",
                                            div { class: "flex justify-between items-start",
                                                div { class: "flex-1",
                                                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                                                        "{template.name}"
                                                    }
                                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                                        "{template.description.as_deref().unwrap_or(\"\")}"
                                                    }
                                                    div { class: "flex items-center space-x-4 mt-2",
                                                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                                            "{template.template_type:?}"
                                                        }
                                                        if template.is_default {
                                                            span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                                                                "Default"
                                                            }
                                                        }
                                                    }
                                                }
                                                div { class: "flex space-x-2",
                                                    button {
                                                        class: "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 text-sm font-medium",
                                                        onclick: move |_| selected_template.set(Some(template.clone())),
                                                        "View"
                                                    }
                                                    if !template.is_default {
                                                        button {
                                                            class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-200 text-sm font-medium",
                                                            onclick: move |_| {
                                                                spawn(async move {
                                                                    let repo = crate::get_repository();
                                                                    if let Err(e) = repo.delete_outreach_template(template.id).await {
                                                                        log::error!("Failed to delete template: {:?}", e);
                                                                    } else {
                                                                        templates.restart();
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
                    }
                    None => {
                        rsx! {
                            div { class: "p-8 text-center",
                                div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4" }
                                p { class: "text-gray-600 dark:text-gray-400", "Loading templates..." }
                            }
                        }
                    }
                }
            }

            // Template Preview Modal
            if let Some(template) = selected_template() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                        div { class: "p-6",
                            div { class: "flex justify-between items-start mb-4",
                                div {
                                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                                        "{template.name}"
                                    }
                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                        "{template.description.as_deref().unwrap_or(\"\")}"
                                    }
                                }
                                button {
                                    class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-200",
                                    onclick: move |_| selected_template.set(None),
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

                            if let Some(subject) = &template.subject {
                                div { class: "mb-4",
                                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                        "Subject"
                                    }
                                    div { class: "bg-gray-50 dark:bg-gray-700 p-3 rounded-md",
                                        "{subject}"
                                    }
                                }
                            }

                            div { class: "mb-4",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                    "Content"
                                }
                                div { class: "bg-gray-50 dark:bg-gray-700 p-3 rounded-md whitespace-pre-wrap font-mono text-sm",
                                    "{template.content}"
                                }
                            }

                            div { class: "flex justify-end",
                                button {
                                    class: "bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                                    onclick: move |_| selected_template.set(None),
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