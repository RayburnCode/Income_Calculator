// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::get_repository;
use shared::models::document::format_file_size;
use crate::components::{FileUpload, SelectedFile, Accordion};
use crate::components::accordion::AccordionItem;
use shared::models::document::UploadDocumentRequest;
use std::collections::HashMap;


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
    let mut show_upload_modal = use_signal(|| false);
    let  upload_status = use_signal::<Option<String>>(|| None);
    let  is_uploading = use_signal(|| false);

    // Function to categorize documents based on filename
    let categorize_document = |filename: &str| -> &'static str {
        let filename_lower = filename.to_lowercase();
        if filename_lower.contains("id") || filename_lower.contains("identification") || filename_lower.contains("license") || filename_lower.contains("passport") || filename_lower.contains("ssn") {
            "Identification"
        } else if filename_lower.contains("income") || filename_lower.contains("pay") || filename_lower.contains("w2") || filename_lower.contains("1099") || filename_lower.contains("salary") || filename_lower.contains("commission") {
            "Income"
        } else if filename_lower.contains("appraisal") || filename_lower.contains("valuation") || filename_lower.contains("property") {
            "Appraisals"
        } else if filename_lower.contains("tax") || filename_lower.contains("return") || filename_lower.contains("1040") {
            "Tax Documents"
        } else if filename_lower.contains("bank") || filename_lower.contains("statement") || filename_lower.contains("account") {
            "Bank Statements"
        } else if filename_lower.contains("credit") || filename_lower.contains("score") {
            "Credit Reports"
        } else if filename_lower.contains("loan") || filename_lower.contains("mortgage") || filename_lower.contains("application") {
            "Loan Documents"
        } else {
            "Other Documents"
        }
    };
    

    rsx! {
        Fragment {
            div { class: "bg-neutral-primary-soft border border-default rounded-base p-6 shadow-xs",
                div { class: "flex justify-between items-center mb-6",
                    h2 { class: "text-2xl font-bold text-heading", "Documents" }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-base hover:bg-blue-700 transition-colors font-medium shadow-sm hover:shadow-md",
                        onclick: move |_| show_upload_modal.set(true),
                        "Upload Document"
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
                                    // Group documents by category
                                    let mut categorized_docs: HashMap<

                                        // Create accordion items

                                        // In a real implementation, you'd trigger a download
                                        // For now, we'll just log the success

                                        &str,
                                        Vec<shared::models::document::Document>,
                                    > = HashMap::new();
                                    for doc in &docs {
                                        let category = categorize_document(&doc.filename);
                                        categorized_docs
                                            .entry(category)
                                            .or_insert_with(Vec::new)
                                            .push(doc.clone());
                                    }
                                    let accordion_items = vec![
                                        (
                                            "Identification",
                                            "ID cards, driver's licenses, passports, SSN documents",
                                        ),
                                        ("Income", "Pay stubs, W-2s, 1099s, salary statements"),
                                        ("Appraisals", "Property appraisals and valuations"),
                                        ("Tax Documents", "Tax returns, 1040 forms"),
                                        ("Bank Statements", "Bank account statements"),
                                        ("Credit Reports", "Credit reports and scores"),
                                        ("Loan Documents", "Loan applications, mortgage documents"),
                                        ("Other Documents", "Miscellaneous documents"),
                                    ]
                                        .into_iter()
                                        .map(|(category, description)| {
                                            let category_docs = categorized_docs
                                                .get(category)
                                                .cloned()
                                                .unwrap_or_default();
                                            AccordionItem {
                                                id: category.to_lowercase().replace(" ", "_"),
                                                title: format!("{} ({})", category, category_docs.len()),
                                                content: if category_docs.is_empty() {
                                                    rsx! {
                                                        div { class: "text-center py-4 text-body text-sm",
                                                            p { "No {category} documents uploaded yet." }
                                                            p { class: "text-xs text-gray-500 mt-1", "{description}" }
                                                        }
                                                    }
                                                } else {
                                                    rsx! {
                                                        div { class: "space-y-2",
                                                            for doc in category_docs {
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
                                                },
                                                initially_open: category == "Identification"
                                                    || category == "Income",
                                                show_checkbox: false,
                                                checkbox_checked: false,
                                                on_checkbox_change: None,
                                            }
                                        })
                                        .collect::<Vec<_>>();
                                    rsx! {
                                        Accordion { items: accordion_items, collapse_others: false }
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

                // Upload Modal
                if show_upload_modal() {
                    div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                        div { class: "bg-white p-6 rounded-lg max-w-md w-full mx-4",
                            div { class: "flex justify-between items-center mb-4",
                                h3 { class: "text-lg font-semibold text-heading",
                                    "Upload New Document"
                                }
                                button {
                                    class: "text-gray-500 hover:text-gray-700 text-xl font-bold",
                                    onclick: move |_| show_upload_modal.set(false),
                                    "×"
                                }
                            }

                            FileUpload {
                                label: "Select Document".to_string(),
                                on_file_selected: move |file: SelectedFile| {
                                    let client_id = id;
                                    let mut upload_status_clone = upload_status.clone();
                                    let mut is_uploading_clone = is_uploading.clone();
                                    let mut documents_clone = documents.clone();
                                    spawn(async move {
                                        is_uploading_clone.set(true);
                                        upload_status_clone.set(Some("Reading file...".to_string()));

                                        // For now, create a placeholder file data
                                        // In a real implementation, you'd need to modify FileUpload to provide file data
                                        let file_data = b"Placeholder file content - actual file reading needs to be implemented"
                                            .to_vec();
                                        let upload_request = UploadDocumentRequest {
                                            client_id,
                                            filename: file.name.clone(),
                                            description: None,
                                            file_data,
                                        };
                                        let repo = get_repository();
                                        match repo.upload_document(upload_request).await {
                                            Ok(_doc) => {
                                                upload_status_clone.set(Some("Upload successful!".to_string()));
                                                documents_clone.restart();
                                                let mut upload_status_reset = upload_status_clone.clone();
                                                spawn(async move {
                                                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                                    upload_status_reset.set(None);
                                                });
                                            }
                                            Err(e) => {
                                                upload_status_clone.set(Some(format!("Upload failed: {}", e)));
                                            }
                                        }
                                        is_uploading_clone.set(false);
                                    });
                                },
                            }

                            // Upload Status Message
                            if let Some(status) = upload_status() {
                                div { class: if status.contains("successful") { "mt-3 p-3 bg-green-50 border border-green-300 text-green-800 rounded-base" } else if status.contains("failed") || status.contains("Failed") { "mt-3 p-3 bg-red-50 border border-red-300 text-red-800 rounded-base" } else { "mt-3 p-3 bg-blue-50 border border-blue-300 text-blue-800 rounded-base" },
                                    p { "{status}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }}
