use dioxus::prelude::*;
use shared::models::{TimelineEvent, TimelineEventType};
use chrono::{DateTime, Utc};

#[component]
pub fn Timeline(borrower_id: i32) -> Element {
    let timeline_events = use_signal(|| Vec::<TimelineEvent>::new());
    let is_loading = use_signal(|| true);
    let error_message = use_signal(|| None::<String>);

    // Load timeline events
    use_effect(move || {
        let mut timeline_events = timeline_events.clone();
        let mut is_loading = is_loading.clone();
        let mut error_message = error_message.clone();
        let client_id = borrower_id;

        spawn(async move {
            match repository::Repository::new().await {
                Ok(repo) => {
                    match repo.get_timeline_events(client_id).await {
                        Ok(events) => {
                            timeline_events.set(events);
                            error_message.set(None);
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Failed to load timeline: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to connect to database: {}", e)));
                }
            }
            is_loading.set(false);
        });
    });

    // Function to get icon for event type
    let get_event_icon = |event_type: &TimelineEventType| -> &'static str {
        match event_type {
            TimelineEventType::ClientCreated => "👤",
            TimelineEventType::EmailSent => "📧",
            TimelineEventType::EmailReceived => "📨",
            TimelineEventType::PhoneCall => "📞",
            TimelineEventType::Meeting => "🤝",
            TimelineEventType::DocumentRequest => "📄",
            TimelineEventType::DocumentUploaded => "📎",
            TimelineEventType::StatusChange => "🔄",
            TimelineEventType::NoteAdded => "📝",
            TimelineEventType::TaskCompleted => "✅",
            TimelineEventType::ApplicationSubmitted => "📋",
            TimelineEventType::LoanCalculated => "🧮",
            TimelineEventType::Milestone => "🏆",
            TimelineEventType::Other => "📌",
        }
    };

    // Function to format timestamp
    let format_timestamp = |timestamp: DateTime<Utc>| -> String {
        timestamp.format("%b %d, %Y at %I:%M %p").to_string()
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold text-gray-800 dark:text-gray-200 mb-6",
                "Client Timeline"
            }

            // Error message
            if let Some(error) = error_message() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6",
                    "{error}"
                }
            }

            // Loading state
            if *is_loading.read() {
                div { class: "flex justify-center items-center py-8",
                    div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" }
                    span { class: "ml-2 text-gray-600 dark:text-gray-300", "Loading timeline..." }
                }
            } else {
                // Timeline content
                if timeline_events().is_empty() {
                    div { class: "text-center py-8 text-gray-500 dark:text-gray-400",
                        "No timeline events yet. Events will appear here as you interact with the client."
                    }
                } else {
                    div { class: "relative",
                        // Timeline line
                        div { class: "absolute left-6 top-0 bottom-0 w-0.5 bg-gray-300 dark:bg-gray-600" }

                        // Timeline events
                        div { class: "space-y-6",
                            for event in timeline_events().iter() {
                                div { class: "relative flex items-start",
                                    // Event icon
                                    div { class: "flex-shrink-0 w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center text-xl",
                                        "{get_event_icon(&event.event_type)}"
                                    }

                                    // Event content
                                    div { class: "ml-4 flex-1 min-w-0",
                                        div { class: "flex items-center justify-between",
                                            h3 { class: "text-sm font-medium text-gray-900 dark:text-gray-100",
                                                "{event.title}"
                                            }
                                            time { class: "text-sm text-gray-500 dark:text-gray-400",
                                                "{format_timestamp(event.created_at)}"
                                            }
                                        }

                                        if let Some(description) = &event.description {
                                            p { class: "mt-1 text-sm text-gray-600 dark:text-gray-300",
                                                "{description}"
                                            }
                                        }

                                        // User info
                                        if let Some(user_id) = &event.user_id {
                                            p { class: "mt-2 text-xs text-gray-500 dark:text-gray-400",
                                                "by {user_id}"
                                            }
                                        }

                                        // Event type badge
                                        div { class: "mt-2",
                                            span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
                                                "{event.event_type:?}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Add new event button (placeholder for future functionality)
            div { class: "mt-6 pt-6 border-t border-gray-200 dark:border-gray-700",
                button { class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "Add Timeline Event"
                }
            }
        }
    }
}