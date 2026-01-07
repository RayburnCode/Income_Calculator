use dioxus::prelude::*;
use shared::models::AppSettings;

#[component]
pub fn Settings() -> Element {
    let client = use_context::<client::Client>();
    let client_clone = client.clone();
    let mut settings = use_signal(|| AppSettings::default());
    let mut is_loading = use_signal(|| true);

    // Load settings on component mount
    use_effect(move || {
        let client = client_clone.clone();
        spawn(async move {
            match client.get_settings().await {
                Ok(loaded_settings) => {
                    settings.set(loaded_settings);
                    is_loading.set(false);
                }
                Err(e) => {
                    log::error!("Failed to load settings: {:?}", e);
                    is_loading.set(false);
                }
            }
        });
    });

    let save_settings = move |_| {
        let client = client.clone();
        spawn(async move {
            let current_settings = settings();
            match client.save_settings(current_settings).await {
                Ok(_) => {
                    log::info!("Settings saved successfully");
                    // Could show a success message here
                }
                Err(e) => {
                    log::error!("Failed to save settings: {:?}", e);
                    // Could show an error message here
                }
            }
        });
    };
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-4xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Settings" }
                    p { class: "text-gray-600 mt-2", "Configure your Income Calculator preferences" }
                }

                if is_loading() {
                    div { class: "flex justify-center items-center py-12",
                        div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" }
                        span { class: "ml-2 text-gray-600", "Loading settings..." }
                    }
                } else {
                    // Settings sections
                    div { class: "space-y-6",

                        // Application Settings
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                                "Application Settings"
                            }
                            div { class: "space-y-4",
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Application Theme"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Choose your preferred theme"
                                        }
                                    }
                                    select {
                                        class: "border border-gray-300 rounded-md px-3 py-2",
                                        value: "{settings().theme}",
                                        onchange: move |e| {
                                            settings.write().theme = e.value().clone();
                                        },
                                        option { value: "light", "Light" }
                                        option { value: "dark", "Dark" }
                                        option { value: "system", "System" }
                                    }
                                }
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Default Currency Display"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "How currency values are formatted"
                                        }
                                    }
                                    select {
                                        class: "border border-gray-300 rounded-md px-3 py-2",
                                        value: "{settings().currency}",
                                        onchange: move |e| {
                                            settings.write().currency = e.value().clone();
                                        },
                                        option { value: "USD ($)", "USD ($)" }
                                        option { value: "EUR (€)", "EUR (€)" }
                                        option { value: "GBP (£)", "GBP (£)" }
                                    }
                                }
                            }
                        }

                        // Data Management
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                                "Data Management"
                            }
                            div { class: "space-y-4",
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Database Location"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Current: income_calculator.db (local SQLite database)"
                                        }
                                    }
                                    button { class: "bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 text-sm",
                                        "Browse..."
                                    }
                                }
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Auto-Backup"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Automatically backup data on application close"
                                        }
                                    }
                                    input {
                                        r#type: "checkbox",
                                        checked: "{settings().auto_backup}",
                                        onchange: move |e| {
                                            settings.write().auto_backup = e.checked();
                                        },
                                        class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                    }
                                }
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Export Data"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Export all client and calculation data"
                                        }
                                    }
                                    button { class: "bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 text-sm",
                                        "Export"
                                    }
                                }
                            }
                        }

                        // Calculation Defaults
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                                "Calculation Defaults"
                            }
                            div { class: "space-y-4",
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "Default Loan Term"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Default term for new loan calculations"
                                        }
                                    }
                                    select {
                                        class: "border border-gray-300 rounded-md px-3 py-2",
                                        value: "{settings().default_loan_term}",
                                        onchange: move |e| {
                                            if let Ok(term) = e.value().parse::<i32>() {
                                                settings.write().default_loan_term = term;
                                            }
                                        },
                                        option { value: "15", "15 years" }
                                        option { value: "20", "20 years" }
                                        option { value: "30", "30 years" }
                                    }
                                }
                                div { class: "flex items-center justify-between",
                                    div {
                                        label { class: "text-sm font-medium text-gray-700",
                                            "DTI Threshold Warning"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Warn when DTI exceeds this percentage"
                                        }
                                    }
                                    input {
                                        r#type: "number",
                                        step: "0.1",
                                        value: "{settings().dti_threshold}",
                                        onchange: move |e| {
                                            if let Ok(threshold) = e.value().parse::<f64>() {
                                                settings.write().dti_threshold = threshold;
                                            }
                                        },
                                        class: "border border-gray-300 rounded-md px-3 py-2 w-20",
                                    }
                                    span { class: "text-sm text-gray-500 ml-2", "%" }
                                }
                            }
                        }

                        // About & Support
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                                "About & Support"
                            }
                            div { class: "space-y-4",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Application Version"
                                    }
                                    p { class: "text-sm text-gray-500", "Income Calculator v1.0.0" }
                                }
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Database Version"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "SQLite with SeaORM migrations"
                                    }
                                }
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Support"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "For help and documentation, visit the Help section or contact support."
                                    }
                                }
                            }
                        }

                        // Save button
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            div { class: "flex justify-between items-center",
                                p { class: "text-sm text-gray-600",
                                    "Settings are automatically saved to your local database."
                                }
                                button {
                                    class: "bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 font-medium",
                                    onclick: save_settings,
                                    "Save Settings"
                                }
                            }
                        }
                    }
                }
            }
        }
    }}