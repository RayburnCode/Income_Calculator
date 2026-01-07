use dioxus::prelude::*;
use crate::components::Input;

#[derive(Clone, Debug, PartialEq)]
pub struct W2Job {
    pub employer_name: String,
    pub job_title: String,
    pub years_employed: String,
    pub months_employed: String,
    pub annual_salary: String,
    pub hourly_rate: String,
    pub hours_per_week: String,
    pub commission_monthly: String,
    pub bonus_monthly: String,
    pub overtime_monthly: String,
}

impl Default for W2Job {
    fn default() -> Self {
        Self {
            employer_name: String::new(),
            job_title: String::new(),
            years_employed: String::new(),
            months_employed: String::new(),
            annual_salary: String::new(),
            hourly_rate: String::new(),
            hours_per_week: String::new(),
            commission_monthly: String::new(),
            bonus_monthly: String::new(),
            overtime_monthly: String::new(),
        }
    }
}

#[component]
pub fn W2Jobs() -> Element {
    let mut w2_jobs = use_signal(|| vec![W2Job::default()]);
    let mut expanded_job = use_signal(|| None);

    // Calculate totals
    let total_annual_salary = use_memo(move || {
        w2_jobs().iter()
            .map(|job| job.annual_salary.parse::<f64>().unwrap_or(0.0))
            .sum::<f64>()
    });

    let total_monthly_income = use_memo(move || {
        w2_jobs().iter()
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
        let mut jobs = w2_jobs();
        jobs.push(W2Job::default());
        let new_index = jobs.len() - 1;
        w2_jobs.set(jobs);
        expanded_job.set(Some(new_index));
    };

    let mut remove_job = move |index: usize| {
        let mut jobs = w2_jobs();
        if jobs.len() > 1 {
            jobs.remove(index);
            let new_len = jobs.len();
            let should_reset_expanded = expanded_job() == Some(index) ||
                (expanded_job().is_some() && expanded_job().unwrap() > index);

            w2_jobs.set(jobs);

            if should_reset_expanded {
                expanded_job.set(if new_len == 0 { None } else { Some(0) });
            } else if let Some(expanded) = expanded_job() {
                if expanded > index {
                    expanded_job.set(Some(expanded - 1));
                }
            }
        }
    };

    let mut update_job = move |index: usize, field: &str, value: String| {
        let mut jobs = w2_jobs();
        if let Some(job) = jobs.get_mut(index) {
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
        w2_jobs.set(jobs);
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
                        "W-2 Jobs ({w2_jobs().len()})"
                    }
                    div { class: "text-right",
                        div { class: "text-sm text-gray-600", "Total Annual: ${total_annual_salary():.0}" }
                        div { class: "text-sm font-semibold text-green-700", "Monthly: ${total_monthly_income():.0}" }
                    }
                }
            }

            // Job entries - more compact
            div { class: "space-y-2",
                for (index , job) in w2_jobs().iter().enumerate() {
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
                                        if w2_jobs().len() > 1 {
                                            button {
                                                class: "bg-red-500 hover:bg-red-600 text-white text-sm py-1 px-3 rounded transition-colors",
                                                onclick: move |_| remove_job(index),
                                                "Remove"
                                            }
                                        } else {
                                            div {}
                                        }
                                        div { class: "text-xs text-gray-500",
                                            "Job {index + 1}"
                                        }
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