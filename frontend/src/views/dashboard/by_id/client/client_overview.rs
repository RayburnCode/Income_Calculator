use dioxus::prelude::*;
use crate::views::dashboard::by_id::income_worksheet::W2Jobs;
use super::client_info_card::ClientInfoCard;
use shared::models::Status;

#[component]
pub fn ClientOverview(
    id: i32,
    borrower: Signal<Option<shared::models::Borrower>>,
    error_message: Signal<Option<String>>,
    is_editing: Signal<bool>,
    edit_name: Signal<String>,
    edit_status: Signal<Status>,
    edit_email: Signal<String>,
    edit_phone: Signal<String>,
    save_changes: Callback<()>,
    cancel_edit: Callback<()>,
    format_phone_number: fn(&str) -> String,
) -> Element {
    rsx! {
        ClientInfoCard {
            borrower,
            error_message,
            is_editing,
            edit_name,
            edit_status,
            edit_email,
            edit_phone,
            save_changes,
            cancel_edit,
            format_phone_number,
        }

        div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Income Information" }
            W2Jobs { borrower_id: id }
        }

        div { class: "mt-8 bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4", "Loan Information" }
            p { class: "text-gray-600", "Loan details will be displayed here." }
        }
    }
}