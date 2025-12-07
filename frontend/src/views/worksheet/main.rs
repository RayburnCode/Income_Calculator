use dioxus::prelude::*;
use crate::views::worksheet::{Information, Hourly, Salary, OTBonus, Commission, OtherW2, SocialSecurity, Pension, IRA, OtherIncome};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Worksheet() -> Element {
    rsx! {
        // Have a Check box to show/hide the different sections
        // Ability to add a new tab to add multiple borrowers

        Information {}
        // Pay Type
        Hourly {}
        Salary {}
        OTBonus {}
        Commission {}
        OtherW2 {}

        // Other Taxable and Nontaxable Income
        SocialSecurity {}
        Pension {}
        IRA {}
        OtherIncome {}
    }

}
