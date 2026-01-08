use dioxus::prelude::*;
use crate::components::Input;
use client;
use shared::models::{W2Job, W2JobsData};

#[component]
pub fn W2Jobs(borrower_id: i32) -> Element {
    let mut w2_data = use_signal(|| W2JobsData::default());
    let mut expanded_job = use_signal(|| None);

    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        repository::Repository::new().await
    });

    // Load W2 jobs data when component mounts
    use_effect(move || {
        let resource_value = client_resource.read().clone();
        let mut w2_data = w2_data.clone();
        let client_id = borrower_id;
        
        spawn(async move {
            match resource_value.as_ref() {
                Some(Ok(db_client)) => {
                    // Load W2 jobs from database
                    match db_client.get_w2_jobs_data(client_id).await {
                        Ok(Some(data)) => {
                            w2_data.set(data);
                        }
                        Ok(None) => {
                            // No data, keep default
                        }
                        Err(e) => {
                            // For now, just ignore error. In production, show error message
                            tracing::error!("Error loading W2 jobs: {:?}", e); 
                        }
                    }
                }
                Some(Err(e)) => {
                    tracing::error!("Database connection error: {:?}", e);
                }
                None => {
                    // Still loading
                }
            }
        });
    });

    // Calculate totals
    let total_annual_salary = use_memo(move || {
        w2_data().jobs.iter()
            .map(|job| job.annual_salary.parse::<f64>().unwrap_or(0.0))
            .sum::<f64>()
    });

    let total_monthly_income = use_memo(move || {
        w2_data().jobs.iter()
            .map(|job| {
                let salary = job.annual_salary.parse::<f64>().unwrap_or(0.0) / 12.0;
                let commission = job.commission_monthly.parse::<f64>().unwrap_or(0.0);
                let bonus = job.bonus_monthly.parse::<f64>().unwrap_or(0.0);
                let overtime = job.overtime_monthly.parse::<f64>().unwrap_or(0.0);
                salary + commission + bonus + overtime
            })
            .sum::<f64>()
    });

    let add_job = move |_| {
        let mut data = w2_data();
        data.jobs.push(W2Job::default());
        let new_index = data.jobs.len() - 1;
        w2_data.set(data);
        expanded_job.set(Some(new_index));

        // Auto-save to database
        let client_resource = client_resource.clone();
        let current_data = w2_data();
        let client_id = borrower_id;
        spawn(async move {
            if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                if let Err(e) = db_client.save_w2_jobs_data(client_id, &current_data).await {
                    tracing::error!("Error saving W2 jobs: {:?}", e);
                }
            }
        });
    };

    let mut remove_job = move |index: usize| {
        let mut data = w2_data();
        if data.jobs.len() > 1 {
            data.jobs.remove(index);
            let new_len = data.jobs.len();
            let should_reset_expanded = expanded_job() == Some(index) ||
                (expanded_job().is_some() && expanded_job().unwrap() > index);

            w2_data.set(data);

            if should_reset_expanded {
                expanded_job.set(if new_len == 0 { None } else { Some(0) });
            } else if let Some(expanded) = expanded_job() {
                if expanded > index {
                    expanded_job.set(Some(expanded - 1));
                }
            }

            // Auto-save to database
            let client_resource = client_resource.clone();
            let current_data = w2_data();
            let client_id = borrower_id;
            spawn(async move {
                if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                    if let Err(e) = db_client.save_w2_jobs_data(client_id, &current_data).await {
                        tracing::error!("Error saving W2 jobs: {:?}", e);
                    }
                }
            });
        }
    };

    let mut update_job = move |index: usize, field: &str, value: String| {
        let mut data = w2_data();
        if let Some(job) = data.jobs.get_mut(index) {
            match field {
                "employer_name" => job.employer_name = value,
                "job_title" => job.job_title = value,
                "years_employed" => job.years_employed = value,
                "months_employed" => job.months_employed = value,
                "annual_salary" => job.annual_salary = value,
                "hourly_rate" => job.hourly_rate = value,
                "hours_per_week" => job.hours_per_week = value,
                "commission_monthly" => job.commission_monthly = value,
                "bonus_monthly" => job.bonus_monthly = value,
                "overtime_monthly" => job.overtime_monthly = value,
                _ => {}
            }
        }
        w2_data.set(data);

        // Auto-save to database
        let client_resource = client_resource.clone();
        let current_data = w2_data();
        let client_id = borrower_id;
        spawn(async move {
            if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                if let Err(e) = db_client.save_w2_jobs_data(client_id, &current_data).await {
                    tracing::error!("Error saving W2 jobs: {:?}", e);
                }
            }
        });
    };

    let toggle_verified = move |_| {
        let mut data = w2_data();
        data.is_verified = !data.is_verified;
        if data.is_verified {
            data.verified_at = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        } else {
            data.verified_at = None;
        }
        w2_data.set(data);

        // Auto-save to database
        let client_resource = client_resource.clone();
        let current_data = w2_data();
        let client_id = borrower_id;
        spawn(async move {
            if let Some(Ok(db_client)) = client_resource.read().as_ref() {
                if let Err(e) = db_client.save_w2_jobs_data(client_id, &current_data).await {
                    tracing::error!("Error saving W2 jobs: {:?}", e);
                }
            }
        });
    };

    let mut toggle_expanded = move |index: usize| {
        expanded_job.set(if expanded_job() == Some(index) { None } else { Some(index) });
    };

    rsx! {
        div { class: "space-y-4",
            // Compact Header with totals
            div { class: "bg-gradient-to-r from-green-50 to-emerald-50 p-4 rounded-lg shadow-sm border border-green-200",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-lg font-bold text-gray-900 flex items-center gap-2",
                        span { class: "text-green-600", "🏢" }
                        "W-2 Jobs ({w2_data().jobs.len()})"
                        if w2_data().is_verified {
                            span { class: "text-green-600 text-sm ml-2", "✓ Verified" }
                        }
                    }
                    div { class: "flex items-center gap-4",
                        div { class: "text-right",
                            div { class: "text-sm text-gray-600",
                                "Total Annual: ${total_annual_salary():.0}"
                            }
                            div { class: "text-sm font-semibold text-green-700",
                                "Monthly: ${total_monthly_income():.0}"
                            }
                        }
                        button {
                            class: if w2_data().is_verified { "bg-green-500 hover:bg-green-600" } else { "bg-gray-500 hover:bg-gray-600" },
                            class: "text-white text-sm py-1 px-3 rounded transition-colors flex items-center gap-1",
                            onclick: toggle_verified,
                            if w2_data().is_verified {
                                span { "✓" }
                                "Verified"
                            } else {
                                span { "○" }
                                "Mark Verified"
                            }
                        }
                    }
                }
            }

            // Job entries - more compact
            div { class: "space-y-2",
                for (index , job) in w2_data().jobs.iter().enumerate() {
                    div { class: "bg-white border border-gray-200 rounded-lg shadow-sm overflow-hidden",
                        // Compact job header
                        div {
                            class: "px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors",
                            onclick: move |_| toggle_expanded(index),
                            div { class: "flex items-center justify-between",
                                div { class: "flex items-center gap-2 flex-1",
                                    span { class: "text-blue-600",
                                        if expanded_job() == Some(index) {
                                            "📂"
                                        } else {
                                            "📁"
                                        }
                                    }
                                    span { class: "font-medium text-gray-900",
                                        if job.employer_name.is_empty() {
                                            "Job #{index + 1}"
                                        } else {
                                            "{job.employer_name}"
                                        }
                                    }
                                    if !job.job_title.is_empty() {
                                        span { class: "text-gray-500 text-sm", "• {job.job_title}" }
                                    }
                                }
                                div { class: "flex items-center gap-3 text-sm",
                                    if !job.annual_salary.is_empty() {
                                        span { class: "text-green-700 font-semibold",
                                            "${job.annual_salary}"
                                        }
                                    }
                                    span { class: "text-gray-400",
                                        if expanded_job() == Some(index) {
                                            "▼"
                                        } else {
                                            "▶"
                                        }
                                    }
                                }
                            }
                        }

                        // Compact job details (expandable)
                        if expanded_job() == Some(index) {
                            div { class: "px-4 py-3 border-t border-gray-200 bg-gray-50",
                                div { class: "space-y-3",
                                    // Employer Information - compact
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                        Input {
                                            label: "Employer",
                                            placeholder: "Company Name",
                                            value: "{job.employer_name}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "employer_name", evt.value()),
                                        }
                                        Input {
                                            label: "Job Title",
                                            placeholder: "Position",
                                            value: "{job.job_title}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "job_title", evt.value()),
                                        }
                                    }
                                    div { class: "grid grid-cols-2 gap-3",
                                        Input {
                                            label: "Years",
                                            placeholder: "2",
                                            r#type: "number",
                                            value: "{job.years_employed}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "years_employed", evt.value()),
                                        }
                                        Input {
                                            label: "Months",
                                            placeholder: "6",
                                            r#type: "number",
                                            value: "{job.months_employed}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "months_employed", evt.value()),
                                        }
                                    }

                                    // Salary Information - compact
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                        Input {
                                            label: "Annual Salary",
                                            placeholder: "75000",
                                            r#type: "number",
                                            value: "{job.annual_salary}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "annual_salary", evt.value()),
                                        }
                                        Input {
                                            label: "Hourly Rate",
                                            placeholder: "25.00",
                                            r#type: "number",
                                            value: "{job.hourly_rate}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "hourly_rate", evt.value()),
                                        }
                                    }
                                    if !job.hourly_rate.is_empty() {
                                        div { class: "md:col-span-2",
                                            Input {
                                                label: "Hours/Week",
                                                placeholder: "40",
                                                r#type: "number",
                                                value: "{job.hours_per_week}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "hours_per_week", evt.value()),
                                            }
                                        }
                                    }

                                    // Additional Compensation - compact
                                    div { class: "grid grid-cols-3 gap-3",
                                        Input {
                                            label: "Monthly Commission",
                                            placeholder: "500",
                                            r#type: "number",
                                            value: "{job.commission_monthly}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "commission_monthly", evt.value()),
                                        }
                                        Input {
                                            label: "Monthly Bonus",
                                            placeholder: "200",
                                            r#type: "number",
                                            value: "{job.bonus_monthly}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "bonus_monthly", evt.value()),
                                        }
                                        Input {
                                            label: "Monthly Overtime",
                                            placeholder: "150",
                                            r#type: "number",
                                            value: "{job.overtime_monthly}",
                                            oninput: move |evt: Event<FormData>| update_job(index, "overtime_monthly", evt.value()),
                                        }
                                    }

                                    // Job actions - compact
                                    div { class: "flex justify-between items-center pt-3 border-t border-gray-200",
                                        if w2_data().jobs.len() > 1 {
                                            button {
                                                class: "bg-red-500 hover:bg-red-600 text-white text-sm py-1 px-3 rounded transition-colors",
                                                onclick: move |_| remove_job(index),
                                                "Remove"
                                            }
                                        } else {
                                            div {}
                                        }
                                        div { class: "text-xs text-gray-500", "Job {index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Compact Add job button
            div { class: "text-center py-2",
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white text-sm py-2 px-4 rounded-lg shadow-sm transition-colors flex items-center gap-2 mx-auto",
                    onclick: add_job,
                    span { "➕" }
                    "Add Job"
                }
            }

            // Compact Summary section
            div { class: "bg-gray-50 p-3 rounded-lg border border-gray-200",
                div { class: "text-center",
                    div { class: "text-sm text-gray-600 mb-1", "Total Monthly Qualifying Income" }
                    div { class: "text-xl font-bold text-green-700", "${total_monthly_income():.0}" }
                }
            }
        }
    }
}