use sea_orm::{EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, DatabaseConnection};
use shared::models::*;
use database::entities::{
    loan_information, new_loan_details, benefit_to_borrower, other_fees, 
    income_information, savings_calculations, existing_loans, pricing_options,
    consumer_debt,
};
use uuid::Uuid;
use rust_decimal::{Decimal, prelude::ToPrimitive};
use chrono::Utc;

use crate::{parse_credit_type, parse_property_type, parse_occupancy_type, parse_loan_type, parse_loan_purpose};

// Options Template CRUD operations - composite operations that handle all related data

/// Save a complete options template for a borrower
pub async fn save_options_template(db: &DatabaseConnection, template: OptionsTemplateData, borrower_id: i32) -> Result<(), Box<dyn std::error::Error>> {

        // Convert and save loan information
        let loan_info = convert_to_loan_information(&template.loan_information, borrower_id);
        let loan_info_active = loan_information::ActiveModel {
            id: Set(loan_info.id),
            property_type: Set(loan_info.property_type.to_string()),
            occupancy_type: Set(loan_info.occupancy_type.to_string()),
            loan_type: Set(loan_info.loan_type.to_string()),
            new_term_months: Set(loan_info.new_term_months),
            loan_purpose: Set(loan_info.loan_purpose.to_string()),
            appraisal_waiver: Set(loan_info.appraisal_waiver),
            created_at: Set(loan_info.created_at),
            updated_at: Set(Utc::now()),
        };
        loan_info_active.insert(db).await?;

        // Convert and save new loan details
        let new_loan = convert_to_new_loan_details(&template.new_loan, borrower_id);
        let new_loan_active = new_loan_details::ActiveModel {
            id: Set(new_loan.id),
            market_value: Set(Decimal::from_f64_retain(new_loan.market_value).unwrap()),
            sales_price: Set(Decimal::from_f64_retain(new_loan.sales_price).unwrap()),
            down_payment: Set(Decimal::from_f64_retain(new_loan.down_payment).unwrap()),
            base_loan_amount: Set(Decimal::from_f64_retain(new_loan.base_loan_amount).unwrap()),
            subordinated_amount: Set(Decimal::from_f64_retain(new_loan.subordinated_amount).unwrap()),
            total_loan_amount: Set(Decimal::from_f64_retain(new_loan.total_loan_amount).unwrap()),
            note_rate: Set(Decimal::from_f64_retain(new_loan.note_rate).unwrap()),
            appraisal_waiver: Set(new_loan.appraisal_waiver),
            created_at: Set(new_loan.created_at),
            updated_at: Set(Utc::now()),
            ff_umip_percentage: Set(Decimal::from_f64_retain(0.0).unwrap()), // Default values for fields not in frontend
            umip_refund: Set(Decimal::from_f64_retain(0.0).unwrap()),
        };
        new_loan_active.insert(db).await?;

        // Convert and save benefit to borrower
        let benefit = convert_to_benefit_to_borrower(&template.benefit_to_borrower, borrower_id);
        let benefit_active = benefit_to_borrower::ActiveModel {
            id: Set(benefit.id),
            existing_pi: Set(Decimal::from_f64_retain(benefit.existing_pi).unwrap()),
            existing_taxes: Set(Decimal::from_f64_retain(benefit.existing_taxes).unwrap()),
            existing_insurance: Set(Decimal::from_f64_retain(benefit.existing_insurance).unwrap()),
            existing_flood_insurance: Set(Decimal::from_f64_retain(benefit.existing_flood_insurance).unwrap()),
            existing_pmi: Set(Decimal::from_f64_retain(benefit.existing_pmi).unwrap()),
            existing_hoa: Set(Decimal::from_f64_retain(benefit.existing_hoa).unwrap()),
            existing_mortgage_payment: Set(Decimal::from_f64_retain(benefit.existing_mortgage_payment).unwrap()),
            proposed_pi: Set(Decimal::from_f64_retain(benefit.proposed_pi).unwrap()),
            proposed_taxes: Set(Decimal::from_f64_retain(benefit.proposed_taxes).unwrap()),
            proposed_insurance: Set(Decimal::from_f64_retain(benefit.proposed_insurance).unwrap()),
            proposed_flood_insurance: Set(Decimal::from_f64_retain(benefit.proposed_flood_insurance).unwrap()),
            proposed_pmi: Set(Decimal::from_f64_retain(benefit.proposed_pmi).unwrap()),
            proposed_hoa: Set(Decimal::from_f64_retain(benefit.proposed_hoa).unwrap()),
            proposed_mortgage_payment: Set(Decimal::from_f64_retain(benefit.proposed_mortgage_payment).unwrap()),
            escrow_taxes: Set(benefit.escrow_taxes),
            escrow_insurance: Set(benefit.escrow_insurance),
            escrow_flood_insurance: Set(benefit.escrow_flood_insurance),
            overage_shortage: Set(Decimal::from_f64_retain(benefit.overage_shortage).unwrap()),
            debt_paydown: Set(Decimal::from_f64_retain(benefit.debt_paydown).unwrap()),
            existing_total_obligations: Set(Decimal::from_f64_retain(benefit.existing_total_obligations).unwrap()),
            proposed_total_obligations: Set(Decimal::from_f64_retain(benefit.proposed_total_obligations).unwrap()),
            created_at: Set(benefit.created_at),
            updated_at: Set(Utc::now()),
        };
        benefit_active.insert(db).await?;

        // Convert and save other fees
        let other_fees = convert_to_other_fees(&template.other_fees, borrower_id);
        let other_fees_active = other_fees::ActiveModel {
            id: Set(other_fees.id),
            third_party_fees: Set(Decimal::from_f64_retain(other_fees.third_party_fees).unwrap()),
            appraisal_fee: Set(Decimal::from_f64_retain(other_fees.appraisal_fee).unwrap()),
            investor_fee: Set(Decimal::from_f64_retain(other_fees.investor_fee).unwrap()),
            padded_taxes: Set(Decimal::from_f64_retain(other_fees.padded_taxes).unwrap()),
            padded_taxes_months: Set(other_fees.padded_taxes_months as i32),
            padded_insurance: Set(Decimal::from_f64_retain(other_fees.padded_insurance).unwrap()),
            padded_insurance_months: Set(other_fees.padded_insurance_months as i32),
            lender_credit: Set(Decimal::from_f64_retain(other_fees.lender_credit).unwrap()),
            admin_fees: Set(Decimal::from_f64_retain(other_fees.admin_fees).unwrap()),
            tax_service: Set(Decimal::from_f64_retain(other_fees.tax_service).unwrap()),
            flood_certification: Set(Decimal::from_f64_retain(other_fees.flood_certification).unwrap()),
            total_closing_costs: Set(Decimal::from_f64_retain(other_fees.total_closing_costs).unwrap()),
            cash_out_amount: Set(Decimal::from_f64_retain(other_fees.cash_out_amount).unwrap()),
            created_at: Set(other_fees.created_at),
            updated_at: Set(Utc::now()),
        };
        other_fees_active.insert(db).await?;

        // Convert and save income information
        let income_info = convert_to_income_information(&template.income_information, borrower_id);
        let income_info_active = income_information::ActiveModel {
            id: Set(income_info.id),
            borrower_monthly_income: Set(Decimal::from_f64_retain(income_info.borrower_monthly_income).unwrap()),
            coborrower_monthly_income: Set(Decimal::from_f64_retain(income_info.coborrower_monthly_income).unwrap()),
            front_end_ratio: Set(Decimal::from_f64_retain(income_info.front_end_ratio).unwrap()),
            back_end_ratio: Set(Decimal::from_f64_retain(income_info.back_end_ratio).unwrap()),
            created_at: Set(income_info.created_at),
            updated_at: Set(Utc::now()),
        };
        income_info_active.insert(db).await?;

        // Convert and save savings calculation
        let savings = convert_to_savings_calculation(&template.savings, borrower_id);
        let savings_active = savings_calculations::ActiveModel {
            id: Set(savings.id),
            monthly_savings: Set(Decimal::from_f64_retain(savings.monthly_savings).unwrap()),
            annual_savings: Set(Decimal::from_f64_retain(savings.annual_savings).unwrap()),
            debt_paid: Set(Decimal::from_f64_retain(savings.debt_paid).unwrap()),
            payment_reduction: Set(Decimal::from_f64_retain(savings.payment_reduction).unwrap()),
            recoup_period_months: Set(Decimal::from_f64_retain(savings.recoup_period_months).unwrap()),
            created_at: Set(savings.created_at),
            updated_at: Set(Utc::now()),
        };
        savings_active.insert(db).await?;

        // Save existing loans
        for loan in &template.mortgage_payoffs.existing_loans {
            let existing_loan = convert_to_existing_loan(loan, borrower_id);
            let existing_loan_active = existing_loans::ActiveModel {
                id: Set(existing_loan.id),
                position: Set(existing_loan.position as i16),
                loan_balance: Set(Decimal::from_f64_retain(existing_loan.loan_balance).unwrap()),
                monthly_payment: Set(Decimal::from_f64_retain(existing_loan.monthly_payment).unwrap()),
                remaining_term_months: Set(existing_loan.remaining_term_months),
                interest_rate: Set(Decimal::from_f64_retain(existing_loan.interest_rate).unwrap()),
                is_subordinate: Set(existing_loan.is_subordinate),
                created_at: Set(existing_loan.created_at),
                updated_at: Set(Utc::now()),
            };
            existing_loan_active.insert(db).await?;
        }

        // Save consumer debts
        for debt_item in &template.consumer_debt.consumer_debts {
            let consumer_debt = convert_to_consumer_debt(debt_item, borrower_id);
            let consumer_debt_active = consumer_debt::ActiveModel {
                id: Set(consumer_debt.id),
                borrower_id: Set(borrower_id),
                debtor_name: Set(consumer_debt.debtor_name),
                credit_type: Set(consumer_debt.credit_type.to_string()),
                balance: Set(Decimal::from_f64_retain(consumer_debt.balance).unwrap()),
                monthly_payment: Set(Decimal::from_f64_retain(consumer_debt.monthly_payment).unwrap()),
                term_months: Set(consumer_debt.term_months.map(|t| t as i32)),
                interest_rate: Set(consumer_debt.interest_rate.map(|r| Decimal::from_f64_retain(r).unwrap())),
                omit_from_dti: Set(consumer_debt.omit_from_dti),
                pay_off_at_closing: Set(consumer_debt.pay_off_at_closing),
                created_at: Set(consumer_debt.created_at),
                updated_at: Set(Utc::now()),
            };
            consumer_debt_active.insert(db).await?;
        }

        Ok(())
    }

    /// Get a complete options template for a borrower
    pub async fn get_options_template(db: &DatabaseConnection, borrower_id: i32) -> Result<Option<OptionsTemplateData>, Box<dyn std::error::Error>> {

        // Get loan information
        let loan_info_entity = loan_information::Entity::find()
            .filter(loan_information::Column::Id.eq(borrower_id.to_string() + "-loan-info"))
            .one(db)
            .await?;

        if loan_info_entity.is_none() {
            return Ok(None);
        }

        let loan_info_model = loan_info_entity.unwrap();
        let loan_information = LoanInformationData {
            property_type: loan_info_model.property_type.clone(),
            occupancy: loan_info_model.occupancy_type.clone(),
            loan_type: loan_info_model.loan_type.clone(),
            term_months: loan_info_model.new_term_months,
            purpose: loan_info_model.loan_purpose.clone(),
            appraisal_waiver: loan_info_model.appraisal_waiver,
        };

        // Get new loan details
        let new_loan_entity = new_loan_details::Entity::find()
            .filter(new_loan_details::Column::Id.eq(borrower_id.to_string() + "-new-loan"))
            .one(&*db)
            .await?;

        let new_loan = if let Some(model) = new_loan_entity {
            NewLoanData {
                market_value: model.market_value.to_f64().unwrap_or(0.0),
                sales_price: model.sales_price.to_f64().unwrap_or(0.0),
                down_payment: model.down_payment.to_f64().unwrap_or(0.0),
                down_payment_percent: 0.0, // Calculate if needed
                base_loan_amount: model.base_loan_amount.to_f64().unwrap_or(0.0),
                subordinated_amount: model.subordinated_amount.to_f64().unwrap_or(0.0),
                total_loan_amount: model.total_loan_amount.to_f64().unwrap_or(0.0),
                note_rate: model.note_rate.to_f64().unwrap_or(0.0),
                appraisal_waiver: model.appraisal_waiver,
            }
        } else {
            NewLoanData::default()
        };

        // Get benefit to borrower
        let benefit_entity = benefit_to_borrower::Entity::find()
            .filter(benefit_to_borrower::Column::Id.eq(borrower_id.to_string() + "-benefit"))
            .one(db)
            .await?;

        let benefit_to_borrower = if let Some(model) = benefit_entity {
            BenefitToBorrowerData {
                existing_pi: model.existing_pi.to_f64().unwrap_or(0.0),
                existing_taxes: model.existing_taxes.to_f64().unwrap_or(0.0),
                existing_insurance: model.existing_insurance.to_f64().unwrap_or(0.0),
                existing_flood_insurance: model.existing_flood_insurance.to_f64().unwrap_or(0.0),
                existing_pmi: model.existing_pmi.to_f64().unwrap_or(0.0),
                existing_hoa: model.existing_hoa.to_f64().unwrap_or(0.0),
                existing_mortgage_payment: model.existing_mortgage_payment.to_f64().unwrap_or(0.0),
                proposed_pi: model.proposed_pi.to_f64().unwrap_or(0.0),
                proposed_taxes: model.proposed_taxes.to_f64().unwrap_or(0.0),
                proposed_insurance: model.proposed_insurance.to_f64().unwrap_or(0.0),
                proposed_flood_insurance: model.proposed_flood_insurance.to_f64().unwrap_or(0.0),
                proposed_pmi: model.proposed_pmi.to_f64().unwrap_or(0.0),
                proposed_hoa: model.proposed_hoa.to_f64().unwrap_or(0.0),
                proposed_mortgage_payment: model.proposed_mortgage_payment.to_f64().unwrap_or(0.0),
                escrow_taxes: model.escrow_taxes,
                escrow_insurance: model.escrow_insurance,
                escrow_flood_insurance: model.escrow_flood_insurance,
                overage_shortage: model.overage_shortage.to_f64().unwrap_or(0.0),
                debt_paydown: model.debt_paydown.to_f64().unwrap_or(0.0),
                existing_total_obligations: model.existing_total_obligations.to_f64().unwrap_or(0.0),
                proposed_total_obligations: model.proposed_total_obligations.to_f64().unwrap_or(0.0),
            }
        } else {
            BenefitToBorrowerData::default()
        };

        // Get other fees
        let other_fees_entity = other_fees::Entity::find()
            .filter(other_fees::Column::Id.eq(borrower_id.to_string() + "-other-fees"))
            .one(db)
            .await?;

        let other_fees = if let Some(model) = other_fees_entity {
            OtherFeesData {
                third_party_fees: model.third_party_fees.to_f64().unwrap_or(0.0),
                appraisal_fee: model.appraisal_fee.to_f64().unwrap_or(0.0),
                investor_fee: model.investor_fee.to_f64().unwrap_or(0.0),
                padded_taxes: model.padded_taxes.to_f64().unwrap_or(0.0),
                padded_taxes_months: model.padded_taxes_months as u32,
                padded_insurance: model.padded_insurance.to_f64().unwrap_or(0.0),
                padded_insurance_months: model.padded_insurance_months as u32,
                lender_credit: model.lender_credit.to_f64().unwrap_or(0.0),
                admin_fees: model.admin_fees.to_f64().unwrap_or(0.0),
                tax_service: model.tax_service.to_f64().unwrap_or(0.0),
                flood_certification: model.flood_certification.to_f64().unwrap_or(0.0),
                total_closing_costs: model.total_closing_costs.to_f64().unwrap_or(0.0),
                cash_out_amount: model.cash_out_amount.to_f64().unwrap_or(0.0),
            }
        } else {
            OtherFeesData::default()
        };

        // Get income information
        let income_entity = income_information::Entity::find()
            .filter(income_information::Column::Id.eq(borrower_id.to_string() + "-income"))
            .one(db)
            .await?;

        let income_information = if let Some(model) = income_entity {
            IncomeInformationData {
                borrower_monthly_income: model.borrower_monthly_income.to_f64().unwrap_or(0.0),
                coborrower_monthly_income: model.coborrower_monthly_income.to_f64().unwrap_or(0.0),
                front_end_ratio: model.front_end_ratio.to_f64().unwrap_or(0.0),
                back_end_ratio: model.back_end_ratio.to_f64().unwrap_or(0.0),
                w2_jobs_data: None,
            }
        } else {
            IncomeInformationData::default()
        };

        // Get savings calculation
        let savings_entity = savings_calculations::Entity::find()
            .filter(savings_calculations::Column::Id.eq(borrower_id.to_string() + "-savings"))
            .one(db)
            .await?;

        let savings = if let Some(model) = savings_entity {
            SavingsData {
                monthly_savings: model.monthly_savings.to_f64().unwrap_or(0.0),
                annual_savings: model.annual_savings.to_f64().unwrap_or(0.0),
                debt_paid: model.debt_paid.to_f64().unwrap_or(0.0),
                payment_reduction: model.payment_reduction.to_f64().unwrap_or(0.0),
                recoup_period_months: model.recoup_period_months.to_f64().unwrap_or(0.0),
            }
        } else {
            SavingsData::default()
        };

        // Get existing loans
        let existing_loans_entities = existing_loans::Entity::find()
            .filter(existing_loans::Column::Id.starts_with(&borrower_id.to_string()))
            .all(&*db)
            .await?;

        let existing_loans = existing_loans_entities.into_iter().map(|model| {
            ExistingLoanData {
                position: model.position as u8,
                loan_balance: model.loan_balance.to_f64().unwrap_or(0.0),
                monthly_payment: model.monthly_payment.to_f64().unwrap_or(0.0),
                remaining_term_months: model.remaining_term_months as u32,
                interest_rate: model.interest_rate.to_f64().unwrap_or(0.0),
                is_subordinate: model.is_subordinate,
            }
        }).collect();

        let mortgage_payoffs = MortgagePayoffsData {
            existing_loans,
        };

        // Get pricing options
        let pricing_entities = pricing_options::Entity::find()
            .filter(pricing_options::Column::Id.starts_with(&borrower_id.to_string()))
            .all(&*db)
            .await?;

        let pricing_options = pricing_entities.into_iter().map(|model| {
            PricingOptionData {
                description: model.description,
                note_rate: model.note_rate.to_f64().unwrap_or(0.0),
                ysp_percentage: model.ysp_percentage.to_f64().unwrap_or(0.0),
                ysp_dollar: model.ysp_dollar.to_f64().unwrap_or(0.0),
                bd_percentage: model.bd_percentage.to_f64().unwrap_or(0.0),
                bd_dollar: model.bd_dollar.to_f64().unwrap_or(0.0),
                is_selected: model.is_selected,
            }
        }).collect();

        let pricing = PricingData {
            pricing_options,
        };

        // Get consumer debts
        let consumer_debt_entities = consumer_debt::Entity::find()
            .filter(consumer_debt::Column::BorrowerId.eq(borrower_id))
            .all(&*db)
            .await?;

        let consumer_debts = consumer_debt_entities.into_iter().map(|model| {
            ConsumerDebtItemData {
                debtor_name: model.debtor_name,
                credit_type: model.credit_type,
                balance: model.balance.to_f64().unwrap_or(0.0),
                monthly_payment: model.monthly_payment.to_f64().unwrap_or(0.0),
                term_months: model.term_months.map(|t| t as u32),
                interest_rate: model.interest_rate.map(|r| r.to_f64().unwrap_or(0.0)),
                omit_from_dti: model.omit_from_dti,
                pay_off_at_closing: model.pay_off_at_closing,
            }
        }).collect();

        let consumer_debt = ConsumerDebtData {
            consumer_debts,
        };

        // For now, return defaults for debt to income, and title fees
        let debt_to_income = DebtToIncomeData::default();
        let title_fees = TitleFeesData::default();

        let template = OptionsTemplateData {
            loan_information,
            mortgage_payoffs,
            new_loan,
            benefit_to_borrower,
            other_fees,
            pricing,
            consumer_debt,
            debt_to_income,
            title_fees,
            income_information,
            savings,
        };

        Ok(Some(template))
    }

    /// Update an existing options template
    pub async fn update_options_template(db: &DatabaseConnection, template: OptionsTemplateData, borrower_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // For now, delete and re-insert. In a production system, you'd want more sophisticated update logic
        delete_options_template(db, borrower_id).await?;
        save_options_template(db, template, borrower_id).await
    }

    /// Delete an options template and all its related data
    pub async fn delete_options_template(db: &DatabaseConnection, borrower_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let borrower_prefix = borrower_id.to_string();

        // Delete all related records
        loan_information::Entity::delete_many()
            .filter(loan_information::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        new_loan_details::Entity::delete_many()
            .filter(new_loan_details::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        benefit_to_borrower::Entity::delete_many()
            .filter(benefit_to_borrower::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        other_fees::Entity::delete_many()
            .filter(other_fees::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        income_information::Entity::delete_many()
            .filter(income_information::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        savings_calculations::Entity::delete_many()
            .filter(savings_calculations::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        existing_loans::Entity::delete_many()
            .filter(existing_loans::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        pricing_options::Entity::delete_many()
            .filter(pricing_options::Column::Id.starts_with(&borrower_prefix))
            .exec(db)
            .await?;

        consumer_debt::Entity::delete_many()
            .filter(consumer_debt::Column::BorrowerId.eq(borrower_id))
            .exec(db)
            .await?;

        Ok(())
    }

    /// Get all options templates (for admin purposes)
    pub async fn get_all_options_templates(_db: &DatabaseConnection) -> Result<Vec<(i32, OptionsTemplateData)>, Box<dyn std::error::Error>> {
        // This would require querying all borrowers and their templates
        // For now, return empty vec
        Ok(vec![])
    }

    pub async fn get_all_mortgage_refinance_options(_db: &DatabaseConnection) -> Result<Vec<shared::models::MortgageRefinanceOptions>, Box<dyn std::error::Error>> {
        // Stub implementation
        Ok(vec![])
    }

    // Helper conversion functions
    pub fn convert_to_loan_information(data: &LoanInformationData, _borrower_id: i32) -> LoanInformation {
        LoanInformation {
            id: Uuid::new_v4(),
            property_type: parse_property_type(&data.property_type),
            occupancy_type: parse_occupancy_type(&data.occupancy),
            loan_type: parse_loan_type(&data.loan_type),
            new_term_months: data.term_months,
            loan_purpose: parse_loan_purpose(&data.purpose),
            appraisal_waiver: data.appraisal_waiver,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_new_loan_details(data: &NewLoanData, _borrower_id: i32) -> NewLoanDetails {
        NewLoanDetails {
            id: Uuid::new_v4(),
            market_value: data.market_value,
            sales_price: data.sales_price,
            down_payment: data.down_payment,
            base_loan_amount: data.base_loan_amount,
            subordinated_amount: data.subordinated_amount,
            ff_umip_percentage: 0.0, // Default value
            umip_refund: 0.0, // Default value
            total_loan_amount: data.total_loan_amount,
            note_rate: data.note_rate,
            appraisal_waiver: data.appraisal_waiver,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_benefit_to_borrower(data: &BenefitToBorrowerData, _borrower_id: i32) -> BenefitToBorrower {
        BenefitToBorrower {
            id: Uuid::new_v4(),
            existing_pi: data.existing_pi,
            existing_taxes: data.existing_taxes,
            existing_insurance: data.existing_insurance,
            existing_flood_insurance: data.existing_flood_insurance,
            existing_pmi: data.existing_pmi,
            existing_hoa: data.existing_hoa,
            existing_mortgage_payment: data.existing_mortgage_payment,
            proposed_pi: data.proposed_pi,
            proposed_taxes: data.proposed_taxes,
            proposed_insurance: data.proposed_insurance,
            proposed_flood_insurance: data.proposed_flood_insurance,
            proposed_pmi: data.proposed_pmi,
            proposed_hoa: data.proposed_hoa,
            proposed_mortgage_payment: data.proposed_mortgage_payment,
            escrow_taxes: data.escrow_taxes,
            escrow_insurance: data.escrow_insurance,
            escrow_flood_insurance: data.escrow_flood_insurance,
            overage_shortage: data.overage_shortage,
            debt_paydown: data.debt_paydown,
            existing_total_obligations: data.existing_total_obligations,
            proposed_total_obligations: data.proposed_total_obligations,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_other_fees(data: &OtherFeesData, _borrower_id: i32) -> OtherFees {
        OtherFees {
            id: Uuid::new_v4(),
            third_party_fees: data.third_party_fees,
            appraisal_fee: data.appraisal_fee,
            investor_fee: data.investor_fee,
            padded_taxes: data.padded_taxes,
            padded_taxes_months: data.padded_taxes_months,
            padded_insurance: data.padded_insurance,
            padded_insurance_months: data.padded_insurance_months,
            lender_credit: data.lender_credit,
            admin_fees: data.admin_fees,
            tax_service: data.tax_service,
            flood_certification: data.flood_certification,
            total_closing_costs: data.total_closing_costs,
            cash_out_amount: data.cash_out_amount,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_income_information(data: &IncomeInformationData, _borrower_id: i32) -> IncomeInformation {
        IncomeInformation {
            id: Uuid::new_v4(),
            borrower_monthly_income: data.borrower_monthly_income,
            coborrower_monthly_income: data.coborrower_monthly_income,
            front_end_ratio: data.front_end_ratio,
            back_end_ratio: data.back_end_ratio,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_savings_calculation(data: &SavingsData, _borrower_id: i32) -> SavingsCalculation {
        SavingsCalculation {
            id: Uuid::new_v4(),
            monthly_savings: data.monthly_savings,
            annual_savings: data.annual_savings,
            debt_paid: data.debt_paid,
            payment_reduction: data.payment_reduction,
            recoup_period_months: data.recoup_period_months,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_existing_loan(data: &ExistingLoanData, _borrower_id: i32) -> ExistingLoan {
        ExistingLoan {
            id: Uuid::new_v4(),
            position: data.position,
            loan_balance: data.loan_balance,
            monthly_payment: data.monthly_payment,
            remaining_term_months: data.remaining_term_months,
            interest_rate: data.interest_rate,
            is_subordinate: data.is_subordinate,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_pricing_option(data: &PricingOptionData, _borrower_id: i32) -> PricingOption {
        PricingOption {
            id: Uuid::new_v4(),
            description: data.description.clone(),
            note_rate: data.note_rate,
            ysp_percentage: data.ysp_percentage,
            ysp_dollar: data.ysp_dollar,
            bd_percentage: data.bd_percentage,
            bd_dollar: data.bd_dollar,
            is_selected: data.is_selected,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn convert_to_consumer_debt(data: &ConsumerDebtItemData, _borrower_id: i32) -> ConsumerDebt {
        ConsumerDebt {
            id: Uuid::new_v4(),
            debtor_name: data.debtor_name.clone(),
            credit_type: parse_credit_type(&data.credit_type),
            balance: data.balance,
            monthly_payment: data.monthly_payment,
            term_months: data.term_months,
            interest_rate: data.interest_rate,
            omit_from_dti: data.omit_from_dti,
            pay_off_at_closing: data.pay_off_at_closing,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
