use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    pub id: i32,
    pub client_id: i32,
    pub title: String,
    pub description: String,
    pub condition_type: ConditionType,
    pub severity: ConditionSeverity,
    pub status: ConditionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    Medical,
    Financial,
    Legal,
    Employment,
    Housing,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionStatus {
    Active,
    Resolved,
    Monitoring,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateConditionRequest {
    pub client_id: i32,
    pub title: String,
    pub description: String,
    pub condition_type: ConditionType,
    pub severity: ConditionSeverity,
    pub status: ConditionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateConditionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub condition_type: Option<ConditionType>,
    pub severity: Option<ConditionSeverity>,
    pub status: Option<ConditionStatus>,
}