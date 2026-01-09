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
        date_of_birth: model.date_of_birth,
        social_security_number: model.social_security_number.clone(),
        address: model.address.clone(),
        city: model.city.clone(),
        state: model.state.clone(),
        zip_code: model.zip_code.clone(),
        mailing_address_different: model.mailing_address_different,
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

/// Convert database timeline_events model to domain model
pub fn timeline_event_to_domain(model: &timeline_events::Model) -> TimelineEvent {
    TimelineEvent {
        id: model.id,
        borrower_id: model.borrower_id,
        event_type: serde_json::from_str(&model.event_type).unwrap_or(TimelineEventType::Other),
        title: model.title.clone(),
        description: model.description.clone(),
        metadata: model.metadata.clone(),
        user_id: model.user_id.clone(),
        created_at: model.created_at,
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

/// Convert database outreach_templates model to domain model
pub fn outreach_template_to_domain(model: &outreach_templates::Model) -> OutreachTemplate {
    OutreachTemplate {
        id: model.id,
        name: model.name.clone(),
        template_type: serde_json::from_str(&model.template_type).unwrap_or(TemplateType::Other),
        subject: model.subject.clone(),
        content: model.content.clone(),
        description: model.description.clone(),
        is_default: model.is_default,
        is_active: model.is_active,
        created_by: model.created_by.clone(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Convert database campaigns model to domain model
pub fn campaign_to_domain(model: &campaigns::Model) -> Campaign {
    Campaign {
        id: model.id,
        name: model.name.clone(),
        description: model.description.clone(),
        campaign_type: serde_json::from_str(&model.campaign_type).unwrap_or(CampaignType::Email),
        template_id: model.template_id,
        segment_criteria: model.segment_criteria.clone(),
        status: serde_json::from_str(&model.status).unwrap_or(CampaignStatus::Draft),
        scheduled_date: model.scheduled_date,
        completed_date: model.completed_date,
        target_audience_count: model.target_audience_count,
        sent_count: model.sent_count,
        opened_count: model.opened_count,
        clicked_count: model.clicked_count,
        converted_count: model.converted_count,
        created_by: model.created_by.clone(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Convert database ab_tests model to domain model
pub fn ab_test_to_domain(model: &ab_tests::Model) -> ABTest {
    ABTest {
        id: model.id,
        campaign_id: model.campaign_id,
        test_name: model.test_name.clone(),
        subject_a: model.subject_a.clone(),
        subject_b: model.subject_b.clone(),
        winner: model.winner.clone(),
        sent_a: model.sent_a,
        sent_b: model.sent_b,
        opened_a: model.opened_a,
        opened_b: model.opened_b,
        clicked_a: model.clicked_a,
        clicked_b: model.clicked_b,
        created_at: model.created_at,
    }
}
