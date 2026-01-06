pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m20260106_000000_create_mortgage_refinance_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260106_000000_create_mortgage_refinance_tables::Migration),
        ]
    }
} 