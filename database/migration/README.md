<!-- @format -->

# Database Migration Guide

This directory contains SeaORM migrations for the Income Calculator application.

## Prerequisites

- Rust toolchain installed
- SQLite database (configured in your application)
- Environment variable `DATABASE_URL` set to your database location

## Running Migrations

### Navigate to the migration directory

```bash
cd database/migration
```

### Available Commands

#### Apply All Pending Migrations

```bash
cargo run -- up
```

This command runs all migrations that haven't been applied yet.

#### Apply Specific Number of Migrations

```bash
cargo run -- up -n 1
```

This applies only the next pending migration.

#### Rollback Last Migration

```bash
cargo run -- down
```

This rolls back the most recently applied migration.

#### Rollback Specific Number of Migrations

```bash
cargo run -- down -n 2
```

This rolls back the last 2 migrations.

#### Check Migration Status

```bash
cargo run -- status
```

This shows which migrations have been applied and which are pending.

#### Refresh Database (Rollback All + Reapply All)

```bash
cargo run -- fresh
```

This rolls back all migrations and reapplies them from scratch.

#### Reset Database (Drop All Tables + Reapply All)

```bash
cargo run -- reset
```

This drops all tables and reapplies all migrations.

## Environment Setup

Set your database URL before running migrations:

```bash
# For SQLite (example)
export DATABASE_URL="sqlite://./income_calculator.db?mode=rwc"

# Or add to your .env file
echo 'DATABASE_URL="sqlite://./income_calculator.db?mode=rwc"' >> .env
```

## Migration Files

Current migrations in order:

1. `m20260106_000001_create_client_info_tables.rs` - Client information tables
2. `m20260106_000002_create_income_worksheet_tables.rs` - Income worksheet tables
3. `m20260106_000003_create_loan_and_calculation_tables.rs` - Loan and calculation tables
4. `m20260106_000004_create_settings_table.rs` - Application settings table
5. `m20260106_000007_create_fee_and_refinance_tables.rs` - Fee and refinance tables
6. `m20260106_000008_create_junction_tables.rs` - Junction/relationship tables

## Creating New Migrations

To create a new migration:

```bash
# From the project root or database directory
sea-orm-migration generate <migration_name>
```

This will create a new migration file in `src/` with the current timestamp.

After creating the migration:

1. Implement the `up()` method (apply changes)
2. Implement the `down()` method (rollback changes)
3. Add the migration to the `Migrator` in `lib.rs`

## Common Issues

### Database URL Not Set

If you see an error about database connection, ensure `DATABASE_URL` is set:

```bash
echo $DATABASE_URL  # Should show your database path
```

### Permission Issues

Make sure the directory where your SQLite database is located has write permissions.

### Migration Order

Migrations are applied in the order they are listed in `lib.rs`. The order matters if migrations depend on tables created by previous migrations.

## Running Migrations from Application Code

You can also run migrations programmatically from your Rust application:

```rust
use migration::{Migrator, MigratorTrait};

// In your application startup
Migrator::up(&db, None).await?;
```

## Additional Resources

- [SeaORM Migration Documentation](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)
- [SeaORM CLI Reference](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/)
