use dioxus::prelude::*;
use shared::models::document::{
    is_allowed_file_type, get_file_extension, format_file_size, MAX_FILE_SIZE, ALLOWED_FILE_TYPES,
};

#[derive(Clone, Debug)]
pub struct SelectedFile {
    pub name: String,
    pub size: Option<i64>,
    pub file_type: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct FileUploadProps {
    /// Optional label text (defaults to "Upload file")
    #[props(default = "Upload file".to_string())]
    pub label: String,
    
    /// Optional help text (defaults to showing allowed file types)
    #[props(default = None)]
    pub help_text: Option<String>,
    
    /// Callback when a file is selected with validation passed
    pub on_file_selected: EventHandler<SelectedFile>,
    
    /// Whether to show file preview after selection
    #[props(default = true)]
    pub show_preview: bool,
    
    /// Accept attribute for file input (defaults to allowed types)
    #[props(default = None)]
    pub accept: Option<String>,
    
    /// Whether to allow clearing the selected file
    #[props(default = true)]
    pub allow_clear: bool,
}

#[component]
pub fn FileUpload(props: FileUploadProps) -> Element {
    let mut selected_file = use_signal::<Option<SelectedFile>>(|| None);
    let mut error_message = use_signal::<Option<String>>(|| None);
    
    let file_input_id = use_hook(|| format!("file_input_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    
    // Generate accept attribute from allowed file types if not provided
    let accept_attr = props.accept.clone().unwrap_or_else(|| {
        ALLOWED_FILE_TYPES
            .iter()
            .map(|(ext, _)| format!(".{}", ext))
            .collect::<Vec<_>>()
            .join(",")
    });
    
    let help_text = props.help_text.clone().unwrap_or_else(|| {
        let extensions: Vec<&str> = ALLOWED_FILE_TYPES.iter().map(|(ext, _)| *ext).collect();
        format!("Allowed: {}. Max size: {}", 
            extensions.join(", ").to_uppercase(),
            format_file_size(MAX_FILE_SIZE)
        )
    });
    
    let on_file_change = move |evt: Event<FormData>| {
        let files = evt.files();
        
        if let Some(file_data) = files.first() {
            let file_name = file_data.name();
            
            // Validate file type
            if !is_allowed_file_type(&file_name) {
                error_message.set(Some(format!(
                    "File type not allowed. Please upload: {}",
                    ALLOWED_FILE_TYPES.iter()
                        .map(|(ext, _)| ext.to_uppercase())
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
                selected_file.set(None);
                return;
            }
            
            // Clear any previous errors
            error_message.set(None);
            
            // Create file info
            let file_info = SelectedFile {
                name: file_name.clone(),
                size: None, // Size not easily available in cross-platform way
                file_type: get_file_extension(&file_name)
                    .unwrap_or_else(|| "unknown".to_string()),
            };
            
            selected_file.set(Some(file_info.clone()));
            props.on_file_selected.call(file_info);
        }
    };
    
    let on_clear = move |_| {
        selected_file.set(None);
        error_message.set(None);
    };
    
    rsx! {
        div { class: "w-full",

            // Label
            label {
                class: "block mb-2.5 text-sm font-medium text-heading",
                r#for: "{file_input_id}",
                "{props.label}"
            }

            // File Input
            input {
                id: "{file_input_id}",
                r#type: "file",
                accept: "{accept_attr}",
                onchange: on_file_change,
                class: "cursor-pointer bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full shadow-xs placeholder:text-body",
                aria_describedby: "{file_input_id}_help",
            }

            // Help Text
            p {
                class: "mt-1 text-sm text-gray-500 dark:text-gray-300",
                id: "{file_input_id}_help",
                "{help_text}"
            }

            // Error Message
            if let Some(error) = error_message() {
                div {
                    class: "mt-2 p-3 bg-red-50 border border-red-200 rounded-base text-sm text-red-700",
                    role: "alert",
                    "{error}"
                }
            }

            // File Preview
            if props.show_preview {
                if let Some(file_info) = selected_file() {
                    div { class: "mt-4 p-4 bg-gray-50 border border-gray-200 rounded-base",

                        h4 { class: "text-sm font-medium text-heading mb-2", "Selected File" }

                        div { class: "space-y-1 text-sm text-body",

                            p {
                                span { class: "font-medium", "Name: " }
                                "{file_info.name}"
                            }

                            if let Some(size) = file_info.size {
                                p {
                                    span { class: "font-medium", "Size: " }
                                    "{format_file_size(size)}"
                                }
                            }

                            p {
                                span { class: "font-medium", "Type: " }
                                "{file_info.file_type.to_uppercase()}"
                            }
                        }

                        if props.allow_clear {
                            div { class: "mt-4",
                                button {
                                    r#type: "button",
                                    onclick: on_clear,
                                    class: "px-4 py-2 bg-gray-200 text-gray-700 rounded-base hover:bg-gray-300 focus:ring-2 focus:ring-gray-400 focus:outline-none text-sm font-medium",
                                    "Clear"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}