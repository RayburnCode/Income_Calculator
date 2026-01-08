use serde::{Deserialize, Serialize};
use std::fmt;

// Enums for dropdown selections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    SFR,
    Manufactured,
    MultiUnit,
    Condo,
    PUD,
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyType::SFR => write!(f, "SFR"),
            PropertyType::Manufactured => write!(f, "Manufactured"),
            PropertyType::MultiUnit => write!(f, "MultiUnit"),
            PropertyType::Condo => write!(f, "Condo"),
            PropertyType::PUD => write!(f, "PUD"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OccupancyType {
    Primary,
    Secondary,
    Investment,
}

impl fmt::Display for OccupancyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OccupancyType::Primary => write!(f, "Primary"),
            OccupancyType::Secondary => write!(f, "Secondary"),
            OccupancyType::Investment => write!(f, "Investment"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoanType {
    CNV,
    FHA,
    VA,
    NonQM,
}

impl fmt::Display for LoanType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoanType::CNV => write!(f, "CNV"),
            LoanType::FHA => write!(f, "FHA"),
            LoanType::VA => write!(f, "VA"),
            LoanType::NonQM => write!(f, "NonQM"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoanPurpose {
    Purchase,
    CashOut,
    Refinance,
    IRRRLStreamline,
}

impl fmt::Display for LoanPurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoanPurpose::Purchase => write!(f, "Purchase"),
            LoanPurpose::CashOut => write!(f, "CashOut"),
            LoanPurpose::Refinance => write!(f, "Refinance"),
            LoanPurpose::IRRRLStreamline => write!(f, "IRRRLStreamline"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreditType {
    Installment,
    Mortgage,
    Revolving,
    Lease,
}

impl std::fmt::Display for CreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditType::Installment => write!(f, "Installment"),
            CreditType::Mortgage => write!(f, "Mortgage"),
            CreditType::Revolving => write!(f, "Revolving"),
            CreditType::Lease => write!(f, "Lease"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Approved,
    Rejected,
    Closed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Inactive => write!(f, "Inactive"),
            Status::Pending => write!(f, "Pending"),
            Status::Approved => write!(f, "Approved"),
            Status::Rejected => write!(f, "Rejected"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}

// Parsing functions for converting strings to enums
pub fn parse_property_type(s: &str) -> PropertyType {
    match s {
        "Manufactured" => PropertyType::Manufactured,
        "MultiUnit" => PropertyType::MultiUnit,
        "Condo" => PropertyType::Condo,
        "PUD" => PropertyType::PUD,
        _ => PropertyType::SFR,
    }
}

pub fn parse_occupancy_type(s: &str) -> OccupancyType {
    match s {
        "Secondary" => OccupancyType::Secondary,
        "Investment" => OccupancyType::Investment,
        _ => OccupancyType::Primary,
    }
}

pub fn parse_loan_type(s: &str) -> LoanType {
    match s {
        "FHA" => LoanType::FHA,
        "VA" => LoanType::VA,
        "NonQM" => LoanType::NonQM,
        _ => LoanType::CNV,
    }
}

pub fn parse_loan_purpose(s: &str) -> LoanPurpose {
    match s {
        "CashOut" => LoanPurpose::CashOut,
        "Refinance" => LoanPurpose::Refinance,
        "IRRRLStreamline" => LoanPurpose::IRRRLStreamline,
        _ => LoanPurpose::Purchase,
    }
}

pub fn parse_status(s: &str) -> Status {
    match s {
        "Inactive" => Status::Inactive,
        "Pending" => Status::Pending,
        "Approved" => Status::Approved,
        "Rejected" => Status::Rejected,
        "Closed" => Status::Closed,
        _ => Status::Active,
    }
}