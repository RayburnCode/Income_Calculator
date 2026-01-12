//! UI components for the template system

use dioxus::prelude::*;
use shared::models::{OutreachTemplate, Borrower};
use crate::components::input::Input;

/// Modal for using templates with client data
#[component]
pub fn TemplateUseModal(
    show_modal: Signal<bool>,
    templates: Resource<Vec<OutreachTemplate>>,
    borrower: Resource<Option<Borrower>>,
    selected_template: Signal<Option<OutreachTemplate>>,
    generated_content: Signal<String>,
    generated_subject: Signal<String>,
    on_use_template: EventHandler<OutreachTemplate>,
) -> Element {
    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                div { class: "p-6",
                    div { class: "flex justify-between items-start mb-6",
                        h2 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                            "Use Template with Client Data"
                        }
                        button {
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-200",
                            onclick: move |_| {
                                show_modal.set(false);
                                selected_template.set(None);
                                generated_content.set(String::new());
                                generated_subject.set(String::new());
                            },
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

                    // Template Selection/Content
                    {
                        if selected_template().is_none() {
                            rsx! {
                                div { class: "mb-6",

                                // TODO: Add functionality to send email, create timeline event, etc.

            

                                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4", "Select a Template" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                        {
                                            match templates.read_unchecked().as_ref() {
                                                Some(templates_list) => {
                                                    let templates_list = templates_list.clone();
                                                    rsx! {
                                                        for template in templates_list.into_iter().filter(|t| t.is_active) {
                                                            div {
                                                                class: "border border-gray-200 dark:border-gray-600 rounded-lg p-4 hover:border-blue-500 dark:hover:border-blue-400 cursor-pointer transition-colors",
                                                                onclick: move |_| on_use_template.call(template.clone()),
                                                                div { class: "flex justify-between items-start mb-2",
                                                                    h4 { class: "font-medium text-gray-900 dark:text-white", "{template.name}" }
                                                                    span { class: "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                                                        "{template.template_type:?}"
                                                                    }
                                                                }
                                                                p { class: "text-sm text-gray-600 dark:text-gray-400",
                                                                    "{template.description.as_deref().unwrap_or(\"\")}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                None => rsx! {
                                                    div { class: "text-center py-8",
                                                        div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4" }
                                                        p { class: "text-gray-600 dark:text-gray-400", "Loading templates..." }
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        } else if let Some(template) = selected_template() {
                            rsx! {
                                div { class: "space-y-6",
                                    div { class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4",
                                        div { class: "flex items-center",
                                            svg {
                                                class: "w-5 h-5 text-blue-600 dark:text-blue-400 mr-2",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z",
                                                    clip_rule: "evenodd",
                                                }
                                            }
                                            p { class: "text-sm text-blue-800 dark:text-blue-200",
                                                "Template variables have been automatically filled with {
                                                                                                                                                    borrower.read_unchecked().as_ref()
                                                                                                                                                        .and_then(|b| b.as_ref())
                                                                                                                                                        .map(|b| b.name.clone())
                                                                                                                                                        .unwrap_or_default()
                                                                                                                                                }'s information."
                                            }
                                        }
                                    }
            
                                    {if !generated_subject().is_empty() { Some(rsx! {
                                        div {
                                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "Subject"
                                            }
                                            div { class: "bg-gray-50 dark:bg-gray-700 p-3 rounded-md border", "{generated_subject()}" }
                                        }
                                    }) } else { None }}
            
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                            "Content"
                                        }
                                        div { class: "bg-gray-50 dark:bg-gray-700 p-4 rounded-md border whitespace-pre-wrap font-mono text-sm max-h-96 overflow-y-auto",
                                            "{generated_content()}"
                                        }
                                    }
            
                                    div { class: "flex justify-between items-center pt-4 border-t border-gray-200 dark:border-gray-600",
                                        div { class: "text-sm text-gray-600 dark:text-gray-400", "Template: {template.name}" }
                                        div { class: "flex space-x-3",
                                            button {
                                                class: "px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 transition-colors",
                                                onclick: move |_| {
                                                    selected_template.set(None);
                                                    generated_content.set(String::new());
                                                    generated_subject.set(String::new());
                                                },
                                                "Back to Templates"
                                            }
                                            button {
                                                class: "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                                                onclick: move |_| {
                                                    log::info!("Template used: {}", template.name);
                                                    show_modal.set(false);
                                                    selected_template.set(None);
                                                    generated_content.set(String::new());
                                                    generated_subject.set(String::new());
                                                },
                                                "Use This Template"
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "text-center py-8 text-gray-500 dark:text-gray-400", "No template selected" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Template variable reference component
#[component]
pub fn TemplateVariablesHelp() -> Element {
    let variables = super::engine::TemplateEngine::get_available_variables();

    rsx! {
        div { class: "bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg p-4",
            h4 { class: "font-medium text-gray-900 dark:text-white mb-3",
                "Available Template Variables"
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-2 text-sm",
                for (var , desc) in variables {
                    div { class: "flex justify-between",
                        code { class: "bg-white dark:bg-gray-700 px-2 py-1 rounded text-blue-600 dark:text-blue-400 font-mono",
                            "{{{var}}}"
                        }
                        span { class: "text-gray-600 dark:text-gray-400", "{desc}" }
                    }
                }
            }
        }
    }
}

/// Template preview component
#[component]
pub fn TemplatePreview(
    template: OutreachTemplate,
    borrower: Option<Borrower>,
) -> Element {
    let borrower_for_content = borrower.clone();
    let processed_content = use_memo(move || {
        if let Some(borrower_ref) = borrower_for_content.as_ref() {
            super::engine::TemplateEngine::replace_variables(&template.content, borrower_ref)
        } else {
            template.content.clone()
        }
    });

    let processed_subject = use_memo(move || {
        template.subject.as_ref().map(|subject| {
            if let Some(borrower_ref) = borrower.as_ref() {
                super::engine::TemplateEngine::replace_variables(subject, borrower_ref)
            } else {
                subject.clone()
            }
        })
    });

    rsx! {
        div { class: "space-y-4",
            {processed_subject().map(|subject| rsx! {
                div {
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                        "Subject"
                    }
                    div { class: "bg-white dark:bg-gray-700 p-3 rounded-md border font-medium", "{subject}" }
                }
            })}

            div {
                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                    "Content Preview"
                }
                div { class: "bg-white dark:bg-gray-700 p-4 rounded-md border whitespace-pre-wrap font-mono text-sm max-h-96 overflow-y-auto",
                    "{processed_content()}"
                }
            }
        }
    }
}