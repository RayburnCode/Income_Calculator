use serde::{Deserialize, Serialize};
use validator::Validate; 

/// Maximum file size: 10MB
pub const MAX_FILE_SIZE: i64 = 10 * 1024 * 1024;

/// Allowed file types with MIME types
pub const ALLOWED_FILE_TYPES: &[(&str, &[&str])] = &[
    ("pdf", &["application/pdf"]),
    ("doc", &["application/msword"]),
    ("docx", &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]),
    ("xls", &["application/vnd.ms-excel"]),
    ("xlsx", &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]),
    ("txt", &["text/plain"]),
    ("jpg", &["image/jpeg"]),
    ("jpeg", &["image/jpeg"]),
    ("png", &["image/png"]),
    ("gif", &["image/gif"]),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct Document {
    pub id: i32,
    pub client_id: i32,
    pub filename: String,
    pub file_size: i64,
    pub file_type: String,
    pub file_path: String,
    pub mime_type: String,
    pub upload_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>, // SHA-256 hash for integrity verification
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UploadDocumentRequest {
    pub client_id: i32,
    
    #[validate(length(min = 1, max = 255, message = "Filename must be between 1-255 characters"))]
    #[validate(custom(function = "validate_filename"))]
    pub filename: String,
    
    #[validate(length(max = 500, message = "Description too long"))]
    pub description: Option<String>,
    
    #[validate(custom(function = "validate_file_size"))]
    pub file_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDocumentRequest {
    #[validate(length(min = 1, max = 255, message = "Filename must be between 1-255 characters"))]
    #[validate(custom(function = "validate_filename"))]
    pub filename: Option<String>,
    
    #[validate(length(max = 500, message = "Description too long"))]
    pub description: Option<String>,
}

/// Validate filename doesn't contain dangerous characters
fn validate_filename(filename: &str) -> Result<(), validator::ValidationError> {
    // Check for dangerous characters
    let dangerous_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
    if filename.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(validator::ValidationError::new("invalid_filename"));
    }
    
    // Check for path traversal attempts
    if filename.contains("..") {
        return Err(validator::ValidationError::new("path_traversal_attempt"));
    }
    
    // Must have an extension
    if !filename.contains('.') {
        return Err(validator::ValidationError::new("no_file_extension"));
    }
    
    Ok(())
}

/// Validate file size is within limits
fn validate_file_size(file_data: &[u8]) -> Result<(), validator::ValidationError> {
    if file_data.is_empty() {
        return Err(validator::ValidationError::new("empty_file"));
    }
    
    if file_data.len() as i64 > MAX_FILE_SIZE {
        return Err(validator::ValidationError::new("file_too_large"));
    }
    
    Ok(())
}

/// Helper function to extract file extension
pub fn get_file_extension(filename: &str) -> Option<String> {
    filename
        .rsplit('.')
        .next()
        .map(|ext| ext.to_lowercase())
}

/// Check if file type is allowed based on extension
pub fn is_allowed_file_type(filename: &str) -> bool {
    if let Some(ext) = get_file_extension(filename) {
        ALLOWED_FILE_TYPES.iter().any(|(allowed_ext, _)| *allowed_ext == ext)
    } else {
        false
    }
}

/// Get MIME type for file extension
pub fn get_mime_type(filename: &str) -> Option<String> {
    if let Some(ext) = get_file_extension(filename) {
        ALLOWED_FILE_TYPES
            .iter()
            .find(|(allowed_ext, _)| *allowed_ext == ext)
            .and_then(|(_, mimes)| mimes.first())
            .map(|mime| mime.to_string())
    } else {
        None
    }
}

/// Format file size for display
pub fn format_file_size(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_filename() {
        assert!(validate_filename("document.pdf").is_ok());
        assert!(validate_filename("my_file.docx").is_ok());
        assert!(validate_filename("report-2024.xlsx").is_ok());
        
        assert!(validate_filename("../etc/passwd").is_err());
        assert!(validate_filename("file/with/slash.pdf").is_err());
        assert!(validate_filename("file:name.pdf").is_err());
        assert!(validate_filename("noextension").is_err());
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(get_file_extension("test.pdf"), Some("pdf".to_string()));
        assert_eq!(get_file_extension("test.DOCX"), Some("docx".to_string()));
        assert_eq!(get_file_extension("noext"), None);
    }

    #[test]
    fn test_is_allowed_file_type() {
        assert!(is_allowed_file_type("document.pdf"));
        assert!(is_allowed_file_type("image.PNG"));
        assert!(!is_allowed_file_type("script.exe"));
        assert!(!is_allowed_file_type("file.zip"));
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 bytes");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
    }
}