pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m20260106_000001_create_client_info_tables;
mod m20260106_000002_create_income_worksheet_tables;
mod m20260106_000003_create_loan_and_calculation_tables;
mod m20260106_000004_create_settings_table;
mod m20260106_000007_create_fee_and_refinance_tables;
mod m20260106_000008_create_junction_tables;

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
        ]
    }
} 