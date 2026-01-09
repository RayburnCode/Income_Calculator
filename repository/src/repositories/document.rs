//! Document repository - handles document CRUD operations

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait, QueryFilter, QueryOrder, ColumnTrait};
use shared::models::*;
use database::entities::document;
use crate::converters::*;
use tokio::fs;
use sha2::{Sha256, Digest};

pub struct DocumentRepository;

impl DocumentRepository {
    pub async fn create(db: &DatabaseConnection, upload_request: UploadDocumentRequest) -> Result<Document, Box<dyn std::error::Error>> {
        // Generate file path and ensure directory exists
        let file_path = Self::generate_file_path(upload_request.client_id, &upload_request.filename)?;

        // Calculate checksum
        let checksum = Self::calculate_checksum(&upload_request.file_data);

        // Write file to disk
        fs::write(&file_path, &upload_request.file_data).await?;

        // Get file extension for file_type
        let file_type = shared::models::document::get_file_extension(&upload_request.filename)
            .unwrap_or_else(|| "unknown".to_string());

        // Get MIME type
        let mime_type = shared::models::document::get_mime_type(&upload_request.filename)
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let active_model = document::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            client_id: Set(upload_request.client_id),
            filename: Set(upload_request.filename),
            file_size: Set(upload_request.file_data.len() as i64),
            file_type: Set(file_type),
            file_path: Set(file_path),
            mime_type: Set(mime_type),
            upload_date: Set(chrono::Utc::now()),
            description: Set(upload_request.description),
            checksum: Set(Some(checksum)),
        };

        let inserted = active_model.insert(db).await?;
        Ok(document_to_domain(&inserted))
    }

    pub async fn get_by_client_id(db: &DatabaseConnection, client_id: i32) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
        let entities = document::Entity::find()
            .filter(document::Column::ClientId.eq(client_id))
            .order_by_desc(document::Column::UploadDate)
            .all(db)
            .await?;
        Ok(entities.iter().map(document_to_domain).collect())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Document>, Box<dyn std::error::Error>> {
        let entity = document::Entity::find_by_id(id).one(db).await?;
        Ok(entity.as_ref().map(document_to_domain))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // First get the document to get the file path
        if let Some(doc_entity) = document::Entity::find_by_id(id).one(db).await? {
            // Delete the file from disk
            if let Err(e) = fs::remove_file(&doc_entity.file_path).await {
                eprintln!("Warning: Failed to delete file {}: {}", doc_entity.file_path, e);
                // Continue with database deletion even if file deletion fails
            }

            // Delete from database
            document::Entity::delete_by_id(id).exec(db).await?;
        }
        Ok(())
    }

    pub async fn get_file_data(db: &DatabaseConnection, id: i32) -> Result<Option<(Document, Vec<u8>)>, Box<dyn std::error::Error>> {
        if let Some(entity) = document::Entity::find_by_id(id).one(db).await? {
            let document = document_to_domain(&entity);
            match fs::read(&entity.file_path).await {
                Ok(data) => Ok(Some((document, data))),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn count_by_client(db: &DatabaseConnection, client_id: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let count = document::Entity::find()
            .filter(document::Column::ClientId.eq(client_id))
            .count(db)
            .await?;
        Ok(count as i64)
    }

    // Helper methods
    fn generate_file_path(client_id: i32, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Get the app data directory
        let proj_dirs = directories::ProjectDirs::from("", "", "Income Calculator")
            .ok_or("Could not determine application data directory")?;

        let documents_dir = proj_dirs.data_dir().join("documents").join(client_id.to_string());

        // Ensure directory exists
        std::fs::create_dir_all(&documents_dir)?;

        // Sanitize filename to prevent path traversal
        let sanitized_filename = Self::sanitize_filename(filename);
        let file_path = documents_dir.join(sanitized_filename);

        Ok(file_path.to_string_lossy().to_string())
    }

    fn sanitize_filename(filename: &str) -> String {
        // Remove any path separators and dangerous characters
        filename
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c => c,
            })
            .collect()
    }

    fn calculate_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}