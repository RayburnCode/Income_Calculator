use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-6",
            div { class: "max-w-4xl mx-auto",
                // Header
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Settings" }
                    p { class: "text-gray-600 mt-2",
                        "Manage your application preferences and configuration"
                    }
                }

                // Settings sections
                div { class: "space-y-6",
                    // General Settings
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h2 { class: "text-xl font-semibold text-gray-800 mb-4", "General Settings" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Application Theme"
                                    }
                                    p { class: "text-sm text-gray-500", "Choose your preferred theme" }
                                }
                                select { class: "border border-gray-300 rounded-md px-3 py-2",
                                    option { "Light" }
                                    option { "Dark" }
                                    option { "System" }
                                }
                            }
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Language"
                                    }
                                    p { class: "text-sm text-gray-500", "Select your language" }
                                }
                                select { class: "border border-gray-300 rounded-md px-3 py-2",
                                    option { "English" }
                                    option { "Spanish" }
                                    option { "French" }
                                }
                            }
                        }
                    }

                    // Notification Settings
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Notifications" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Email Notifications"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "Receive email updates about your account"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                }
                            }
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Push Notifications"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "Receive push notifications in your browser"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                }
                            }
                        }
                    }

                    // Security Settings
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Security" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Two-Factor Authentication"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "Add an extra layer of security"
                                    }
                                }
                                button { class: "bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 text-sm",
                                    "Enable 2FA"
                                }
                            }
                            div { class: "flex items-center justify-between",
                                div {
                                    label { class: "text-sm font-medium text-gray-700",
                                        "Change Password"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "Update your account password"
                                    }
                                }
                                button { class: "bg-gray-600 text-white px-4 py-2 rounded hover:bg-gray-700 text-sm",
                                    "Change Password"
                                }
                            }
                        }
                    }

                    // Save button
                    div { class: "bg-white p-6 rounded-lg shadow-md",
                        div { class: "flex justify-end",
                            button { class: "bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700 font-medium",
                                "Save Settings"
                            }
                        }
                    }
                }
            }
        }
    }
}