use dioxus::prelude::*;

#[component]
pub fn OtherFeesSection() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Other Fees" }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "3rd Party Fee:" }
                    input {
                        r#type: "number",
                        name: "thirdPartyFees",
                        value: "123456",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Appraisal Fee:" }
                    input {
                        r#type: "number",
                        name: "appraisalFee",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Investor Fee:" }
                    input {
                        r#type: "number",
                        name: "investorFee",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Padded Taxes:" }
                    input {
                        r#type: "number",
                        name: "paymentPadTaxes",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Padded Insurance:" }
                    input {
                        r#type: "number",
                        name: "paymentPadInsurance",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Lender Credit:" }
                    input {
                        r#type: "number",
                        name: "lenderCredit",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Admin:" }
                    input {
                        r#type: "number",
                        id: "adminFees",
                        value: "895",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Tax Service:" }
                    input {
                        r#type: "number",
                        id: "taxService",
                        value: "88",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Flood Cert:" }
                    input {
                        r#type: "number",
                        id: "floodCert",
                        value: "8",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                    }
                }
                div { class: "md:col-span-2 lg:col-span-3 grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Total Closing Costs:"
                        }
                        input {
                            r#type: "number",
                            name: "closingCosts",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Cash Out (Bring to Close):"
                        }
                        input {
                            r#type: "number",
                            name: "cashOutAmount",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md",
                        }
                    }
                }
            }
        }
    }
}