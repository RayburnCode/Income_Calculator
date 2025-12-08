use dioxus::prelude::*;
use crate::components::Input;
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Information() -> Element {
    rsx! {
        p { "Borrowers Name" }
        Input {}
        p { "Employer Name" }
        p { "Income Type" }

        p { "Loan Number (if applicable)" }
    }
}
