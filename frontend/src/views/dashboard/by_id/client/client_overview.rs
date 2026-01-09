use dioxus::prelude::*;
use crate::views::dashboard::by_id::income_worksheet::W2Jobs;
use crate::views::dashboard::by_id::outreach::timeline::Timeline;
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
    edit_date_of_birth: Signal<String>,
    edit_social_security_number: Signal<String>,
    edit_address: Signal<String>,
    edit_city: Signal<String>,
    edit_state: Signal<String>,
    edit_zip_code: Signal<String>,
    edit_mailing_address_different: Signal<bool>,
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
            edit_date_of_birth,
            edit_social_security_number,
            edit_address,
            edit_city,
            edit_state,
            edit_zip_code,
            edit_mailing_address_different,
            save_changes,
            cancel_edit,
            format_phone_number,
        }

        div { class: "mt-8 bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold text-gray-800 dark:text-gray-200 mb-4",
                "Income Information"
            }
            W2Jobs { borrower_id: id }
        }

        div { class: "mt-8",
            Timeline { id }
        }
    }
    }
