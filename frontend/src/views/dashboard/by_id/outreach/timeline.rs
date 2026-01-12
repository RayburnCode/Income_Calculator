use dioxus::prelude::*;
use shared::models::{TimelineEvent, TimelineEventType};
use chrono::{DateTime, Utc};

#[component]
pub fn Timeline(id: i32) -> Element {
    let timeline_events = use_signal(|| Vec::<TimelineEvent>::new());
    let is_loading = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let mut show_add_modal = use_signal(|| false);

    // Form state for new event
    let mut new_event_type = use_signal(|| TimelineEventType::NoteAdded);
    let mut new_event_title = use_signal(|| String::new());
    let mut new_event_description = use_signal(|| String::new());
    let is_saving = use_signal(|| false);

    // Load timeline events
    use_effect(move || {
        let mut timeline_events = timeline_events.clone();
        let mut is_loading = is_loading.clone();
        let mut error_message = error_message.clone();
        let client_id = id;

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

    // Function to save new timeline event
    let save_timeline_event = move |_| {
        let mut timeline_events = timeline_events.clone();
        let mut error_message = error_message.clone();
        let mut show_add_modal = show_add_modal.clone();
        let mut is_saving = is_saving.clone();
        let client_id = id;
        let event_type = new_event_type();
        let title = new_event_title().clone();
        let description = new_event_description().clone();

        if title.trim().is_empty() {
            error_message.set(Some("Title is required".to_string()));
            return;
        }

        spawn(async move {
            is_saving.set(true);
            error_message.set(None);

            let new_event = TimelineEvent {
                id: 0, // Will be set by database
                borrower_id: client_id,
                event_type,
                title,
                description: if description.trim().is_empty() { None } else { Some(description) },
                metadata: None,
                user_id: Some("current_user".to_string()), // TODO: Get from auth context
                created_at: Utc::now(),
            };

            match repository::Repository::new().await {
                Ok(repo) => {
                    match repo.create_timeline_event(new_event).await {
                        Ok(_) => {
                            // Reload timeline events
                            match repo.get_timeline_events(client_id).await {
                                Ok(events) => {
                                    timeline_events.set(events);
                                    show_add_modal.set(false);
                                    new_event_title.set(String::new());
                                    new_event_description.set(String::new());
                                    new_event_type.set(TimelineEventType::NoteAdded);
                                }
                                Err(e) => {
                                    error_message.set(Some(format!("Failed to reload timeline: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Failed to save timeline event: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to connect to database: {}", e)));
                }
            }
            is_saving.set(false);
        });
    };

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

            // Add new event button
            div { class: "mt-6 pt-6 border-t border-gray-200 dark:border-gray-700",
                button {
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| show_add_modal.set(true),
                    "Add Timeline Event"
                }
            }

            // Add Timeline Event Modal
            if *show_add_modal.read() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-xl max-w-md w-full mx-4",
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4",
                            "Add Timeline Event"
                        }

                        // Event Type Selection
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "Event Type"
                            }
                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-gray-200",
                                value: "{new_event_type():?}",
                                onchange: move |evt| {
                                    let event_type = match evt.value().as_str() {
                                        "NoteAdded" => TimelineEventType::NoteAdded,
                                        "PhoneCall" => TimelineEventType::PhoneCall,
                                        "Meeting" => TimelineEventType::Meeting,
                                        "EmailSent" => TimelineEventType::EmailSent,
                                        "EmailReceived" => TimelineEventType::EmailReceived,
                                        "DocumentRequest" => TimelineEventType::DocumentRequest,
                                        "DocumentUploaded" => TimelineEventType::DocumentUploaded,
                                        "StatusChange" => TimelineEventType::StatusChange,
                                        "TaskCompleted" => TimelineEventType::TaskCompleted,
                                        "ApplicationSubmitted" => TimelineEventType::ApplicationSubmitted,
                                        "LoanCalculated" => TimelineEventType::LoanCalculated,
                                        "Milestone" => TimelineEventType::Milestone,
                                        "Other" => TimelineEventType::Other,
                                        _ => TimelineEventType::NoteAdded, // Default fallback
                                    };
                                    new_event_type.set(event_type);
                                },
                                option { value: "NoteAdded", "Note Added" }
                                option { value: "PhoneCall", "Phone Call" }
                                option { value: "Meeting", "Meeting" }
                                option { value: "EmailSent", "Email Sent" }
                                option { value: "EmailReceived", "Email Received" }
                                option { value: "DocumentRequest", "Document Request" }
                                option { value: "DocumentUploaded", "Document Uploaded" }
                                option { value: "StatusChange", "Status Change" }
                                option { value: "TaskCompleted", "Task Completed" }
                                option { value: "ApplicationSubmitted", "Application Submitted" }
                                option { value: "LoanCalculated", "Loan Calculated" }
                                option { value: "Milestone", "Milestone" }
                                option { value: "Other", "Other" }
                            }
                        }

                        // Title Input
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "Title *"
                            }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-gray-200",
                                r#type: "text",
                                placeholder: "Enter event title",
                                value: "{new_event_title()}",
                                oninput: move |evt| new_event_title.set(evt.value()),
                            }
                        }

                        // Description Input
                        div { class: "mb-6",
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "Description (Optional)"
                            }
                            textarea {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-gray-200",
                                rows: "3",
                                placeholder: "Enter event description",
                                value: "{new_event_description()}",
                                oninput: move |evt| new_event_description.set(evt.value()),
                            }
                        }

                        // Buttons
                        div { class: "flex justify-end space-x-3",
                            button {
                                class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                                onclick: move |_| {
                                    show_add_modal.set(false);
                                    new_event_title.set(String::new());
                                    new_event_description.set(String::new());
                                    new_event_type.set(TimelineEventType::NoteAdded);
                                    error_message.set(None);
                                },
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: *is_saving.read() || new_event_title().trim().is_empty(),
                                onclick: save_timeline_event,
                                if *is_saving.read() {
                                    "Saving..."
                                } else {
                                    "Save Event"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}