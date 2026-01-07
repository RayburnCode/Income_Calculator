use dioxus::prelude::*;
use shared::models::BenefitToBorrowerData;

#[component]
pub fn BenefitToBorrowerSection(data: BenefitToBorrowerData, loan_purpose: String, on_change: EventHandler<BenefitToBorrowerData>) -> Element {
    let mut local_data = use_signal(|| data.clone());

    // Update local data when prop changes
    use_effect(move || {
        local_data.set(data.clone());
    });

    // Calculate existing mortgage payment
    let existing_mortgage_payment = use_memo(move || {
        let data = local_data();
        let mut total = data.existing_pi;

        if data.escrow_taxes { total += data.existing_taxes; }
        if data.escrow_insurance { total += data.existing_insurance; }
        if data.escrow_flood_insurance { total += data.existing_flood_insurance; }

        total += data.existing_pmi + data.existing_hoa;
        total
    });

    // Calculate proposed mortgage payment
    let proposed_mortgage_payment = use_memo(move || {
        let data = local_data();
        let mut total = data.proposed_pi;

        if data.escrow_taxes { total += data.proposed_taxes; }
        if data.escrow_insurance { total += data.proposed_insurance; }
        if data.escrow_flood_insurance { total += data.proposed_flood_insurance; }

        total += data.proposed_pmi + data.proposed_hoa;
        total
    });

    // Calculate overage/shortage (benefit to borrower)
    let overage_shortage = use_memo({
        let loan_purpose_clone = loan_purpose.clone();
        move || {
            if loan_purpose_clone == "purchase" {
                0.0 // No existing loan for purchase
            } else {
                existing_mortgage_payment() - proposed_mortgage_payment()
            }
        }
    });

    // Calculate overage/shortage class
    let overage_class = use_memo(move || {
        let overage = overage_shortage();
        if overage > 0.0 {
            "w-full px-2 py-1 border rounded bg-green-50 text-green-800 font-semibold"
        } else if overage < 0.0 {
            "w-full px-2 py-1 border rounded bg-red-50 text-red-800 font-semibold"
        } else {
            "w-full px-2 py-1 border rounded bg-gray-50 font-semibold"
        }
    });

    // Calculate existing total obligations
    let existing_total_obligations = use_memo(move || {
        let data = local_data();
        data.existing_pi + data.existing_taxes + data.existing_insurance +
        data.existing_flood_insurance + data.existing_pmi + data.existing_hoa
    });

    // Calculate proposed total obligations
    let proposed_total_obligations = use_memo(move || {
        let data = local_data();
        data.proposed_pi + data.proposed_taxes + data.proposed_insurance +
        data.proposed_flood_insurance + data.proposed_pmi + data.proposed_hoa
    });

    // Generic update function for numeric fields
    let mut update_field = move |field_name: &str, value: String| {
        if let Ok(num_value) = value.parse::<f64>() {
            let mut data = local_data();
            match field_name {
                "existing_pi" => data.existing_pi = num_value,
                "existing_taxes" => data.existing_taxes = num_value,
                "existing_insurance" => data.existing_insurance = num_value,
                "existing_flood_insurance" => data.existing_flood_insurance = num_value,
                "existing_pmi" => data.existing_pmi = num_value,
                "existing_hoa" => data.existing_hoa = num_value,
                "proposed_pi" => data.proposed_pi = num_value,
                "proposed_taxes" => data.proposed_taxes = num_value,
                "proposed_insurance" => data.proposed_insurance = num_value,
                "proposed_flood_insurance" => data.proposed_flood_insurance = num_value,
                "proposed_pmi" => data.proposed_pmi = num_value,
                "proposed_hoa" => data.proposed_hoa = num_value,
                "debt_paydown" => data.debt_paydown = num_value,
                _ => {}
            }
            local_data.set(data.clone());
            on_change.call(data);
        }
    };

    // Update checkbox function
    let mut update_checkbox = move |field_name: &str, checked: bool| {
        let mut data = local_data();
        match field_name {
            "escrow_taxes" => data.escrow_taxes = checked,
            "escrow_insurance" => data.escrow_insurance = checked,
            "escrow_flood_insurance" => data.escrow_flood_insurance = checked,
            _ => {}
        }
        local_data.set(data.clone());
        on_change.call(data);
    };
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md mb-6",
            h4 { class: "text-lg font-semibold mb-4 text-black", "Benefit to the Borrower" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full table-auto border-collapse border border-gray-300",
                    thead {
                        tr { class: "bg-gray-50",
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                ""
                            }
                            if loan_purpose != "purchase" {
                                th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                    "Existing Loan"
                                }
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Proposed Loan"
                            }
                            th { class: "border border-gray-300 px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Escrow"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "PI:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "text",
                                        id: "existingPI",
                                        name: "existingPI",
                                        value: "{local_data().existing_pi}",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedPI",
                                    name: "proposedPI",
                                    value: "{local_data().proposed_pi}",
                                    oninput: move |e: Event<FormData>| update_field("proposed_pi", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Taxes:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingTaxes",
                                        name: "existingTaxes",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedTaxes",
                                    name: "proposedTaxes",
                                    value: "{local_data().proposed_taxes}",
                                    oninput: move |e: Event<FormData>| update_field("proposed_taxes", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowTaxes",
                                    name: "escrowTaxes",
                                    checked: "{local_data().escrow_taxes}",
                                    onchange: move |e| update_checkbox("escrow_taxes", e.checked()),
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Insurance:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingInsurance",
                                        name: "existingInsurance",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedInsurance",
                                    name: "proposedInsurance",
                                    value: "{local_data().proposed_insurance}",
                                    oninput: move |e: Event<FormData>| update_field("proposed_insurance", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowInsurance",
                                    name: "escrowInsurance",
                                    checked: "{local_data().escrow_insurance}",
                                    onchange: move |e| update_checkbox("escrow_insurance", e.checked()),
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Flood Insurance:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingFloodInsurance",
                                        name: "existingFloodInsurance",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedFloodInsurance",
                                    name: "proposedFloodInsurance",
                                    value: "{local_data().proposed_flood_insurance}",
                                    oninput: move |e: Event<FormData>| update_field("proposed_flood_insurance", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2 text-center",
                                input {
                                    r#type: "checkbox",
                                    id: "escrowFloodInsurance",
                                    name: "escrowFloodInsurance",
                                    checked: "{local_data().escrow_flood_insurance}",
                                    onchange: move |e| update_checkbox("escrow_flood_insurance", e.checked()),
                                    class: "w-4 h-4",
                                }
                            }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "PMI:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingPMI",
                                        name: "existingPMI",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                div { class: "flex space-x-2",
                                    input {
                                        r#type: "number",
                                        id: "proposedPMI",
                                        name: "proposedPMI",
                                        value: "{local_data().proposed_pmi}",
                                        oninput: move |e: Event<FormData>| update_field("proposed_pmi", e.value()),
                                        class: "flex-1 px-2 py-1 border rounded",
                                    }
                                    input {
                                        r#type: "number",
                                        id: "proposedPMIamount",
                                        name: "proposedPMIamount",
                                        class: "flex-1 px-2 py-1 border rounded",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "HOA:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingHOA",
                                        name: "existingHOA",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-gray-50",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedHOA",
                                    name: "proposedHOA",
                                    value: "{local_data().proposed_hoa}",
                                    oninput: move |e: Event<FormData>| update_field("proposed_hoa", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Mortgage Payment:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingmortgagePayment",
                                        name: "existingmortgagePayment",
                                        value: "{existing_mortgage_payment():.2}",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-blue-50 text-blue-800 font-semibold",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedmortgagePayment",
                                    name: "proposedmortgagePayment",
                                    value: "{proposed_mortgage_payment():.2}",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-green-50 text-green-800 font-semibold",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Overage/Shortage:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "overageShortage",
                                    name: "overageShortage",
                                    value: "{overage_shortage():.2}",
                                    readonly: true,
                                    class: "{overage_class()}",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Debt Paydown:"
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "debtPaydown",
                                    name: "debtPaydown",
                                    value: "{local_data().debt_paydown}",
                                    oninput: move |e: Event<FormData>| update_field("debt_paydown", e.value()),
                                    class: "w-full px-2 py-1 border rounded",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                        tr {
                            td { class: "border border-gray-300 px-4 py-2 font-semibold",
                                "Total Obligations:"
                            }
                            if loan_purpose != "purchase" {
                                td { class: "border border-gray-300 px-4 py-2",
                                    input {
                                        r#type: "number",
                                        id: "existingObligation",
                                        name: "existingObligation",
                                        value: "{existing_total_obligations():.2}",
                                        readonly: true,
                                        class: "w-full px-2 py-1 border rounded bg-blue-50 text-blue-800 font-semibold",
                                    }
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2",
                                input {
                                    r#type: "number",
                                    id: "proposedObligation",
                                    name: "proposedObligation",
                                    value: "{proposed_total_obligations():.2}",
                                    readonly: true,
                                    class: "w-full px-2 py-1 border rounded bg-green-50 text-green-800 font-semibold",
                                }
                            }
                            td { class: "border border-gray-300 px-4 py-2", "" }
                        }
                    }
                }
            }
        }
    }
}