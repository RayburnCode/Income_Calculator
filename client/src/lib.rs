// client/src/lib.rs
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use shared::models::*;
use database::entities::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

mod income;

#[derive(Clone)]
pub struct Client {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl Client {
    pub async fn new() -> Result<Self, String> {
        match database::establish_connection().await {
            Ok(conn) => Ok(Self {
                db: Arc::new(Mutex::new(conn)),
            }),
            Err(database::DatabaseError::ConnectionFailed(msg)) => {
                Err(format!("Failed to connect to database. Please check the database setup.\nDetails: {}", msg))
            },
            Err(database::DatabaseError::MigrationFailed(msg)) => {
                Err(format!("Database migration failed. The database may be corrupted or have an incompatible schema.\nDetails: {}", msg))
            },
            Err(database::DatabaseError::PathError(msg)) => {
                Err(format!("Database path error. Please check file permissions and directory structure.\nDetails: {}", msg))
            },
        }
    }

    // Example function for loan information
    pub async fn save_loan_information(&self, info: LoanInformation) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = loan_information::ActiveModel {
            id: Set(info.id),
            property_type: Set(info.property_type.to_string()),
            occupancy_type: Set(info.occupancy_type.to_string()),
            loan_type: Set(info.loan_type.to_string()),
            new_term_months: Set(info.new_term_months),
            loan_purpose: Set(info.loan_purpose.to_string()),
            appraisal_waiver: Set(info.appraisal_waiver),
            created_at: Set(info.created_at),
            updated_at: Set(info.updated_at),
        };
        active_model.insert(&*db).await?;
        Ok(())
    }

