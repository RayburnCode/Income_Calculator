use dioxus::prelude::*;

#[component]
pub fn DebtToIncomeSection() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4", "Income" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    label {
                        r#for: "borrowersIncome",
                        class: "block text-sm font-medium text-gray-700 mb-1",
                    }
                    "Borrower's Monthly Income:"
                    input {
                        r#type: "number",
                        name: "borrowersIncome",
                        id: "borrowersIncome",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md mt-1",
                    }
                }
                div {
                    label {
                        r#for: "coborrowersIncome",
                        class: "block text-sm font-medium text-gray-700 mb-1",
                    }
                    "Co-Borrower's Monthly Income:"
                    input {
                        r#type: "number",
                        name: "coborrowersIncome",
                        id: "coborrowersIncome",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md mt-1",
                    }
                }
            }

            div { class: "mt-6",
                h4 { class: "text-lg font-semibold mb-4", "DTI" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div {
                        label {
                            r#for: "frontRatio",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                        }
                        "Front End Ratio:"
                        input {
                            r#type: "text",
                            class: "form-control",
                            id: "frontRatio",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 mt-1",
                        }
                    }
                    div {
                        label {
                            r#for: "backRatio",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                        }
                        "Back End Ratio:"
                        input {
                            r#type: "text",
                            class: "form-control",
                            id: "backRatio",
                            readonly: true,
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 mt-1",
                        }
                    }
                }
            }
        }
    }
}