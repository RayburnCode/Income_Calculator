use dioxus::prelude::*;
use shared::models::PricingData;

#[component]
pub fn PricingSection(data: PricingData, on_change: EventHandler<PricingData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Pricing" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Description"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Rate"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "YSP %"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "YSP $"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "BD %"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "BD $"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Select"
                            }
                        }
                    }
                    tbody {
                        // Rate option 1
                        tr {
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "text",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "noteRate1",
                                    id: "noteRate1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "ysp1",
                                    id: "ysp1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "123456",
                                    name: "yspDollar",
                                    id: "yspDollar1",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bd1",
                                    id: "bd1",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bdDollar",
                                    id: "bdDollar1",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "selectRate1",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        // Rate option 2
                        tr {
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "text",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "noteRate2",
                                    id: "noteRate2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "ysp2",
                                    id: "ysp2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "123456",
                                    name: "yspDollar",
                                    id: "yspDollar2",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bd2",
                                    id: "bd2",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bdDollar",
                                    id: "bdDollar2",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "selectRate2",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        // Rate option 3
                        tr {
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "text",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "noteRate3",
                                    id: "noteRate3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "ysp3",
                                    id: "ysp3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    value: "123456",
                                    name: "yspDollar",
                                    id: "yspDollar3",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bd3",
                                    id: "bd3",
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    name: "bdDollar",
                                    id: "bdDollar3",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-gray-50",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    name: "selectRate3",
                                    class: "w-4 h-4",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}