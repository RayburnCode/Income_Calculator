use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "loan_information")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub property_type: String,
    pub occupancy_type: String,
    pub loan_type: String,
    pub new_term_months: u32,
    pub loan_purpose: String,
    pub appraisal_waiver: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}