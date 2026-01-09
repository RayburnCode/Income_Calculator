use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use crate::models::*;
use super::enums::Status;

// Timeline Event Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimelineEventType {
    ClientCreated,
    EmailSent,
    EmailReceived,
    PhoneCall,
    Meeting,
    DocumentRequest,
    DocumentUploaded,
    StatusChange,
    NoteAdded,
    TaskCompleted,
    ApplicationSubmitted,
    LoanCalculated,
    Milestone,
    Other,
}

impl Default for TimelineEventType {
    fn default() -> Self {
        TimelineEventType::Other
    }
}

// Template Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    Email,
    Letter,
    DocumentRequest,
    StatusUpdate,
    WelcomeMessage,
    FollowUp,
    Other,
}

impl Default for TemplateType {
    fn default() -> Self {
        TemplateType::Other
    }
}

// Campaign Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Running,
    Paused,
    Completed,
    Cancelled,
}

impl Default for CampaignStatus {
    fn default() -> Self {
        CampaignStatus::Draft
    }
}

// Campaign Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignType {
    Email,
    SMS,
    DirectMail,
}

impl Default for CampaignType {
    fn default() -> Self {
        CampaignType::Email
    }
}

// Segmentation Criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SegmentationCriteria {
    Status,
    LoanType,
    PropertyType,
    DateRange,
    Custom,
}

impl Default for SegmentationCriteria {
    fn default() -> Self {
        SegmentationCriteria::Status
    }
}

// Campaign Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Campaign {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: CampaignType,
    pub template_id: i32, // Reference to outreach template
    pub segment_criteria: serde_json::Value, // JSON object defining segmentation rules
    pub status: CampaignStatus,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub completed_date: Option<DateTime<Utc>>,
    pub target_audience_count: Option<i32>,
    pub sent_count: i32,
    pub opened_count: i32,
    pub clicked_count: i32,
    pub converted_count: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Campaign {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            name: String::new(),
            description: None,
            campaign_type: CampaignType::Email,
            template_id: 0,
            segment_criteria: serde_json::Value::Object(serde_json::Map::new()),
            status: CampaignStatus::Draft,
            scheduled_date: None,
            completed_date: None,
            target_audience_count: None,
            sent_count: 0,
            opened_count: 0,
            clicked_count: 0,
            converted_count: 0,
            created_by: String::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

// A/B Test Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ABTest {
    pub id: i32,
    pub campaign_id: i32,
    pub test_name: String,
    pub subject_a: String,
    pub subject_b: String,
    pub winner: Option<String>, // "A" or "B"
    pub sent_a: i32,
    pub sent_b: i32,
    pub opened_a: i32,
    pub opened_b: i32,
    pub clicked_a: i32,
    pub clicked_b: i32,
    pub created_at: DateTime<Utc>,
}

impl Default for ABTest {
    fn default() -> Self {
        Self {
            id: 0,
            campaign_id: 0,
            test_name: String::new(),
            subject_a: String::new(),
            subject_b: String::new(),
            winner: None,
            sent_a: 0,
            sent_b: 0,
            opened_a: 0,
            opened_b: 0,
            clicked_a: 0,
            clicked_b: 0,
            created_at: Utc::now(),
        }
    }
}

// Campaign Analytics Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignAnalytics {
    pub campaign_id: i32,
    pub total_sent: i32,
    pub total_opened: i32,
    pub total_clicked: i32,
    pub total_converted: i32,
    pub open_rate: f64,
    pub click_rate: f64,
    pub conversion_rate: f64,
    pub bounce_rate: f64,
    pub unsubscribe_rate: f64,
}

impl Default for CampaignAnalytics {
    fn default() -> Self {
        Self {
            campaign_id: 0,
            total_sent: 0,
            total_opened: 0,
            total_clicked: 0,
            total_converted: 0,
            open_rate: 0.0,
            click_rate: 0.0,
            conversion_rate: 0.0,
            bounce_rate: 0.0,
            unsubscribe_rate: 0.0,
        }
    }
}

// Outreach Template Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutreachTemplate {
    pub id: i32,
    pub name: String,
    pub template_type: TemplateType,
    pub subject: Option<String>, // For emails
    pub content: String,
    pub description: Option<String>,
    pub is_default: bool, // Whether this is a system-provided template
    pub is_active: bool,
    pub created_by: Option<String>, // User ID who created it, None for system templates
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for OutreachTemplate {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            name: String::new(),
            template_type: TemplateType::Other,
            subject: None,
            content: String::new(),
            description: None,
            is_default: false,
            is_active: true,
            created_by: None,
            created_at: now,
            updated_at: now,
        }
    }
}

// Timeline Event Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEvent {
    pub id: i32,
    pub borrower_id: i32,
    pub event_type: TimelineEventType,
    pub title: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>, // For storing additional event-specific data
    pub user_id: Option<String>, // Who performed the action
    pub created_at: DateTime<Utc>,
}

impl Default for TimelineEvent {
    fn default() -> Self {
        Self {
            id: 0,
            borrower_id: 0,
            event_type: TimelineEventType::Other,
            title: String::new(),
            description: None,
            metadata: None,
            user_id: None,
            created_at: Utc::now(),
        }
    }
}

// Borrower Information Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Borrower {
    pub id: i32,
    pub name: String,
    pub employer_name: Option<String>,
    pub income_type: Option<String>,
    pub loan_number: Option<String>,
    pub status: Option<Status>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub social_security_number: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub mailing_address_different: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Borrower {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            name: String::new(),
            employer_name: None,
            income_type: None,
            loan_number: None,
            status: Some(Status::Active),
            email: None,
            phone_number: None,
            date_of_birth: None,
            social_security_number: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            mailing_address_different: Some(false),
            created_at: now,
            updated_at: now,
        }
    }
}

// Main Mortgage Refinance Options Template Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MortgageRefinanceOptions {
    pub id: Uuid,
    pub borrower_id: i32, // Reference to borrower/client

    // Core information
    pub loan_information: LoanInformation,
    pub new_loan_details: NewLoanDetails,

    // Existing loans to be paid off
    pub existing_loans: Vec<ExistingLoan>,

    // Comparison data
    pub benefit_to_borrower: BenefitToBorrower,

    // Financial details
    pub other_fees: OtherFees,
    pub pricing_options: Vec<PricingOption>,
    pub consumer_debts: Vec<ConsumerDebt>,
    pub income_information: IncomeInformation,

    // Calculations
    pub savings_calculation: SavingsCalculation,

    // Metadata
    pub status: String, // draft, submitted, approved, etc.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

impl Default for MortgageRefinanceOptions {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            borrower_id: 0, // Should be set when creating
            loan_information: LoanInformation::default(),
            new_loan_details: NewLoanDetails::default(),
            existing_loans: vec![ExistingLoan::default()],
            benefit_to_borrower: BenefitToBorrower::default(),
            other_fees: OtherFees::default(),
            pricing_options: vec![
                PricingOption::default(),
                PricingOption::default(),
                PricingOption::default(),
            ],
            consumer_debts: Vec::new(),
            income_information: IncomeInformation::default(),
            savings_calculation: SavingsCalculation::default(),
            status: "draft".to_string(),
            created_at: now,
            updated_at: now,
            submitted_at: None,
        }
    }
}