//! Settings repository - handles application settings

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use shared::models::*;
use database::entities::settings;
use chrono::Utc;

pub struct SettingsRepository;

impl SettingsRepository {
    pub async fn get(db: &DatabaseConnection) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let settings = settings::Entity::find().one(db).await?;

        match settings {
            Some(model) => Ok(AppSettings {
                id: model.id,
                theme: model.theme,
                currency: model.currency,
                default_loan_term: model.default_loan_term,
                dti_threshold: model.dti_threshold,
                auto_backup: model.auto_backup,
            }),
            None => {
                let default_settings = AppSettings::default();
                Self::save(db, default_settings.clone()).await?;
                Ok(default_settings)
            }
        }
    }

    pub async fn save(db: &DatabaseConnection, settings: AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = settings::ActiveModel {
            id: Set(settings.id),
            theme: Set(settings.theme.clone()),
            currency: Set(settings.currency.clone()),
            default_loan_term: Set(settings.default_loan_term),
            dti_threshold: Set(settings.dti_threshold),
            auto_backup: Set(settings.auto_backup),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        match active_model.update(db).await {
            Ok(_) => Ok(()),
            Err(_) => {
                let insert_model = settings::ActiveModel {
                    id: Set(settings.id),
                    theme: Set(settings.theme),
                    currency: Set(settings.currency),
                    default_loan_term: Set(settings.default_loan_term),
                    dti_threshold: Set(settings.dti_threshold),
                    auto_backup: Set(settings.auto_backup),
                    created_at: Set(Utc::now()),
                    updated_at: Set(Utc::now()),
                };
                insert_model.insert(db).await?;
                Ok(())
            }
        }
    }
}
