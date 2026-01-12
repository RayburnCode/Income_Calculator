//! Template engine for variable replacement and processing

use shared::models::Borrower;
use chrono::Utc;

/// Template engine for processing templates with client data
pub struct TemplateEngine;

impl TemplateEngine {
    /// Replace template variables with borrower data
    pub fn replace_variables(content: &str, borrower: &Borrower) -> String {
        let mut result = content.to_string();

        // Basic borrower information
        result = result.replace("{{borrower_name}}", &borrower.name);
        result = result.replace("{{borrower_email}}", &borrower.email.as_deref().unwrap_or(""));
        result = result.replace("{{borrower_phone}}", &borrower.phone_number.as_deref().unwrap_or(""));
        result = result.replace("{{loan_number}}", &borrower.loan_number.as_deref().unwrap_or(""));
        result = result.replace("{{employer_name}}", &borrower.employer_name.as_deref().unwrap_or(""));
        result = result.replace("{{income_type}}", &borrower.income_type.as_deref().unwrap_or(""));

        // Address information
        result = result.replace("{{address}}", &borrower.address.as_deref().unwrap_or(""));
        result = result.replace("{{city}}", &borrower.city.as_deref().unwrap_or(""));
        result = result.replace("{{state}}", &borrower.state.as_deref().unwrap_or(""));
        result = result.replace("{{zip_code}}", &borrower.zip_code.as_deref().unwrap_or(""));

        // Status
        result = result.replace("{{status}}", &format!("{:?}", borrower.status.as_ref().unwrap_or(&shared::models::Status::Active)));

        // Date formatting
        if let Some(dob) = borrower.date_of_birth {
            result = result.replace("{{date_of_birth}}", &dob.format("%B %d, %Y").to_string());
        } else {
            result = result.replace("{{date_of_birth}}", "");
        }

        // Current date
        result = result.replace("{{current_date}}", &Utc::now().format("%B %d, %Y").to_string());
        result = result.replace("{{current_date_short}}", &Utc::now().format("%m/%d/%Y").to_string());

        result
    }

    /// Validate that a template has all required variables for a borrower
    pub fn validate_template(content: &str, borrower: &Borrower) -> Vec<String> {
        let mut missing_vars = Vec::new();

        // Check for required variables that should be filled
        if content.contains("{{borrower_name}}") && borrower.name.trim().is_empty() {
            missing_vars.push("borrower_name".to_string());
        }

        if content.contains("{{borrower_email}}") && borrower.email.as_ref().map_or(true, |e| e.trim().is_empty()) {
            missing_vars.push("borrower_email".to_string());
        }

        if content.contains("{{loan_number}}") && borrower.loan_number.as_ref().map_or(true, |ln| ln.trim().is_empty()) {
            missing_vars.push("loan_number".to_string());
        }

        missing_vars
    }

    /// Get all available template variables
    pub fn get_available_variables() -> Vec<(&'static str, &'static str)> {
        vec![
            ("borrower_name", "Client's full name"),
            ("borrower_email", "Client's email address"),
            ("borrower_phone", "Client's phone number"),
            ("loan_number", "Associated loan number"),
            ("employer_name", "Client's employer"),
            ("income_type", "Type of income"),
            ("address", "Street address"),
            ("city", "City"),
            ("state", "State"),
            ("zip_code", "ZIP code"),
            ("status", "Current client status"),
            ("date_of_birth", "Client's date of birth (formatted)"),
            ("current_date", "Today's date (long format)"),
            ("current_date_short", "Today's date (MM/DD/YYYY)"),
        ]
    }
}