use dioxus::prelude::*;
use crate::get_repository;
use shared::models::note::CreateNoteRequest;

#[component] 
pub fn ClientNotes(id: i32) -> Element {
    let _client_data = use_resource(move || async move { 
        let repo = get_repository();
        repo.get_borrower(id).await 
    });
    let mut notes = use_resource(move || async move { 
        let repo = get_repository();
        repo.get_notes_by_client(id).await 
    });
    
    let mut note_content = use_signal(|| String::new());
    let mut show_form = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    let mut success_message = use_signal(|| String::new());

    let handle_submit = move |_| {
        spawn(async move {
            error_message.set(String::new());
            success_message.set(String::new());

            let content = note_content.read().clone();
             
            if content.trim().is_empty() {
                error_message.set("Note content cannot be empty".to_string());
                return;
            }

            if content.len() > 500 {
                error_message.set("Note content must be 500 characters or less".to_string());
                return;
            }

            let new_note = CreateNoteRequest {
                client_id: id,
                user_id: 1,
                content,
            };

            let repo = get_repository();
            match repo.create_note(new_note).await {
                Ok(_) => {
                    success_message.set("Note added successfully!".to_string());
                    note_content.set(String::new());
                    show_form.set(false);
                    notes.restart();
                }
                Err(e) => {
                    error_message.set(format!("Failed to add note: {}", e));
                }
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 dark:bg-gray-900 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-gray-100",
                        "Notes"
                    }
                    p { class: "text-gray-600 dark:text-gray-400 mt-2",
                        "Manage and track client notes and important information"
                    }
                }

                // Main Content
                div { class: "space-y-6",
                    div { class: "flex justify-between items-center",
                        h2 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100",
                            "Notes"
                        }
                        button {
                            class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium shadow-sm hover:shadow-md",
                            onclick: move |_| show_form.set(!show_form()),
                            if show_form() {
                                "Cancel"
                            } else {
                                "Add Note"
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

                    // Add Note Form
                    if show_form() {
                        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6",
                            h3 { class: "text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100",
                                "Add New Note"
                            }

                            textarea {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-base focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 min-h-[100px]",
                                placeholder: "Enter note content (max 500 characters)...",
                                value: "{note_content}",
                                oninput: move |evt| note_content.set(evt.value()),
                                maxlength: 500,
                            }

                            div { class: "flex justify-between items-center mt-4",
                                span { class: "text-sm text-gray-600 dark:text-gray-400",
                                    "{note_content().len()}/500 characters"
                                }
                                button {
                                    class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium",
                                    onclick: handle_submit,
                                    "Save Note"
                                }
                            }
                        }
                    }

                    // Notes List Section
                    div {
                        h3 { class: "text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100",
                            "All Notes"
                        }

                        if let Some(Ok(note_list)) = notes.read().as_ref() {
                            if note_list.is_empty() {
                                div { class: "text-center py-12 text-gray-600 dark:text-gray-400",
                                    p { "No notes added yet." }
                                }
                            } else {
                                div { class: "space-y-4",
                                    for note in note_list.iter().rev() {
                                        div {
                                            key: "{note.id}",
                                            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 hover:shadow-md transition-shadow",

                                            div { class: "flex justify-between items-start mb-3",
                                                div { class: "flex items-center gap-2",
                                                    span { class: "text-xs text-gray-500 dark:text-gray-400",
                                                        "User #{note.user_id}"
                                                    }
                                                    span { class: "text-xs text-gray-500 dark:text-gray-400",
                                                        "•"
                                                    }
                                                    span { class: "text-xs text-gray-500 dark:text-gray-400",
                                                        // Format timestamp to remove seconds
                                                        {note.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                                    }
                                                }
                                                button {
                                                    class: "text-red-600 hover:text-red-800 text-sm font-medium",
                                                    onclick: {
                                                        let note_id = note.id;
                                                        move |_| {
                                                            spawn(async move {
                                                                let repo = get_repository();
                                                                if let Ok(_) = repo.delete_note(note_id).await {
                                                                    notes.restart();
                                                                }
                                                            });
                                                        }
                                                    },
                                                    "Delete"
                                                }
                                            }

                                            div { class: "text-gray-900 dark:text-gray-100 whitespace-pre-wrap",
                                                "{note.content}"
                                            }
                                        }
                                    }
                                }
                            }
                        } else if let Some(Err(e)) = notes.read().as_ref() {
                            div { class: "bg-red-50 border border-red-300 text-red-800 px-4 py-3 rounded-base",
                                p { "Error loading notes: {e}" }
                            }
                        } else {
                            div { class: "text-center py-12",
                                p { class: "text-gray-600 dark:text-gray-400", "Loading notes..." }
                            }
                        }
                    }
                }
            }
        }
    }
}
