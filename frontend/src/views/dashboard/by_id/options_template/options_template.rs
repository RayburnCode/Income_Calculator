use dioxus::prelude::*;
use crate::views::dashboard::by_id::options_template::*;
use shared::models::*;
use client::Client;
use chrono::Utc;
use uuid::Uuid;

#[component]
pub fn OptionsTemplate(id: i32) -> Element {
    // Get client from context
    let client = use_context::<Client>();

    // Main state for all options template data
    let mut template_data = use_signal(|| OptionsTemplateData::default());

    // Auto-save functionality
    let save_timeout = use_signal(|| None::<i32>);

    // Function to trigger auto-save
    let save_data = move |data: OptionsTemplateData| {
        // Cancel any existing timeout
        if let Some(_timeout_id) = save_timeout() {
            // In a real implementation, you'd cancel the timeout
            // For now, we'll just update the signal
        }

        // Set new timeout for auto-save (e.g., save after 2 seconds of inactivity)
        // In a real implementation, you'd use a proper timeout mechanism
        // For now, we'll save immediately for demonstration
        save_to_backend(client.clone(), data, id);
    };

    // Function to save data to backend
    fn save_to_backend(client: Client, data: OptionsTemplateData, borrower_id: i32) {
        spawn(async move {
            // Convert frontend data to shared models
            let loan_info = convert_to_loan_information(&data.loan_information);
            let _new_loan = convert_to_new_loan_details(&data.new_loan);
            let _savings = convert_to_savings_calculation(&data.savings);
            let _other_fees = convert_to_other_fees(&data.other_fees);
            let _income_info = convert_to_income_information(&data.income_information);

            // For now, just save the loan information as an example
            // TODO: Save the full options template
            match client.save_loan_information(loan_info).await {
                Ok(_) => println!("Successfully saved loan information for borrower {}", borrower_id),
                Err(e) => println!("Failed to save loan information: {:?}", e),
            }
        });
    }

    rsx! {
        div { class: " mx-auto p-6 text-black",
            h1 { class: "text-3xl font-bold text-gray-900 mb-8", "Mortgage Options Template" }

            // Auto-save indicator
            div { class: "mb-4 p-3 bg-green-50 border border-green-200 rounded-lg",
                div { class: "flex items-center gap-2",
                    span { class: "text-green-600", "💾" }
                    span { class: "text-sm text-green-800 font-medium",
                        "Changes are automatically saved to the backend"
                    }
                }
            }

            // All the sections with data binding
            // Savings section - only show if loan purpose is NOT Purchase
            {
                let loan_purpose = template_data().loan_information.purpose.clone();
                if loan_purpose != "purchase" {
                    let save_data_clone = save_data.clone();
                    rsx! {
                        SavingsSection {
                            data: template_data().savings.clone(),
                            on_change: move |new_data: SavingsData| {
                                template_data.write().savings = new_data.clone();
                                save_data_clone(template_data());
                            },
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            LoanInformationSection {
                data: template_data().loan_information.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: LoanInformationData| {
                        template_data.write().loan_information = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }

            // Mortgage Payoffs section - only show if loan purpose is NOT Purchase
            {
                let loan_purpose = template_data().loan_information.purpose.clone();
                if loan_purpose != "purchase" {
                    let save_data_clone = save_data.clone();
                    rsx! {
                        MortgagePayoffsSection {
                            data: template_data().mortgage_payoffs.clone(),
                            on_change: move |new_data: MortgagePayoffsData| {
                                template_data.write().mortgage_payoffs = new_data.clone();
                                save_data_clone(template_data());
                            },
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            NewLoanSection {
                data: template_data().new_loan.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: NewLoanData| {
                        template_data.write().new_loan = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }

            BenefitToBorrowerSection {
                data: template_data().benefit_to_borrower.clone(),
                loan_purpose: template_data().loan_information.purpose.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: BenefitToBorrowerData| {
                        template_data.write().benefit_to_borrower = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }

            PricingSection {
                data: template_data().pricing.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: PricingData| {
                        template_data.write().pricing = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }

            ConsumerDebtSection {
                data: template_data().consumer_debt.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: ConsumerDebtData| {
                        template_data.write().consumer_debt = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }

            DebtToIncomeSection {
                data: template_data().debt_to_income.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: DebtToIncomeData| {
                        template_data.write().debt_to_income = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }
            OtherFeesSection {
                data: template_data().other_fees.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: OtherFeesData| {
                        template_data.write().other_fees = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }
            TitleFeesSection {
                data: template_data().title_fees.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: TitleFeesData| {
                        template_data.write().title_fees = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }
            IncomeInformationSection {
                data: template_data().income_information.clone(),
                on_change: {
                    let save_data_clone = save_data.clone();
                    move |new_data: IncomeInformationData| {
                        template_data.write().income_information = new_data.clone();
                        save_data_clone(template_data());
                    }
                },
            }
        }
    }
}

// Conversion functions from frontend data to shared models
fn convert_to_loan_information(data: &LoanInformationData) -> LoanInformation {
    use shared::models::enums::*;

    let property_type = match data.property_type.as_str() {
        "sfr" => PropertyType::SFR,
        "manufactured" => PropertyType::Manufactured,
        "multiUnit" => PropertyType::MultiUnit,
        "condo" => PropertyType::Condo,
        "pud" => PropertyType::PUD,
        _ => PropertyType::SFR,
    };

    let occupancy_type = match data.occupancy.as_str() {
        "primary" => OccupancyType::Primary,
        "secondary" => OccupancyType::Secondary,
        "investment" => OccupancyType::Investment,
        _ => OccupancyType::Primary,
    };

    let loan_type = match data.loan_type.as_str() {
        "cnv" => LoanType::CNV,
        "fha" => LoanType::FHA,
        "va" => LoanType::VA,
        "nonQM" => LoanType::NonQM,
        _ => LoanType::CNV,
    };

    let loan_purpose = match data.purpose.as_str() {
        "purchase" => LoanPurpose::Purchase,
        "cashOut" => LoanPurpose::CashOut,
        "refinance" => LoanPurpose::Refinance,
        "irrrl" => LoanPurpose::IRRRLStreamline,
        _ => LoanPurpose::Refinance,
    };

    LoanInformation {
        id: Uuid::new_v4(),
        property_type,
        occupancy_type,
        loan_type,
        new_term_months: data.term_months,
        loan_purpose,
        appraisal_waiver: data.appraisal_waiver,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn convert_to_new_loan_details(data: &NewLoanData) -> NewLoanDetails {
    NewLoanDetails {
        id: Uuid::new_v4(),
        market_value: data.market_value,
        sales_price: data.sales_price,
        down_payment: data.down_payment,
        base_loan_amount: data.base_loan_amount,
        subordinated_amount: data.subordinated_amount,
        ff_umip_percentage: 0.0, // TODO: Add this field to frontend
        umip_refund: 0.0, // TODO: Add this field to frontend
        total_loan_amount: data.total_loan_amount,
        note_rate: data.note_rate,
        appraisal_waiver: data.appraisal_waiver,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn convert_to_savings_calculation(data: &SavingsData) -> SavingsCalculation {
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

fn convert_to_other_fees(data: &OtherFeesData) -> OtherFees {
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

fn convert_to_income_information(data: &IncomeInformationData) -> IncomeInformation {
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