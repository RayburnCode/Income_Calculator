//! W2 Jobs repository - handles W2 employment data

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use shared::models::*;
use database::entities::w2_jobs;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::Utc;
use crate::converters::*;

pub struct W2JobsRepository;

impl W2JobsRepository {
    pub async fn get_by_borrower(db: &DatabaseConnection, borrower_id: i32) -> Result<Option<W2JobsData>, Box<dyn std::error::Error>> {
        let jobs: Vec<w2_jobs::Model> = w2_jobs::Entity::find()
            .filter(w2_jobs::Column::BorrowerId.eq(borrower_id))
            .all(db)
            .await?;

        if jobs.is_empty() {
            return Ok(None);
        }

        let w2_jobs: Vec<W2Job> = jobs.iter().map(w2_job_to_domain).collect();

        Ok(Some(W2JobsData {
            jobs: w2_jobs,
            is_verified: false,
            verified_at: None,
        }))
    }

    pub async fn save_for_borrower(db: &DatabaseConnection, borrower_id: i32, w2_data: &W2JobsData) -> Result<(), Box<dyn std::error::Error>> {
        // Delete existing jobs for this borrower
        w2_jobs::Entity::delete_many()
            .filter(w2_jobs::Column::BorrowerId.eq(borrower_id))
            .exec(db)
            .await?;

        // Insert new jobs
        for job in &w2_data.jobs {
            let active_model = w2_jobs::ActiveModel {
                id: Set(Uuid::new_v4()),
                borrower_id: Set(borrower_id),
                employer_name: Set(job.employer_name.clone()),
                job_title: Set(job.job_title.clone()),
                years_employed: Set(job.years_employed.parse::<i32>().ok()),
                months_employed: Set(job.months_employed.parse::<i32>().ok()),
                annual_salary: Set(job.annual_salary.parse::<Decimal>().ok()),
                hourly_rate: Set(job.hourly_rate.parse::<Decimal>().ok()),
                hours_per_week: Set(job.hours_per_week.parse::<i32>().ok()),
                commission_monthly: Set(job.commission_monthly.parse::<Decimal>().ok()),
                bonus_monthly: Set(job.bonus_monthly.parse::<Decimal>().ok()),
                overtime_monthly: Set(job.overtime_monthly.parse::<Decimal>().ok()),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
            };
            active_model.insert(db).await?;
        }

        Ok(())
    }
}
