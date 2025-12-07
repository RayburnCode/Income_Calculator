use dioxus::prelude::*;
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Information() -> Element {
    rsx! {
        p { "Borrowers Name" }
        p { "Employer Name" }
        p { "Income Type" }

        p { "Loan Number (if applicable)" }
    }
}
