use dioxus::prelude::*;
use crate::views::dashboard::by_id::options_template::*;
use shared::models::*;
use repository::Repository;

#[component]
pub fn OptionsTemplate(id: i32) -> Element {
    // Get client from context
    let client = use_context::<Repository>();

    // Main state for all options template data
    let mut template_data = use_signal(|| OptionsTemplateData::default());

    // Auto-save functionality
    let save_timeout = use_signal(|| None::<i32>);

    // Function to trigger auto-save
    let client_for_save = client.clone();
    let save_timeout_for_save = save_timeout.clone();
    let save_data = move |data: OptionsTemplateData| {
        // Cancel any existing timeout
        if let Some(_timeout_id) = save_timeout_for_save() {
            // In a real implementation, you'd cancel the timeout
            // For now, we'll just update the signal
        }

        // Set new timeout for auto-save (e.g., save after 2 seconds of inactivity)
        // In a real implementation, you'd use a proper timeout mechanism
        // For now, we'll save immediately for demonstration
        save_to_backend(client_for_save.clone(), data, id);
    };

    // Load data on component mount
    use_effect(move || {
        let client_clone = client.clone();
        spawn(async move {
            // Load options template data
            match client_clone.get_options_template(id).await {
                Ok(Some(mut data)) => {
                    // Try to load W2 jobs data from income worksheet
                    match client_clone.get_w2_jobs_data(id).await {
                        Ok(Some(w2_data)) => {
                            data.income_information.w2_jobs_data = Some(w2_data);
                        }
                        Ok(None) => {
                            // No W2 data available, leave as None
                        }
                        Err(e) => {
                            println!("Failed to load W2 jobs data: {:?}", e);
                        }
                    }
                    template_data.set(data);
                }
                Ok(None) => {
                    // No existing template, try to load W2 data for new template
                    match client_clone.get_w2_jobs_data(id).await {
                        Ok(Some(w2_data)) => {
                            let mut data = OptionsTemplateData::default();
                            data.income_information.w2_jobs_data = Some(w2_data);
                            template_data.set(data);
                        }
                        Ok(None) => {
                            // No existing template or W2 data, use defaults
                            println!("No existing options template or W2 data found for borrower {}", id);
                        }
                        Err(e) => {
                            println!("Failed to load W2 jobs data: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to load options template: {:?}", e);
                }
            }
        });
    });

    // Function to save data to backend
    fn save_to_backend(client: Repository, data: OptionsTemplateData, borrower_id: i32) {
        spawn(async move {
            // Save the full options template
            match client.save_options_template(data, borrower_id).await {
                Ok(_) => println!("Successfully saved options template for borrower {}", borrower_id),
                Err(e) => println!("Failed to save options template: {:?}", e),
            }
        });
    }

    rsx! {
        div { class: "max-w-full mx-auto p-4 sm:p-6 text-black",
            h1 { class: "text-2xl sm:text-3xl font-bold text-gray-900 mb-6 sm:mb-8",
                "Mortgage Options Template"
            }

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
                total_loan_amount: template_data().new_loan.total_loan_amount,
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
        }
    }
}