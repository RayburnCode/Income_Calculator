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
    let mut expanded_job = use_signal(|| Some(0));

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
        div { class: "space-y-6",
            // Header with totals
            div { class: "bg-gradient-to-r from-green-50 to-emerald-50 p-6 rounded-xl shadow-md border-2 border-green-200",
                h3 { class: "text-2xl font-bold text-gray-900 mb-4 flex items-center gap-3",
                    span { class: "text-green-600", "🏢" }
                    "W-2 Employment Information"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    div { class: "bg-white p-4 rounded-lg border-2 border-green-300",
                        h4 { class: "text-sm font-semibold text-gray-700 mb-2", "Total Annual Salary" }
                        p { class: "text-2xl font-bold text-green-700", "${total_annual_salary():.2}" }
                    }
                    div { class: "bg-white p-4 rounded-lg border-2 border-green-300",
                        h4 { class: "text-sm font-semibold text-gray-700 mb-2", "Monthly Income" }
                        p { class: "text-2xl font-bold text-green-700",
                            "${total_monthly_income():.2}"
                        }
                    }
                    div { class: "bg-white p-4 rounded-lg border-2 border-green-300",
                        h4 { class: "text-sm font-semibold text-gray-700 mb-2", "Number of Jobs" }
                        p { class: "text-2xl font-bold text-green-700", "{w2_jobs().len()}" }
                    }
                }
            }

            // Job entries
            div { class: "space-y-4",
                for (index , job) in w2_jobs().iter().enumerate() {
                    div { class: "bg-white border-2 border-gray-200 rounded-xl shadow-md overflow-hidden",
                        // Job header
                        div {
                            class: "bg-gradient-to-r from-blue-50 to-indigo-50 px-6 py-4 cursor-pointer hover:bg-blue-100 transition-colors",
                            onclick: move |_| toggle_expanded(index),
                            div { class: "flex items-center justify-between",
                                div { class: "flex items-center gap-3",
                                    span { class: "text-blue-600 text-xl",
                                        if expanded_job() == Some(index) {
                                            "📂"
                                        } else {
                                            "📁"
                                        }
                                    }
                                    h4 { class: "text-lg font-semibold text-gray-900",
                                        if job.employer_name.is_empty() {
                                            "Job #{index + 1}"
                                        } else {
                                            "{job.employer_name}"
                                        }
                                    }
                                    if !job.job_title.is_empty() {
                                        span { class: "text-gray-600 text-sm", " - {job.job_title}" }
                                    }
                                }
                                div { class: "flex items-center gap-3",
                                    if !job.annual_salary.is_empty() {
                                        span { class: "text-green-700 font-semibold",
                                            "${job.annual_salary}/year"
                                        }
                                    }
                                    span { class: "text-gray-500 text-sm",
                                        if expanded_job() == Some(index) {
                                            "▼"
                                        } else {
                                            "▶"
                                        }
                                    }
                                }
                            }
                        }

                        // Job details (expandable)
                        if expanded_job() == Some(index) {
                            div { class: "p-6 border-t border-gray-200",
                                div { class: "space-y-6",
                                    // Employer Information
                                    div { class: "bg-gray-50 p-4 rounded-lg",
                                        h5 { class: "text-md font-semibold text-gray-900 mb-4",
                                            "Employer Information"
                                        }
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            Input {
                                                label: "Employer Name",
                                                placeholder: "Company Name",
                                                value: "{job.employer_name}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "employer_name", evt.value()),
                                            }
                                            Input {
                                                label: "Job Title",
                                                placeholder: "Position/Title",
                                                value: "{job.job_title}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "job_title", evt.value()),
                                            }
                                        }
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mt-4",
                                            Input {
                                                label: "Years Employed",
                                                placeholder: "2",
                                                r#type: "number",
                                                value: "{job.years_employed}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "years_employed", evt.value()),
                                            }
                                            Input {
                                                label: "Months Employed",
                                                placeholder: "6",
                                                r#type: "number",
                                                value: "{job.months_employed}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "months_employed", evt.value()),
                                            }
                                        }
                                    }

                                    // Salary Information
                                    div { class: "bg-green-50 p-4 rounded-lg",
                                        h5 { class: "text-md font-semibold text-gray-900 mb-4",
                                            "Salary Information"
                                        }
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            Input {
                                                label: "Annual Base Salary",
                                                placeholder: "75000",
                                                r#type: "number",
                                                value: "{job.annual_salary}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "annual_salary", evt.value()),
                                            }
                                            Input {
                                                label: "Hourly Rate (if applicable)",
                                                placeholder: "25.00",
                                                r#type: "number",
                                                value: "{job.hourly_rate}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "hourly_rate", evt.value()),
                                            }
                                        }
                                        if !job.hourly_rate.is_empty() {
                                            div { class: "mt-4",
                                                Input {
                                                    label: "Hours per Week",
                                                    placeholder: "40",
                                                    r#type: "number",
                                                    value: "{job.hours_per_week}",
                                                    oninput: move |evt: Event<FormData>| update_job(index, "hours_per_week", evt.value()),
                                                }
                                            }
                                        }
                                    }

                                    // Additional Compensation
                                    div { class: "bg-yellow-50 p-4 rounded-lg",
                                        h5 { class: "text-md font-semibold text-gray-900 mb-4",
                                            "Additional Compensation"
                                        }
                                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                            Input {
                                                label: "Monthly Commission",
                                                placeholder: "500.00",
                                                r#type: "number",
                                                value: "{job.commission_monthly}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "commission_monthly", evt.value()),
                                            }
                                            Input {
                                                label: "Monthly Bonus",
                                                placeholder: "200.00",
                                                r#type: "number",
                                                value: "{job.bonus_monthly}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "bonus_monthly", evt.value()),
                                            }
                                            Input {
                                                label: "Monthly Overtime",
                                                placeholder: "150.00",
                                                r#type: "number",
                                                value: "{job.overtime_monthly}",
                                                oninput: move |evt: Event<FormData>| update_job(index, "overtime_monthly", evt.value()),
                                            }
                                        }
                                    }

                                    // Job actions
                                    div { class: "flex justify-between items-center pt-4 border-t border-gray-200",
                                        if w2_jobs().len() > 1 {
                                            button {
                                                class: "bg-red-500 hover:bg-red-700 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                                                onclick: move |_| remove_job(index),
                                                "Remove Job"
                                            }
                                        } else {
                                            div {}
                                        }
                                        div { class: "text-sm text-gray-600",
                                            "Job #{index + 1} of {w2_jobs().len()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Add job button
            div { class: "text-center",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg shadow-md transition-colors flex items-center gap-2 mx-auto",
                    onclick: add_job,
                    span { "➕" }
                    "Add Another W-2 Job"
                }
            }

            // Summary section
            div { class: "bg-gradient-to-r from-indigo-50 to-purple-50 p-6 rounded-xl shadow-md border-2 border-indigo-200 mt-8",
                h4 { class: "text-xl font-bold text-gray-900 mb-4", "Income Summary" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "bg-white p-4 rounded-lg",
                        h5 { class: "font-semibold text-gray-900 mb-2",
                            "Total Monthly Qualifying Income"
                        }
                        p { class: "text-3xl font-bold text-green-700",
                            "${total_monthly_income():.2}"
                        }
                        p { class: "text-sm text-gray-600 mt-1",
                            "Based on Fannie Mae/Freddie Mac guidelines"
                        }
                    }
                    div { class: "bg-white p-4 rounded-lg",
                        h5 { class: "font-semibold text-gray-900 mb-2", "Annual Income" }
                        p { class: "text-3xl font-bold text-blue-700", "${total_annual_salary():.2}" }
                        p { class: "text-sm text-gray-600 mt-1", "Base salary only" }
                    }
                }
            }
        }
    }
}