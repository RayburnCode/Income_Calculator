use dioxus::prelude::*;
use crate::get_repository;
use crate::components::{FileUpload, SelectedFile};
use shared::models::document::UploadDocumentRequest;

#[component]
pub fn UploadDocuments(id: i32) -> Element {
    let _client_data = use_resource(move || async move {
        let repo = get_repository();
        repo.get_borrower(id).await.map_err(|e| e.to_string())
    });
    let mut upload_status = use_signal::<Option<String>>(|| None);
    let mut is_uploading = use_signal(|| false);
    let mut documents: Resource<Result<Vec<shared::models::document::Document>, String>> = 
        use_resource(move || async move { 
            let repo = get_repository();
            repo.get_documents_by_client(id).await.map_err(|e| e.to_string())
        });

    rsx! {
        p { "Upload Documents View - Under Construction" }

        // Upload Section
        div { class: "mb-6 p-4 bg-neutral-secondary-soft border border-default rounded-base",
            h3 { class: "text-lg font-semibold mb-4 text-heading", "Upload New Document" }

            FileUpload {
                label: "Select Document".to_string(),
                on_file_selected: move |file: SelectedFile| {
                    let client_id = id;
                    spawn(async move {
                        is_uploading.set(true);
                        upload_status.set(Some("Reading file...".to_string()));

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
                                upload_status.set(Some("Upload successful!".to_string()));
                                documents.restart();
                                let mut upload_status_clone = upload_status.clone();
                                spawn(async move {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                    upload_status_clone.set(None);
                                });
                            }
                            Err(e) => {
                                upload_status.set(Some(format!("Upload failed: {}", e)));
                            }
                        }
                        is_uploading.set(false);
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