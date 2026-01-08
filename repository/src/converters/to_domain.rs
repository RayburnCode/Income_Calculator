//! Converters from database entities to domain models

use database::entities::*;
use shared::models::*;
use rust_decimal::prelude::ToPrimitive;

use super::parsers;

/// Convert database borrower model to domain model
pub fn borrower_to_domain(model: &borrower::Model) -> Borrower {
    Borrower {
        id: model.id,
        name: model.name.clone(),
        employer_name: model.employer_name.clone(),
        income_type: model.income_type.clone(),
        loan_number: model.loan_number.clone(),
        status: model.status.as_ref().map(|s| parsers::parse_status(s)),
        email: model.email.clone(),
        phone_number: model.phone_number.clone(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Convert database loan_information model to domain model
pub fn loan_information_to_domain(model: &loan_information::Model) -> LoanInformation {
    LoanInformation {
        id: model.id,
        property_type: parse_property_type(&model.property_type),
        occupancy_type: parse_occupancy_type(&model.occupancy_type),
        loan_type: parse_loan_type(&model.loan_type),
        new_term_months: model.new_term_months,
        loan_purpose: parse_loan_purpose(&model.loan_purpose),
        appraisal_waiver: model.appraisal_waiver,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Convert database income_information model to domain model
pub fn income_information_to_domain(model: &income_information::Model) -> IncomeInformation {
    IncomeInformation {
        id: model.id,
        borrower_monthly_income: model.borrower_monthly_income.to_f64().unwrap_or(0.0),
        coborrower_monthly_income: model.coborrower_monthly_income.to_f64().unwrap_or(0.0),
        front_end_ratio: model.front_end_ratio.to_f64().unwrap_or(0.0),
        back_end_ratio: model.back_end_ratio.to_f64().unwrap_or(0.0),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Convert database w2_jobs model to domain model
pub fn w2_job_to_domain(model: &w2_jobs::Model) -> W2Job {
    W2Job {
        employer_name: model.employer_name.clone(),
        job_title: model.job_title.clone(),
        years_employed: model.years_employed.map(|y| y.to_string()).unwrap_or_default(),
        months_employed: model.months_employed.map(|m| m.to_string()).unwrap_or_default(),
        annual_salary: model.annual_salary.map(|s| s.to_string()).unwrap_or_default(),
        hourly_rate: model.hourly_rate.map(|r| r.to_string()).unwrap_or_default(),
        hours_per_week: model.hours_per_week.map(|h| h.to_string()).unwrap_or_default(),
        commission_monthly: model.commission_monthly.map(|c| c.to_string()).unwrap_or_default(),
        bonus_monthly: model.bonus_monthly.map(|b| b.to_string()).unwrap_or_default(),
        overtime_monthly: model.overtime_monthly.map(|o| o.to_string()).unwrap_or_default(),
    }
}
