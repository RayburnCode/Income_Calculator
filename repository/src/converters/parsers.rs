//! String parsers for enum types

use shared::models::*;

pub fn parse_property_type(s: &str) -> PropertyType {
    match s {
        "SFR" => PropertyType::SFR,
        "Manufactured" => PropertyType::Manufactured,
        "MultiUnit" => PropertyType::MultiUnit,
        "Condo" => PropertyType::Condo,
        "PUD" => PropertyType::PUD,
        _ => PropertyType::SFR,
    }
}

pub fn parse_occupancy_type(s: &str) -> OccupancyType {
    match s {
        "Primary" => OccupancyType::Primary,
        "Secondary" => OccupancyType::Secondary,
        "Investment" => OccupancyType::Investment,
        _ => OccupancyType::Primary,
    }
}

pub fn parse_loan_type(s: &str) -> LoanType {
    match s {
        "CNV" => LoanType::CNV,
        "FHA" => LoanType::FHA,
        "VA" => LoanType::VA,
        "NonQM" => LoanType::NonQM,
        _ => LoanType::CNV,
    }
}

pub fn parse_loan_purpose(s: &str) -> LoanPurpose {
    match s {
        "Purchase" => LoanPurpose::Purchase,
        "CashOut" => LoanPurpose::CashOut,
        "Refinance" => LoanPurpose::Refinance,
        "IRRRLStreamline" => LoanPurpose::IRRRLStreamline,
        _ => LoanPurpose::Refinance,
    }
}

pub fn parse_credit_type(s: &str) -> CreditType {
    match s {
        "Installment" => CreditType::Installment,
        "Mortgage" => CreditType::Mortgage,
        "Revolving" => CreditType::Revolving,
        "Lease" => CreditType::Lease,
        _ => CreditType::Installment,
    }
}

pub fn parse_status(s: &str) -> Status {
    match s {
        "Active" => Status::Active,
        "Inactive" => Status::Inactive,
        "Pending" => Status::Pending,
        _ => Status::Active,
    }
}

pub fn parse_condition_type(s: &str) -> ConditionType {
    match s {
        "Medical" => ConditionType::Medical,
        "Financial" => ConditionType::Financial,
        "Legal" => ConditionType::Legal,
        "Employment" => ConditionType::Employment,
        "Housing" => ConditionType::Housing,
        "Other" => ConditionType::Other,
        _ => ConditionType::Other,
    }
}

pub fn parse_condition_severity(s: &str) -> ConditionSeverity {
    match s {
        "Low" => ConditionSeverity::Low,
        "Medium" => ConditionSeverity::Medium,
        "High" => ConditionSeverity::High,
        "Critical" => ConditionSeverity::Critical,
        _ => ConditionSeverity::Medium,
    }
}

pub fn parse_condition_status(s: &str) -> ConditionStatus {
    match s {
        "Active" => ConditionStatus::Active,
        "Resolved" => ConditionStatus::Resolved,
        "Monitoring" => ConditionStatus::Monitoring,
        "Inactive" => ConditionStatus::Inactive,
        _ => ConditionStatus::Active,
    }
}
