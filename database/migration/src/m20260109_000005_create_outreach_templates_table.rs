use sea_orm_migration::prelude::*;
use chrono;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create outreach_templates table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("outreach_templates"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("template_type")).string().not_null())
                    .col(ColumnDef::new(Alias::new("subject")).string().null())
                    .col(ColumnDef::new(Alias::new("content")).text().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("is_default")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("is_active")).boolean().not_null().default(true))
                    .col(ColumnDef::new(Alias::new("created_by")).string().null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Insert default templates
        let now = chrono::Utc::now();
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Alias::new("outreach_templates"))
                    .columns([
                        Alias::new("name"),
                        Alias::new("template_type"),
                        Alias::new("subject"),
                        Alias::new("content"),
                        Alias::new("description"),
                        Alias::new("is_default"),
                        Alias::new("is_active"),
                        Alias::new("created_by"),
                        Alias::new("created_at"),
                        Alias::new("updated_at"),
                    ])
                    .values_panic([
                        "Welcome Email".into(),
                        "Email".into(),
                        "Welcome to Our Mortgage Services!".into(),
                        "Dear {{borrower_name}},

Welcome to our mortgage services! We're excited to help you with your home financing needs.

Your loan application has been received and we're currently reviewing your information. We'll be in touch soon with next steps.

If you have any questions, please don't hesitate to contact us.

Best regards,
Your Mortgage Team".into(),
                        "Default welcome email for new clients".into(),
                        true.into(),
                        true.into(),
                        Option::<String>::None.into(),
                        now.into(),
                        now.into(),
                    ])
                    .values_panic([
                        "Document Request".into(),
                        "Email".into(),
                        "Additional Documents Needed for Your Application".into(),
                        "Dear {{borrower_name}},

Thank you for your loan application. To continue processing, we need the following additional documents:

{{document_list}}

Please send these documents as soon as possible so we can move forward with your application.

If you have any questions about what we need, please contact us.

Best regards,
Your Mortgage Team".into(),
                        "Template for requesting additional documents".into(),
                        true.into(),
                        true.into(),
                        Option::<String>::None.into(),
                        now.into(),
                        now.into(),
                    ])
                    .values_panic([
                        "Application Status Update".into(),
                        "Email".into(),
                        "Update on Your Loan Application".into(),
                        "Dear {{borrower_name}},

We wanted to provide you with an update on your loan application.

Current Status: {{application_status}}

{{additional_notes}}

We'll continue to keep you updated as we progress. If you have any questions, please don't hesitate to reach out.

Best regards,
Your Mortgage Team".into(),
                        "Template for sending application status updates".into(),
                        true.into(),
                        true.into(),
                        Option::<String>::None.into(),
                        now.into(),
                        now.into(),
                    ])
                    .values_panic([
                        "Pre-Qualification Results".into(),
                        "Email".into(),
                        "Your Pre-Qualification Results".into(),
                        "Dear {{borrower_name}},

Thank you for completing the pre-qualification process. Here are your results:

Estimated Loan Amount: {{loan_amount}}
Estimated Interest Rate: {{interest_rate}}
Estimated Monthly Payment: {{monthly_payment}}

These are preliminary estimates based on the information you provided. Final terms will be determined after a full application and underwriting process.

Please contact us if you'd like to proceed with a full application or have any questions.

Best regards,
Your Mortgage Team".into(),
                        "Template for sending pre-qualification results".into(),
                        true.into(),
                        true.into(),
                        Option::<String>::None.into(),
                        now.into(),
                        now.into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop outreach_templates table
        manager
            .drop_table(Table::drop().table(Alias::new("outreach_templates")).to_owned())
            .await?;

        Ok(())
    }
}