    pub async fn get_loan_information(&self, id: Uuid) -> Result<Option<LoanInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entity = loan_information::Entity::find_by_id(id).one(&*db).await?;
        match entity {
            Some(model) => {
                let info = LoanInformation {
                    id: model.id,
                    property_type: parse_property_type(&model.property_type),
                    occupancy_type: parse_occupancy_type(&model.occupancy_type),
                    loan_type: parse_loan_type(&model.loan_type),
                    new_term_months: model.new_term_months,
                    loan_purpose: parse_loan_purpose(&model.loan_purpose),
                    appraisal_waiver: model.appraisal_waiver,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                };
                Ok(Some(info))
            }
            None => Ok(None),
        }
    }

    // Add more functions for other models as needed
    pub async fn save_borrower(&self, borrower: Borrower) -> Result<(), Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let active_model = borrower::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(borrower.name),
            employer_name: Set(borrower.employer_name),
            income_type: Set(borrower.income_type),
            loan_number: Set(borrower.loan_number),
            created_at: Set(borrower.created_at),
            updated_at: Set(borrower.updated_at),
        };
        active_model.insert(&*db).await?;
        Ok(())
    }

    pub async fn get_borrower(&self, id: i32) -> Result<Option<Borrower>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entity = borrower::Entity::find_by_id(id).one(&*db).await?;
        match entity {
            Some(model) => {
                let borrower = Borrower {
                    id: model.id,
                    name: model.name,
                    employer_name: model.employer_name,
                    income_type: model.income_type,
                    loan_number: model.loan_number,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                };
                Ok(Some(borrower))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_borrowers(&self) -> Result<Vec<Borrower>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entities = borrower::Entity::find().all(&*db).await?;
        let borrowers = entities.into_iter().map(|model| Borrower {
            id: model.id,
            name: model.name,
            employer_name: model.employer_name,
            income_type: model.income_type,
            loan_number: model.loan_number,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }).collect();
        Ok(borrowers)
    }

    pub async fn get_all_loan_information(&self) -> Result<Vec<LoanInformation>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entities = loan_information::Entity::find().all(&*db).await?;
        let loans = entities.into_iter().map(|model| LoanInformation {
            id: model.id,
            property_type: parse_property_type(&model.property_type),
            occupancy_type: parse_occupancy_type(&model.occupancy_type),
            loan_type: parse_loan_type(&model.loan_type),
            new_term_months: model.new_term_months,
            loan_purpose: parse_loan_purpose(&model.loan_purpose),
            appraisal_waiver: model.appraisal_waiver,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }).collect();
        Ok(loans)
    }

    pub async fn get_all_mortgage_refinance_options(&self) -> Result<Vec<MortgageRefinanceOptions>, Box<dyn std::error::Error>> {
        let db = self.db.lock().await;
        let entities = mortgage_refinance::Entity::find().all(&*db).await?;

        let mut options = Vec::new();

        for model in entities {
            // Load related entities separately (borrower is referenced by ID only)
            let loan_info = loan_information::Entity::find_by_id(model.loan_information_id).one(&*db).await?;
            let new_loan = new_loan_details::Entity::find_by_id(model.new_loan_details_id).one(&*db).await?;
            let benefit = benefit_to_borrower::Entity::find_by_id(model.benefit_to_borrower_id).one(&*db).await?;
            let fees = other_fees::Entity::find_by_id(model.other_fees_id).one(&*db).await?;
            let income = income_information::Entity::find_by_id(model.income_information_id).one(&*db).await?;
            let savings = savings_calculations::Entity::find_by_id(model.savings_calculation_id).one(&*db).await?;

            // For now, load empty collections for many-to-many relationships
            // TODO: Implement proper loading when junction table entities are available
            let existing_loans = vec![];
            let pricing_options = vec![];
            let consumer_debts = vec![];

            // Convert database models to shared models (borrower is referenced by ID only)
            let loan_info_model = loan_info.map(|l| LoanInformation {
                id: l.id,
                property_type: parse_property_type(&l.property_type),
                occupancy_type: parse_occupancy_type(&l.occupancy_type),
                loan_type: parse_loan_type(&l.loan_type),
                new_term_months: l.new_term_months as u32,
                loan_purpose: parse_loan_purpose(&l.loan_purpose),
                appraisal_waiver: l.appraisal_waiver,
                created_at: l.created_at,
                updated_at: l.updated_at,
            }).unwrap_or_default();

            let new_loan_model = new_loan.map(|n| NewLoanDetails {
                id: n.id,
                market_value: n.market_value.to_string().parse().unwrap_or(0.0),
                sales_price: n.sales_price.to_string().parse().unwrap_or(0.0),
                down_payment: n.down_payment.to_string().parse().unwrap_or(0.0),
                base_loan_amount: n.base_loan_amount.to_string().parse().unwrap_or(0.0),
                subordinated_amount: n.subordinated_amount.to_string().parse().unwrap_or(0.0),
                ff_umip_percentage: n.ff_umip_percentage.to_string().parse().unwrap_or(0.0),
                umip_refund: n.umip_refund.to_string().parse().unwrap_or(0.0),
                total_loan_amount: n.total_loan_amount.to_string().parse().unwrap_or(0.0),
                note_rate: n.note_rate.to_string().parse().unwrap_or(0.0),
                appraisal_waiver: n.appraisal_waiver,
                created_at: n.created_at,
                updated_at: n.updated_at,
            }).unwrap_or_default();

            let benefit_model = benefit.map(|b| BenefitToBorrower {
                id: b.id,
                existing_pi: b.existing_pi.to_string().parse().unwrap_or(0.0),
                existing_taxes: b.existing_taxes.to_string().parse().unwrap_or(0.0),
                existing_insurance: b.existing_insurance.to_string().parse().unwrap_or(0.0),
                existing_flood_insurance: b.existing_flood_insurance.to_string().parse().unwrap_or(0.0),
                existing_pmi: b.existing_pmi.to_string().parse().unwrap_or(0.0),
                existing_hoa: b.existing_hoa.to_string().parse().unwrap_or(0.0),
                existing_mortgage_payment: b.existing_mortgage_payment.to_string().parse().unwrap_or(0.0),
                proposed_pi: b.proposed_pi.to_string().parse().unwrap_or(0.0),
                proposed_taxes: b.proposed_taxes.to_string().parse().unwrap_or(0.0),
                proposed_insurance: b.proposed_insurance.to_string().parse().unwrap_or(0.0),
                proposed_flood_insurance: b.proposed_flood_insurance.to_string().parse().unwrap_or(0.0),
                proposed_pmi: b.proposed_pmi.to_string().parse().unwrap_or(0.0),
                proposed_hoa: b.proposed_hoa.to_string().parse().unwrap_or(0.0),
                proposed_mortgage_payment: b.proposed_mortgage_payment.to_string().parse().unwrap_or(0.0),
                escrow_taxes: b.escrow_taxes,
                escrow_insurance: b.escrow_insurance,
                escrow_flood_insurance: b.escrow_flood_insurance,
                overage_shortage: b.overage_shortage.to_string().parse().unwrap_or(0.0),
                debt_paydown: b.debt_paydown.to_string().parse().unwrap_or(0.0),
                existing_total_obligations: b.existing_total_obligations.to_string().parse().unwrap_or(0.0),
                proposed_total_obligations: b.proposed_total_obligations.to_string().parse().unwrap_or(0.0),
                created_at: b.created_at,
                updated_at: b.updated_at,
            }).unwrap_or_default();

            let fees_model = fees.map(|f| OtherFees {
                id: f.id,
                third_party_fees: f.third_party_fees.to_string().parse().unwrap_or(0.0),
                appraisal_fee: f.appraisal_fee.to_string().parse().unwrap_or(0.0),
                investor_fee: f.investor_fee.to_string().parse().unwrap_or(0.0),
                padded_taxes: f.padded_taxes.to_string().parse().unwrap_or(0.0),
                padded_insurance: f.padded_insurance.to_string().parse().unwrap_or(0.0),
                lender_credit: f.lender_credit.to_string().parse().unwrap_or(0.0),
                admin_fees: f.admin_fees.to_string().parse().unwrap_or(0.0),
                tax_service: f.tax_service.to_string().parse().unwrap_or(0.0),
                flood_certification: f.flood_certification.to_string().parse().unwrap_or(0.0),
                total_closing_costs: f.total_closing_costs.to_string().parse().unwrap_or(0.0),
                cash_out_amount: f.cash_out_amount.to_string().parse().unwrap_or(0.0),
                created_at: f.created_at,
                updated_at: f.updated_at,
            }).unwrap_or_default();

            let income_model = income.map(|i| IncomeInformation {
                id: i.id,
                borrower_monthly_income: i.borrower_monthly_income.to_string().parse().unwrap_or(0.0),
                coborrower_monthly_income: i.coborrower_monthly_income.to_string().parse().unwrap_or(0.0),
                front_end_ratio: i.front_end_ratio.to_string().parse().unwrap_or(0.0),
                back_end_ratio: i.back_end_ratio.to_string().parse().unwrap_or(0.0),
                created_at: i.created_at,
                updated_at: i.updated_at,
            }).unwrap_or_default();

            let savings_model = savings.map(|s| SavingsCalculation {
                id: s.id,
                monthly_savings: s.monthly_savings.to_string().parse().unwrap_or(0.0),
                annual_savings: s.annual_savings.to_string().parse().unwrap_or(0.0),
                debt_paid: s.debt_paid.to_string().parse().unwrap_or(0.0),
                payment_reduction: s.payment_reduction.to_string().parse().unwrap_or(0.0),
                recoup_period_months: s.recoup_period_months.to_string().parse().unwrap_or(0.0),
                created_at: s.created_at,
                updated_at: s.updated_at,
            }).unwrap_or_default();

            let option = MortgageRefinanceOptions {
                id: model.id,
                borrower_id: model.borrower_id,
                loan_information: loan_info_model,
                new_loan_details: new_loan_model,
                existing_loans,
                benefit_to_borrower: benefit_model,
                other_fees: fees_model,
                pricing_options,
                consumer_debts,
                income_information: income_model,
                savings_calculation: savings_model,
                status: model.status,
                created_at: model.created_at,
                updated_at: model.updated_at,
                submitted_at: model.submitted_at,
            };

            options.push(option);
        }

        Ok(options)
    }

    // Add more functions for other models as needed
}

