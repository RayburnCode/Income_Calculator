use dioxus::prelude::*;
use shared::models::{IncomeInformationData, IncomeSource};

#[component]
pub fn IncomeInformationSection(data: IncomeInformationData, on_change: EventHandler<IncomeInformationData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Calculate income from W2 jobs if available
    let w2_income_sources = use_memo(move || {
        if let Some(w2_data) = &local_data().w2_jobs_data {
            w2_data.jobs.iter().enumerate().map(|(index, job)| {
                let annual_salary = job.annual_salary.parse::<f64>().unwrap_or(0.0);
                let monthly_salary = annual_salary / 12.0;
                let commission = job.commission_monthly.parse::<f64>().unwrap_or(0.0);
                let bonus = job.bonus_monthly.parse::<f64>().unwrap_or(0.0);
                let overtime = job.overtime_monthly.parse::<f64>().unwrap_or(0.0);
                let total_monthly = monthly_salary + commission + bonus + overtime;

                IncomeSource {
                    id: format!("w2_job_{}", index),
                    name: if job.employer_name.is_empty() {
                        format!("Job #{}", index + 1)
                    } else {
                        format!("{} - {}", job.employer_name, job.job_title)
                    },
                    monthly_amount: total_monthly,
                    category: "Employment".to_string(),
                    included_in_dti: true,
                }
            }).collect::<Vec<_>>()
        } else {
            // No fallback mock data - show empty if no W2 data available
            Vec::new()
        }
    });

    let total_monthly_income = use_memo(move || {
        w2_income_sources().iter()
            .filter(|source| source.included_in_dti)
            .map(|source| source.monthly_amount)
            .sum::<f64>()
    });

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    // Update income selection
    let mut update_income_selection = move |income_id: String, included: bool| {
        // For now, this is just for display - actual income comes from W2 jobs
        // In the future, this could allow excluding certain income sources
        println!("Income selection updated: {} -> {}", income_id, included);
    };

    // Update borrower income
    let mut update_borrower_income = move |value: f64| {
        let mut data = local_data();
        data.borrower_monthly_income = value;
        local_data.set(data.clone());
        on_change.call(data);
    };

    // Update coborrower income
    let mut update_coborrower_income = move |value: f64| {
        let mut data = local_data();
        data.coborrower_monthly_income = value;
        local_data.set(data.clone());
        on_change.call(data);
    };

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Income Information" }

            // Income Sources from W2 Jobs
            div { class: "mb-6",
                h5 { class: "text-md font-medium mb-3 text-gray-800 flex items-center gap-2",
                    "Income Sources"
                    if let Some(w2_data) = &local_data().w2_jobs_data {
                        if w2_data.is_verified {
                            span { class: "text-green-600 text-sm", "✓ Verified" }
                        }
                    }
                }
                if w2_income_sources().is_empty() {
                    div { class: "text-center py-8 text-gray-500 border border-gray-200 rounded-lg bg-gray-50",
                        div { class: "text-lg mb-2", "📊" }
                        div { class: "font-medium mb-1", "No Income Sources Found" }
                        div { class: "text-sm",
                            "Add income information in the Client Details → Income Worksheet tab"
                        }
                    }
                } else {
                    div { class: "space-y-3 max-h-64 overflow-y-auto border border-gray-200 rounded-lg p-4 bg-gray-50",
                        for income in w2_income_sources() {
                            div { class: "flex items-center justify-between p-3 bg-white rounded-lg border border-gray-200",
                                div { class: "flex items-center space-x-3",
                                    div { class: "w-4 h-4 rounded-full bg-green-500 flex items-center justify-center",
                                        span { class: "text-white text-xs", "✓" }
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
                                    div { class: "text-xs text-green-600 font-medium",
                                        "Included in DTI"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Manual Income Entry (for additional income sources)
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-6",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "Borrower Monthly Income"
                    }
                    input {
                        r#type: "number",
                        value: "{local_data().borrower_monthly_income}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                update_borrower_income(val);
                            }
                        },
                        placeholder: "0.00",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "Co-Borrower Monthly Income"
                    }
                    input {
                        r#type: "number",
                        value: "{local_data().coborrower_monthly_income}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f64>() {
                                update_coborrower_income(val);
                            }
                        },
                        placeholder: "0.00",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                    }
                }
            }

            // Summary
            div { class: "bg-gradient-to-r from-green-50 to-emerald-50 p-4 rounded-lg border border-green-200 mb-6",
                div { class: "flex items-center justify-between",
                    div {
                        h5 { class: "text-md font-semibold text-gray-800",
                            "Total Monthly Qualifying Income"
                        }
                        p { class: "text-sm text-gray-600", "From W-2 jobs + additional income" }
                    }
                    div { class: "text-right",
                        div { class: "text-2xl font-bold text-green-700",
                            "${total_monthly_income() + local_data().borrower_monthly_income + local_data().coborrower_monthly_income:.0}"
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