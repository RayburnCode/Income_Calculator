use dioxus::prelude::*;
use crate::components::Input;
use shared::models::{GeneralIncomeData, GeneralIncomeEntry};
use shared::models::enums::IncomeType;

#[component]
pub fn GeneralIncome(borrower_id: i32) -> Element {
    let mut income_data = use_signal(|| GeneralIncomeData::default());
    let mut expanded_entry = use_signal(|| None);

    // Get the database client from context or create it
    let client_resource = use_resource(|| async {
        repository::Repository::new().await
    });

    // Load general income data when component mounts
    use_effect(move || {
        let resource_value = client_resource.read().clone();
        let  income_data = income_data.clone();
        let client_id = borrower_id;

        spawn(async move {
            match resource_value.as_ref() {
                Some(Ok(db_client)) => {
                    // For now, we'll initialize with default data
                    // In the future, this would load from database
                    // match db_client.get_general_income_data(client_id).await {
                    //     Ok(Some(data)) => {
                    //         income_data.set(data);
                    //     }
                    //     Ok(None) => {
                    //         // No data, keep default
                    //     }
                    //     Err(e) => {
                    //         tracing::error!("Error loading general income: {:?}", e);
                    //     }
                    // }
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
    let total_monthly_income = use_memo(move || {
        income_data().entries.iter()
            .filter_map(|entry| entry.monthly_amount.parse::<f64>().ok())
            .sum::<f64>()
    });

    let total_annual_income = use_memo(move || {
        income_data().entries.iter()
            .filter_map(|entry| entry.annual_amount.parse::<f64>().ok())
            .sum::<f64>()
    });

    let add_income_entry = move |_| {
        let mut data = income_data();
        data.entries.push(GeneralIncomeEntry::default());
        let new_index = data.entries.len() - 1;
        income_data.set(data);
        expanded_entry.set(Some(new_index));

        // TODO: Auto-save to database
    };

    let mut remove_income_entry = move |index: usize| {
        let mut data = income_data();
        if data.entries.len() > 1 {
            data.entries.remove(index);
            let new_len = data.entries.len();
            let should_reset_expanded = expanded_entry() == Some(index) ||
                (expanded_entry().is_some() && expanded_entry().unwrap() > index);

            income_data.set(data);

            if should_reset_expanded {
                expanded_entry.set(if new_len == 0 { None } else { Some(0) });
            } else if let Some(expanded) = expanded_entry() {
                if expanded > index {
                    expanded_entry.set(Some(expanded - 1));
                }
            }

            // TODO: Auto-save to database
        }
    };

    let mut update_income_entry = move |index: usize, field: &str, value: String| {
        let mut data = income_data();
        if let Some(entry) = data.entries.get_mut(index) {
            match field {
                "income_type" => entry.income_type = value,
                "source_name" => entry.source_name = value,
                "description" => entry.description = value,
                "monthly_amount" => entry.monthly_amount = value,
                "annual_amount" => entry.annual_amount = value,
                "notes" => entry.notes = value,
                _ => {}
            }
        }
        income_data.set(data);

        // TODO: Auto-save to database
    };

    let toggle_verified = move |_| {
        let mut data = income_data();
        data.is_verified = !data.is_verified;
        if data.is_verified {
            data.verified_at = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        } else {
            data.verified_at = None;
        }
        income_data.set(data);

        // TODO: Auto-save to database
    };

    let mut toggle_expanded = move |index: usize| {
        expanded_entry.set(if expanded_entry() == Some(index) { None } else { Some(index) });
    };


    rsx! {
        div { class: "space-y-4",
            // Header with totals
            div { class: "bg-gradient-to-r from-green-50 to-emerald-50 p-4 rounded-lg shadow-sm border border-green-200",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-lg font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2",
                        span { class: "text-green-600", "💰" }
                        "Income Sources ({income_data().entries.len()})"
                        if income_data().is_verified {
                            span { class: "text-green-600 text-sm ml-2", "✓ Verified" }
                        }
                    }
                    div { class: "flex items-center gap-4",
                        div { class: "text-right",
                            div { class: "text-sm text-gray-600",
                                "Total Annual: ${total_annual_income():.0}"
                            }
                            div { class: "text-sm font-semibold text-green-700",
                                "Monthly: ${total_monthly_income():.0}"
                            }
                        }
                        button {
                            class: if income_data().is_verified { "bg-green-500 hover:bg-green-600" } else { "bg-gray-500 hover:bg-gray-600" },
                            class: "text-white text-sm py-1 px-3 rounded transition-colors flex items-center gap-1",
                            onclick: toggle_verified,
                            if income_data().is_verified {
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

            // Income entries
            div { class: "space-y-2",
                for (index , entry) in income_data().entries.iter().enumerate() {
                    div { class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm overflow-hidden",
                        // Entry header
                        div {
                            class: "px-4 py-3 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                            onclick: move |_| toggle_expanded(index),
                            div { class: "flex items-center justify-between",
                                div { class: "flex items-center gap-2 flex-1",
                                    span { class: "text-blue-600",
                                        if expanded_entry() == Some(index) {
                                            "📂"
                                        } else {
                                            "📁"
                                        }
                                    }
                                    span { class: "font-medium text-gray-900 dark:text-gray-100",
                                        if entry.source_name.is_empty() {
                                            "{entry.income_type} #{index + 1}"
                                        } else {
                                            "{entry.source_name}"
                                        }
                                    }
                                    if !entry.description.is_empty() {
                                        span { class: "text-gray-500 dark:text-gray-400 text-sm",
                                            "• {entry.description}"
                                        }
                                    }
                                }
                                div { class: "flex items-center gap-3 text-sm",
                                    if !entry.monthly_amount.is_empty() {
                                        span { class: "text-green-700 dark:text-green-400 font-semibold",
                                            "${entry.monthly_amount}/mo"
                                        }
                                    }
                                    if !entry.annual_amount.is_empty() {
                                        span { class: "text-blue-600 dark:text-blue-400",
                                            "${entry.annual_amount}/yr"
                                        }
                                    }
                                    span { class: "text-gray-400 dark:text-gray-500",
                                        if expanded_entry() == Some(index) {
                                            "▼"
                                        } else {
                                            "▶"
                                        }
                                    }
                                }
                            }
                        }

                        // Entry details (expandable)
                        if expanded_entry() == Some(index) {
                            div { class: "px-4 py-3 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700",
                                div { class: "space-y-3",
                                    // Income Type Selection
                                    div { class: "md:col-span-2",
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Income Type"
                                        }
                                        select {
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100",
                                            value: "{entry.income_type}",
                                            onchange: move |evt| update_income_entry(index, "income_type", evt.value()),
                                            for income_type in vec![
                                                "W-2 Employment",
                                                "Self Employment",
                                                "Rental Income",
                                                "Investment Income",
                                                "Social Security",
                                                "Pension",
                                                "Disability",
                                                "Alimony",
                                                "Child Support",
                                                "Other",
                                            ]
                                            {
                                                option { value: "{income_type}", "{income_type}" }
                                            }
                                        }
                                    }

                                    // Source and Description
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                        Input {
                                            label: "Source Name",
                                            placeholder: "Employer, Property Address, etc.",
                                            value: "{entry.source_name}",
                                            oninput: move |evt: Event<FormData>| update_income_entry(index, "source_name", evt.value()),
                                        }
                                        Input {
                                            label: "Description",
                                            placeholder: "Job title, investment type, etc.",
                                            value: "{entry.description}",
                                            oninput: move |evt: Event<FormData>| update_income_entry(index, "description", evt.value()),
                                        }
                                    }

                                    // Amount fields
                                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                        Input {
                                            label: "Monthly Amount",
                                            placeholder: "2500.00",
                                            r#type: "number",
                                            value: "{entry.monthly_amount}",
                                            oninput: move |evt: Event<FormData>| update_income_entry(
                                                index,
                                                "monthly_amount",
                                                evt.value(),
                                            ),
                                        }
                                        Input {
                                            label: "Annual Amount",
                                            placeholder: "30000.00",
                                            r#type: "number",
                                            value: "{entry.annual_amount}",
                                            oninput: move |evt: Event<FormData>| update_income_entry(index, "annual_amount", evt.value()),
                                        }
                                    }

                                    // Notes
                                    div { class: "md:col-span-2",
                                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Notes"
                                        }
                                        textarea {
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 resize-vertical",
                                            placeholder: "Additional notes about this income source...",
                                            rows: "2",
                                            value: "{entry.notes}",
                                            oninput: move |evt| update_income_entry(index, "notes", evt.value()),
                                        }
                                    }

                                    // Entry actions
                                    div { class: "flex justify-between items-center pt-3 border-t border-gray-200 dark:border-gray-700",
                                        if income_data().entries.len() > 1 {
                                            button {
                                                class: "bg-red-500 hover:bg-red-600 text-white text-sm py-1 px-3 rounded transition-colors",
                                                onclick: move |_| remove_income_entry(index),
                                                "Remove"
                                            }
                                        } else {
                                            div {}
                                        }
                                        div { class: "text-xs text-gray-500 dark:text-gray-400",
                                            "Entry {index + 1}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Add income entry button
            div { class: "text-center py-2",
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white text-sm py-2 px-4 rounded-lg shadow-sm transition-colors flex items-center gap-2 mx-auto",
                    onclick: add_income_entry,
                    span { "➕" }
                    "Add Income Source"
                }
            }

            // Summary section
            div { class: "bg-gray-50 dark:bg-gray-700 p-3 rounded-lg border border-gray-200 dark:border-gray-700",
                div { class: "text-center",
                    div { class: "text-sm text-gray-600 dark:text-gray-300 mb-1",
                        "Total Monthly Qualifying Income"
                    }
                    div { class: "text-xl font-bold text-green-700 dark:text-green-400",
                        "${total_monthly_income():.0}"
                    }
                }
            }
        }
    }
}