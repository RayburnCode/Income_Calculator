use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "w2_jobs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub borrower_id: i32,
    pub employer_name: String,
    pub job_title: String,
    pub years_employed: Option<i32>,
    pub months_employed: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub annual_salary: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub hourly_rate: Option<Decimal>,
    pub hours_per_week: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub commission_monthly: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bonus_monthly: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub overtime_monthly: Option<Decimal>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::borrower::Entity",
        from = "Column::BorrowerId",
        to = "super::borrower::Column::Id"
    )]
    Borrower,
}

impl Related<super::borrower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Borrower.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}