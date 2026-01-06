use dioxus::prelude::*;
use crate::components::Input;
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Information() -> Element {
    rsx! {
        Input {
            placeholder: "Borrower's Name",
            label: "Borrower's Name",
            value: "",
        }
        Input { placeholder: "Employer Name", label: "Employer Name", value: "" }
        Input { placeholder: "Income Type", label: "Income Type", value: "" }
        Input { placeholder: "Loan Number", label: "Loan Number", value: "" }
    }
}
 