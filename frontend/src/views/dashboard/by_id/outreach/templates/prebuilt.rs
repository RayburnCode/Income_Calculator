//! Prebuilt templates for common outreach scenarios

use shared::models::{OutreachTemplate, TemplateType};
use chrono::Utc;

/// Collection of prebuilt templates that come with the system
pub struct PrebuiltTemplates;

impl PrebuiltTemplates {
    /// Get all prebuilt templates
    pub fn get_all() -> Vec<OutreachTemplate> {
        vec![
            Self::welcome_email(),
            Self::document_request(),
            Self::status_update(),
            Self::prequalification_results(),
            Self::follow_up_email(),
            Self::application_submitted(),
            Self::loan_approved(),
            Self::closing_reminder(),
        ]
    }

    /// Welcome email template
    pub fn welcome_email() -> OutreachTemplate {
        OutreachTemplate {
            id: 0, // Will be set by database
            name: "Welcome Email".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Welcome to Our Mortgage Services!".to_string()),
            content: r#"Dear {{borrower_name}},

Welcome to our mortgage services! We're excited to help you with your home financing needs.

Your loan application has been received and we're currently reviewing your information. Here's what you can expect next:

1. Document Review (2-3 business days)
2. Initial Underwriting (3-5 business days)
3. Appraisal Coordination (if needed)
4. Final Approval

We'll be in touch soon with next steps. In the meantime, if you have any questions, please don't hesitate to contact us at {{borrower_phone}} or reply to this email.

Best regards,
Your Mortgage Team

Application Details:
- Loan Number: {{loan_number}}
- Status: {{status}}
- Date Received: {{current_date}}"#.to_string(),
            description: Some("Standard welcome email for new clients".to_string()),
            is_default: true,
            is_active: true,
            created_by: None, // System template
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Document request template
    pub fn document_request() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Document Request".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Additional Documents Needed for Your Application".to_string()),
            content: r#"Dear {{borrower_name}},

Thank you for your loan application ({{loan_number}}). To continue processing, we need the following additional documents:

📋 REQUIRED DOCUMENTS:
{{document_list}}

Please send these documents as soon as possible so we can move forward with your application. You can upload them securely through our client portal or reply to this email with attachments.

If you have any questions about what we need or need help obtaining these documents, please contact us immediately.

We're here to help make this process as smooth as possible!

Best regards,
Your Mortgage Team

Application Status: {{status}}
Next Steps: Document review and verification"#.to_string(),
            description: Some("Template for requesting additional documents".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Status update template
    pub fn status_update() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Application Status Update".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Update on Your Loan Application".to_string()),
            content: r#"Dear {{borrower_name}},

We wanted to provide you with an update on your loan application ({{loan_number}}).

📊 CURRENT STATUS: {{status}}

{{additional_notes}}

We're making excellent progress on your application and appreciate your patience during this process. Here's what typically happens next:

- Document Verification: 2-3 business days
- Underwriting Review: 3-5 business days
- Appraisal (if required): 7-10 business days
- Final Approval: 1-2 business days

We'll continue to keep you updated as we progress. If you have any questions about your application status or need assistance with anything, please don't hesitate to reach out.

Best regards,
Your Mortgage Team

Last Updated: {{current_date}}"#.to_string(),
            description: Some("Template for sending application status updates".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Prequalification results template
    pub fn prequalification_results() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Pre-Qualification Results".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Your Pre-Qualification Results".to_string()),
            content: r#"Dear {{borrower_name}},

Thank you for completing the pre-qualification process! We're pleased to share your preliminary results.

🎯 PRE-QUALIFICATION SUMMARY:
Based on the information you provided, here are your estimated loan details:

• Estimated Loan Amount: {{loan_amount}}
• Estimated Interest Rate: {{interest_rate}}
• Estimated Monthly Payment: {{monthly_payment}}
• Employment: {{employer_name}}
• Income Type: {{income_type}}

These are preliminary estimates based on the information you provided and current market conditions. Final terms will be determined after a full application and underwriting process.

📋 NEXT STEPS:
1. Submit formal loan application
2. Provide supporting documentation
3. Schedule property appraisal (if purchasing)
4. Complete underwriting review

Would you like to proceed with a full application? We can help you through every step of the process.

Please contact us if you'd like to discuss these results or have any questions.

Best regards,
Your Mortgage Team

Pre-Qualified On: {{current_date}}"#.to_string(),
            description: Some("Template for sending pre-qualification results".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Follow-up email template
    pub fn follow_up_email() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Follow-Up Email".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Following Up on Your Loan Application".to_string()),
            content: r#"Dear {{borrower_name}},

I hope this email finds you well. I'm following up regarding your loan application ({{loan_number}}) that we discussed on {{current_date}}.

We wanted to check in and see if you have any questions about the process or if there's anything we can help clarify. Your application is currently in "{{status}}" status.

📞 We'd love to hear from you! Please reply to this email or call us at {{borrower_phone}} to discuss:

• Your current timeline expectations
• Any documentation questions
• Rate lock options
• Next steps in the process

We're here to make this process as smooth and stress-free as possible. Don't hesitate to reach out with any questions, no matter how small they might seem.

Looking forward to speaking with you soon!

Best regards,
Your Mortgage Team

Application: {{loan_number}}
Status: {{status}}
Contact: {{borrower_email}} | {{borrower_phone}}"#.to_string(),
            description: Some("Template for following up with clients".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Application submitted template
    pub fn application_submitted() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Application Submitted".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Your Loan Application Has Been Submitted!".to_string()),
            content: r#"Dear {{borrower_name}},

Congratulations! Your loan application ({{loan_number}}) has been successfully submitted and is now being processed.

🎉 APPLICATION SUBMITTED: {{current_date}}

Here's what happens next:

📋 DOCUMENT REVIEW (1-2 business days)
We'll review all the documents you've provided and may request additional information if needed.

🔍 UNDERWRITING (3-5 business days)
Our underwriting team will evaluate your application against lender guidelines and credit requirements.

🏠 APPRAISAL (7-10 days, if required)
If you're purchasing a home, we'll coordinate the appraisal process.

✅ CONDITIONAL APPROVAL (5-7 business days)
Once underwriting is complete, you'll receive conditional approval with final terms.

🎊 FINAL APPROVAL & CLOSING (3-5 business days)
Final approval, rate lock, and closing coordination.

We'll be in touch every step of the way. Your dedicated loan officer will keep you updated on progress and answer any questions.

Thank you for choosing us for your mortgage needs. We're excited to help you achieve your homeownership goals!

Best regards,
Your Mortgage Team

Application: {{loan_number}}
Submitted: {{current_date}}
Status: Under Review"#.to_string(),
            description: Some("Confirmation when application is submitted".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Loan approved template
    pub fn loan_approved() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Loan Approved".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Congratulations! Your Loan Has Been Approved!".to_string()),
            content: r#"Dear {{borrower_name}},

🎉 CONGRATULATIONS! Your loan application ({{loan_number}}) has been APPROVED! 🎉

We're thrilled to share this exciting news with you. After careful review, your loan has been conditionally approved with the following terms:

📊 APPROVED LOAN DETAILS:
• Loan Amount: {{loan_amount}}
• Interest Rate: {{interest_rate}}
• Monthly Payment: {{monthly_payment}}
• Loan Type: {{loan_type}}
• Term: {{loan_term}}

📋 NEXT STEPS TOWARD CLOSING:
1. ✅ Rate Lock - Lock in your interest rate
2. 📝 Final Documentation Review
3. 🏠 Final Appraisal (if not completed)
4. 🔒 Title Search & Insurance
5. 📅 Closing Date Coordination
6. 🏡 Homeowner's Insurance Setup

Your dedicated loan officer will be reaching out shortly to discuss these next steps and answer any questions you may have.

This is an exciting milestone on your homeownership journey! We're here to support you through every step of the closing process.

Please don't hesitate to contact us with any questions or concerns.

Best regards,
Your Mortgage Team

Application: {{loan_number}}
Approved: {{current_date}}
Status: Conditionally Approved"#.to_string(),
            description: Some("Celebration email for loan approval".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Closing reminder template
    pub fn closing_reminder() -> OutreachTemplate {
        OutreachTemplate {
            id: 0,
            name: "Closing Reminder".to_string(),
            template_type: TemplateType::Email,
            subject: Some("Important: Your Loan Closing is Tomorrow".to_string()),
            content: r#"Dear {{borrower_name}},

This is an important reminder that your loan closing for {{loan_number}} is scheduled for tomorrow.

📅 CLOSING DETAILS:
• Date: {{closing_date}}
• Time: {{closing_time}}
• Location: {{closing_location}}
• What to Bring: Valid ID, funds for closing costs

📋 CLOSING PREPARATION CHECKLIST:
□ Valid government-issued photo ID
□ Funds for closing costs (if applicable)
□ Homeowner's insurance policy (if not already provided)
□ Any remaining documents requested
□ Your real estate attorney (if applicable)

🎯 WHAT TO EXPECT AT CLOSING:
• Final review of loan terms and documents
• Signing of remaining paperwork
• Funds disbursement
• Receipt of keys and possession (if purchasing)

If you have any questions or need to reschedule, please contact us immediately at {{borrower_phone}} or reply to this email.

We're excited for you and looking forward to celebrating your homeownership milestone!

Best regards,
Your Mortgage Team

Loan: {{loan_number}}
Closing: {{closing_date}} at {{closing_time}}
Status: Ready for Closing"#.to_string(),
            description: Some("Reminder email before loan closing".to_string()),
            is_default: true,
            is_active: true,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}