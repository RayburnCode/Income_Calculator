use dioxus::prelude::*;
use shared::models::IncomeInformationData;

#[derive(Clone, Debug)]
pub struct IncomeSource {
    pub id: String,
    pub name: String,
    pub monthly_amount: f64,
    pub category: String,
    pub included_in_dti: bool,
}

#[component]
pub fn IncomeInformationSection(data: IncomeInformationData, on_change: EventHandler<IncomeInformationData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Mock income sources - in a real app, this would come from the income worksheet
    let mut available_incomes = use_signal(|| vec![
        IncomeSource {
            id: "w2_jobs".to_string(),
            name: "W-2 Employment".to_string(),
            monthly_amount: 5500.0,
            category: "Employment".to_string(),
            included_in_dti: true,
        },
        IncomeSource {
            id: "commission".to_string(),
            name: "Commission Income".to_string(),
            monthly_amount: 800.0,
            category: "Employment".to_string(),
            included_in_dti: true,
        },
        IncomeSource {
            id: "overtime".to_string(),
            name: "Overtime Pay".to_string(),
            monthly_amount: 600.0,
            category: "Employment".to_string(),
            included_in_dti: false,
        },
        IncomeSource {
            id: "social_security".to_string(),
            name: "Social Security".to_string(),
            monthly_amount: 1200.0,
            category: "Government".to_string(),
            included_in_dti: true,
        },
        IncomeSource {
            id: "pension".to_string(),
            name: "Pension Income".to_string(),
            monthly_amount: 900.0,
            category: "Retirement".to_string(),
            included_in_dti: true,
        },
        IncomeSource {
            id: "rental".to_string(),
            name: "Rental Income".to_string(),
            monthly_amount: 750.0,
            category: "Investment".to_string(),
            included_in_dti: false,
        },
    ]);

    // Calculate total selected income
    let total_selected_income = use_memo(move || {
        available_incomes()
            .iter()
            .filter(|income| income.included_in_dti)
            .map(|income| income.monthly_amount)
            .sum::<f64>()
    });

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    // Update income selection
    let mut update_income_selection = move |income_id: String, included: bool| {
        let mut incomes = available_incomes();
        if let Some(income) = incomes.iter_mut().find(|i| i.id == income_id) {
            income.included_in_dti = included;
        }
        available_incomes.set(incomes);

        // Update the borrower monthly income based on selected incomes
        let total = total_selected_income();
        local_data.write().borrower_monthly_income = total;
        on_change.call(local_data());
    };

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Income Information" }

            // Income Sources Selection
            div { class: "mb-6",
                h5 { class: "text-md font-medium mb-3 text-gray-800",
                    "Select Income Sources for DTI Calculation"
                }
                div { class: "space-y-3 max-h-64 overflow-y-auto border border-gray-200 rounded-lg p-4 bg-gray-50",
                    for income in available_incomes() {
                        div { class: "flex items-center justify-between p-3 bg-white rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors",
                            div { class: "flex items-center space-x-3",
                                input {
                                    r#type: "checkbox",
                                    checked: "{income.included_in_dti}",
                                    onchange: move |e| update_income_selection(income.id.clone(), e.checked()),
                                    class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                }
                                div {
                                    div { class: "font-medium text-gray-900", "{income.name}" }
                                    div { class: "text-sm text-gray-600", "{income.category}" }
                                }
                            }
                            div { class: "text-right",
                                div { class: "font-semibold text-green-700",
                                    "${income.monthly_amount:.0}/mo"
                                }
                                div { class: "text-xs text-gray-500",
                                    if income.included_in_dti {
                                        "Included in DTI"
                                    } else {
                                        "Excluded from DTI"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Summary
            div { class: "bg-gradient-to-r from-green-50 to-emerald-50 p-4 rounded-lg border border-green-200 mb-6",
                div { class: "flex items-center justify-between",
                    div {
                        h5 { class: "text-md font-semibold text-gray-800", "Total Selected Income" }
                        p { class: "text-sm text-gray-600", "Monthly qualifying income for DTI" }
                    }
                    div { class: "text-right",
                        div { class: "text-2xl font-bold text-green-700",
                            "${total_selected_income():.0}"
                        }
                        div { class: "text-sm text-gray-600", "per month" }
                    }
                }
            }

            // Manual Income Override (if needed)
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Manual Borrower Monthly Income:"
                    }
                    input {
                        r#type: "number",
                        step: "0.01",
                        name: "borrowerMonthlyIncome",
                        value: "{local_data().borrower_monthly_income}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().borrower_monthly_income = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        placeholder: "Override auto-calculated amount",
                    }
                    p { class: "text-xs text-gray-500 mt-1", "Leave empty to use selected sources" }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Co-Borrower Monthly Income:"
                    }
                    input {
                        r#type: "number",
                        step: "0.01",
                        name: "coborrowerMonthlyIncome",
                        value: "{local_data().coborrower_monthly_income}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().coborrower_monthly_income = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Front End Ratio (%):" }
                    input {
                        r#type: "number",
                        step: "0.01",
                        name: "frontEndRatio",
                        value: "{local_data().front_end_ratio}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().front_end_ratio = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Back End Ratio (%):" }
                    input {
                        r#type: "number",
                        step: "0.01",
                        name: "backEndRatio",
                        value: "{local_data().back_end_ratio}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                local_data.write().back_end_ratio = val;
                                on_change.call(local_data());
                            }
                        },
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
            }
        }
    }
}