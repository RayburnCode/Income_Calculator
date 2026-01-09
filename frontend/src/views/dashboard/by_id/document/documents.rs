// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::routes::Route;
use crate::get_repository;
use shared::models::document::format_file_size;


#[component]
pub fn ClientDocuments(id: i32) -> Element {
    let _client_data = use_resource(move || async move {
        let repo = get_repository();
        repo.get_borrower(id).await
    });
    let mut documents: Resource<Result<Vec<shared::models::document::Document>, String>> = 
        use_resource(move || async move { 
            let repo = get_repository();
            repo.get_documents_by_client(id).await.map_err(|e| e.to_string())
        });
    

    rsx! {
        div { class: "bg-neutral-primary-soft border border-default rounded-base p-6 shadow-xs",
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-2xl font-bold text-heading", "Documents" }
                Link {
                    class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium shadow-sm hover:shadow-md",
                    to: Route::UploadDocuments { id },
                    "Upload Document"
                }
            }

            // Documents List Section
            div {
                h3 { class: "text-lg font-semibold mb-3 text-heading", "Uploaded Documents" }

                match &*documents.read_unchecked() {
                    Some(Ok(doc_list)) => {
                        let docs = doc_list.clone();

                        if docs.is_empty() {
                            rsx! {
                                div { class: "text-center py-8 text-body",
                                    p { "No documents uploaded yet." }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-2",
                                    for doc in docs {
                                        div {
                                            key: "{doc.id}",
                                            class: "flex items-center justify-between p-3 bg-neutral-secondary-soft border border-default rounded hover:bg-neutral-secondary-medium transition-colors",
                                            div { class: "flex items-center gap-3",
                                                span { class: "text-heading font-medium", "{doc.filename}" }
                                                span { class: "text-body text-sm", "{format_file_size(doc.file_size)}" }
                                                span { class: "text-body text-xs", "{doc.upload_date}" }
                                            }
                                            div { class: "flex gap-2",
                                                button {
                                                    class: "text-fg-brand hover:underline text-sm font-medium",
                                                    onclick: move |_| {
                                                        let repo = get_repository();
                                                        let doc_id = doc.id;
                                                        spawn(async move {
                                                            if let Ok(Some((doc, _data))) = repo.download_document(doc_id).await {
                                                                // In a real implementation, you'd trigger a download
                                                                // For now, we'll just log the success
                                                                web_sys::console::log_1(
                                                                    &format!("Downloaded document: {}", doc.filename).into(),
                                                                );
                                                            }
                                                        });
                                                    },
                                                    "Download"
                                                }
                                                button {
                                                    class: "text-red-600 hover:underline text-sm font-medium",
                                                    onclick: move |_| {
                                                        let repo = get_repository();
                                                        let doc_id = doc.id;
                                                        spawn(async move {
                                                            if let Ok(_) = repo.delete_document(doc_id).await {
                                                                documents.restart();
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
                    Some(Err(e)) => rsx! {
                        div { class: "bg-red-50 border border-red-300 text-red-800 px-4 py-3 rounded-base",
                            p { "Error loading documents: {e}" }
                        }
                    },
                    None => rsx! {
                        div { class: "text-center py-8",
                            p { class: "text-body", "Loading documents..." }
                        }
                    },
                }
            }
        }
    }
}