fn parse_property_type(s: &str) -> PropertyType {
    match s {
        "SFR" => PropertyType::SFR,
        "Manufactured" => PropertyType::Manufactured,
        "MultiUnit" => PropertyType::MultiUnit,
        "Condo" => PropertyType::Condo,
        "PUD" => PropertyType::PUD,
        _ => PropertyType::SFR,
    }
}

fn parse_occupancy_type(s: &str) -> OccupancyType {
    match s {
        "Primary" => OccupancyType::Primary,
        "Secondary" => OccupancyType::Secondary,
        "Investment" => OccupancyType::Investment,
        _ => OccupancyType::Primary,
    }
}

fn parse_loan_type(s: &str) -> LoanType {
    match s {
        "CNV" => LoanType::CNV,
        "FHA" => LoanType::FHA,
        "VA" => LoanType::VA,
        "NonQM" => LoanType::NonQM,
        _ => LoanType::CNV,
    }
}

fn parse_loan_purpose(s: &str) -> LoanPurpose {
    match s {
        "Purchase" => LoanPurpose::Purchase,
        "CashOut" => LoanPurpose::CashOut,
        "Refinance" => LoanPurpose::Refinance,
        "IRRRLStreamline" => LoanPurpose::IRRRLStreamline,
        _ => LoanPurpose::Refinance,
    }
}