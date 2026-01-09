// Note-related models

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Note Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: i32,
    pub client_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: 0,
            client_id: 0,
            user_id: 1,
            content: String::new(),
            created_at: Utc::now(),
        }
    }
}

// Create Note Request (for API/frontend)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateNoteRequest {
    pub client_id: i32,
    pub user_id: i32,
    pub content: String,
}