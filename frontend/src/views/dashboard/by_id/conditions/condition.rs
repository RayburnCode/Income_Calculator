use dioxus::prelude::*;
use crate::get_repository;

#[component]
pub fn ClientConditions(id: i32) -> Element {
    let _client_data = use_resource(move || async move {
        let repo = get_repository();
        repo.get_borrower(id).await
    });
    let conditions = use_resource(move || async move {
        let repo = get_repository();
        repo.get_conditions_by_client(id).await.map_err(|e| e.to_string())
    });
    rsx! {
        div { class: "bg-neutral-primary-soft border border-default rounded-base p-6 shadow-xs",
            h2 { class: "text-lg font-medium mb-4", "Client Conditions" }

            if let Some(Ok(condition_list)) = conditions.read().as_ref() {
                if condition_list.is_empty() {
                    div { class: "text-center py-8 text-gray-600",
                        p { "No conditions found for this client." }
                    }
                } else {
                    div { class: "space-y-3",
                        for condition in condition_list.iter() {
                            div {
                                key: "{condition.id}",
                                class: "p-4 bg-white border border-default rounded-base shadow-xs hover:shadow-sm transition-shadow",
                                div { class: "flex justify-between items-start",
                                    div {
                                        h4 { class: "font-medium text-gray-900", "{condition.title}" }
                                        p { class: "text-sm text-gray-600 mt-1",
                                            "{condition.description}"
                                        }
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
                                }
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
                    p { class: "text-gray-600", "Loading conditions..." }
                }
            }
        }
    }
}
