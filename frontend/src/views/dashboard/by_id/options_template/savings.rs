use dioxus::prelude::*;
use shared::models::SavingsData;

#[component]
pub fn SavingsSection(data: SavingsData, on_change: EventHandler<SavingsData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    // Helper function to update data and trigger on_change
    let mut update_data = move |field: &str, value: String| {
        let mut new_data = local_data();
        match field {
            "monthly_savings" => new_data.monthly_savings = value.parse().unwrap_or(0.0),
            "annual_savings" => new_data.annual_savings = value.parse().unwrap_or(0.0),
            "debt_paid" => new_data.debt_paid = value.parse().unwrap_or(0.0),
            "payment_reduction" => new_data.payment_reduction = value.parse().unwrap_or(0.0),
            "recoup_period_months" => new_data.recoup_period_months = value.parse().unwrap_or(0.0),
            _ => {}
        }
        local_data.set(new_data.clone());
        on_change.call(new_data);
    };

    rsx! {
        div { class: "bg-white p-4 sm:p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Savings" }

            // Mobile-first responsive layout: cards on mobile, table on larger screens
            div { class: "block lg:hidden space-y-4",
                // Monthly Savings
                div { class: "bg-gray-50 p-3 rounded-lg",
                    label { class: "block text-sm font-medium text-gray-700 mb-2", "Monthly Savings" }
                    input {
                        r#type: "number",
                        value: "{local_data().monthly_savings}",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |evt: Event<FormData>| update_data("monthly_savings", evt.value()),
                    }
                }

                // Annual Savings
                div { class: "bg-gray-50 p-3 rounded-lg",
                    label { class: "block text-sm font-medium text-gray-700 mb-2", "Annual Savings" }
                    input {
                        r#type: "number",
                        value: "{local_data().annual_savings}",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |evt: Event<FormData>| update_data("annual_savings", evt.value()),
                    }
                }

                // Debt Paid
                div { class: "bg-gray-50 p-3 rounded-lg",
                    label { class: "block text-sm font-medium text-gray-700 mb-2", "Debt Paid" }
                    input {
                        r#type: "number",
                        value: "{local_data().debt_paid}",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |evt: Event<FormData>| update_data("debt_paid", evt.value()),
                    }
                }

                // Payment Reduction
                div { class: "bg-gray-50 p-3 rounded-lg",
                    label { class: "block text-sm font-medium text-gray-700 mb-2", "Payment Reduction" }
                    input {
                        r#type: "number",
                        value: "{local_data().payment_reduction}",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |evt: Event<FormData>| update_data("payment_reduction", evt.value()),
                    }
                }

                // Recoup Period
                div { class: "bg-gray-50 p-3 rounded-lg",
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "Recoup Period (months)"
                    }
                    input {
                        r#type: "number",
                        value: "{local_data().recoup_period_months}",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |evt: Event<FormData>| update_data("recoup_period_months", evt.value()),
                    }
                }
            }

            // Desktop table layout
            div { class: "hidden lg:block overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Mo Savings"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Year Savings"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Debt Paid"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Pmt Reduction"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Recoup"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().monthly_savings}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("monthly_savings", evt.value()),
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().annual_savings}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("annual_savings", evt.value()),
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().debt_paid}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("debt_paid", evt.value()),
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().payment_reduction}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("payment_reduction", evt.value()),
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "{local_data().recoup_period_months}",
                                    class: "w-full px-2 py-1 border rounded",
                                    oninput: move |evt: Event<FormData>| update_data("recoup_period_months", evt.value()),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}