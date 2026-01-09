pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m20260106_000001_create_client_info_tables;
mod m20260106_000002_create_income_worksheet_tables;
mod m20260106_000003_create_loan_and_calculation_tables;
mod m20260106_000004_create_settings_table;
mod m20260106_000007_create_fee_and_refinance_tables;
mod m20260106_000008_create_junction_tables;
mod m20260109_000001_add_property_type_to_loan_information;
mod m20260109_000002_add_missing_columns_to_loan_information;
mod m20260109_000003_add_borrower_personal_info;
mod m20260109_000004_create_timeline_events_table;
mod m20260109_000005_create_outreach_templates_table;
mod m20260109_000006_create_campaigns_table;
mod m20260109_000007_create_notes_table;
mod m20260109_000008_create_documents_table;
mod m20260109_000009_create_conditions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260106_000001_create_client_info_tables::Migration),
            Box::new(m20260106_000002_create_income_worksheet_tables::Migration),
            Box::new(m20260106_000003_create_loan_and_calculation_tables::Migration),
            Box::new(m20260106_000004_create_settings_table::Migration),
            Box::new(m20260106_000007_create_fee_and_refinance_tables::Migration),
            Box::new(m20260106_000008_create_junction_tables::Migration),
            Box::new(m20260109_000001_add_property_type_to_loan_information::Migration),
            Box::new(m20260109_000002_add_missing_columns_to_loan_information::Migration),
            Box::new(m20260109_000003_add_borrower_personal_info::Migration),
            Box::new(m20260109_000004_create_timeline_events_table::Migration),
            Box::new(m20260109_000005_create_outreach_templates_table::Migration),
            Box::new(m20260109_000006_create_campaigns_table::Migration),
            Box::new(m20260109_000007_create_notes_table::Migration),
            Box::new(m20260109_000008_create_documents_table::Migration),
            Box::new(m20260109_000009_create_conditions_table::Migration),
        ]
    }
} 