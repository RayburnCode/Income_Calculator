use dioxus::prelude::*;
use shared::models::IncomeInformationData;

#[component]
pub fn IncomeInformationSection(data: IncomeInformationData, on_change: EventHandler<IncomeInformationData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Income Information" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1",
                        "Borrower Monthly Income:"
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
                    }
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