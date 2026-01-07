pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m20260106_000000_create_mortgage_refinance_tables;
mod m20260106_000001_create_client_info_tables;
mod m20260106_000002_create_income_worksheet_tables;
mod m20260106_000003_create_options_template_tables;
mod m20260106_000004_create_settings_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260106_000000_create_mortgage_refinance_tables::Migration),
            Box::new(m20260106_000001_create_client_info_tables::Migration),
            Box::new(m20260106_000002_create_income_worksheet_tables::Migration),
            Box::new(m20260106_000003_create_options_template_tables::Migration),
            Box::new(m20260106_000004_create_settings_table::Migration),
        ]
    }
} 