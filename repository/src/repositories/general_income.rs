use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, PaginatorTrait};
use shared::models::{GeneralIncomeData, GeneralIncomeEntry};
use database::entities::{general_income_data, general_income_entries};
use chrono::Utc;
use uuid::Uuid;

/// Save general income data for a borrower
pub async fn save_general_income_data(
    db: &DatabaseConnection,
    borrower_id: i32,
    data: GeneralIncomeData,
) -> Result<(), Box<dyn std::error::Error>> {
    // First, delete existing data for this borrower
    general_income_entries::Entity::delete_many()
        .filter(general_income_entries::Column::BorrowerId.eq(borrower_id))
        .exec(db)
        .await?;

    // Delete existing general income data record
    general_income_data::Entity::delete_many()
        .filter(general_income_data::Column::BorrowerId.eq(borrower_id))
        .exec(db)
        .await?;

    // Insert new general income data
    let general_income_data_id = Uuid::new_v4();
    let general_income_data_active = general_income_data::ActiveModel {
        id: Set(general_income_data_id),
        borrower_id: Set(borrower_id),
        is_verified: Set(data.is_verified),
        verified_at: Set(data.verified_at.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        })),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };
    general_income_data_active.insert(db).await?;

    // Insert income entries
    for entry in &data.entries {
        let verified_at = entry.verified_at.as_ref().map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        });

        let entry_active = general_income_entries::ActiveModel {
            id: Set(entry.id.clone()),
            borrower_id: Set(borrower_id),
            income_type: Set(entry.income_type.clone()),
            source_name: Set(entry.source_name.clone()),
            description: Set(entry.description.clone()),
            monthly_amount: Set(entry.monthly_amount.clone()),
            annual_amount: Set(entry.annual_amount.clone()),
            is_verified: Set(entry.is_verified),
            verified_at: Set(verified_at),
            notes: Set(entry.notes.clone()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        entry_active.insert(db).await?;
    }

    Ok(())
}

/// Load general income data for a borrower
pub async fn get_general_income_data(
    db: &DatabaseConnection,
    borrower_id: i32,
) -> Result<Option<GeneralIncomeData>, Box<dyn std::error::Error>> {
    // Get the general income data record
    let general_income_record = general_income_data::Entity::find()
        .filter(general_income_data::Column::BorrowerId.eq(borrower_id))
        .one(db)
        .await?;

    if let Some(record) = general_income_record {
        // Get all entries for this borrower
        let entries = general_income_entries::Entity::find()
            .filter(general_income_entries::Column::BorrowerId.eq(borrower_id))
            .all(db)
            .await?;

        let general_income_entries: Vec<GeneralIncomeEntry> = entries
            .into_iter()
            .map(|entry| GeneralIncomeEntry {
                id: entry.id,
                income_type: entry.income_type,
                source_name: entry.source_name,
                description: entry.description,
                monthly_amount: entry.monthly_amount,
                annual_amount: entry.annual_amount,
                is_verified: entry.is_verified,
                verified_at: entry.verified_at.map(|dt| dt.to_rfc3339()),
                notes: entry.notes,
            })
            .collect();

        let data = GeneralIncomeData {
            entries: general_income_entries,
            is_verified: record.is_verified,
            verified_at: record.verified_at.map(|dt| dt.to_rfc3339()),
        };

        Ok(Some(data))
    } else {
        Ok(None)
    }
}

/// Delete general income data for a borrower
pub async fn delete_general_income_data(
    db: &DatabaseConnection,
    borrower_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete entries first (due to foreign key constraints)
    general_income_entries::Entity::delete_many()
        .filter(general_income_entries::Column::BorrowerId.eq(borrower_id))
        .exec(db)
        .await?;

    // Delete the main record
    general_income_data::Entity::delete_many()
        .filter(general_income_data::Column::BorrowerId.eq(borrower_id))
        .exec(db)
        .await?;

    Ok(())
}