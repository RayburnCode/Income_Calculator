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