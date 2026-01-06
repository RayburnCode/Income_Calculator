use dioxus::prelude::*;

#[component]
pub fn Pension() -> Element {
    rsx! {
        p { "Pension" }
    }

}


// With Documentation				
// 	Annual Benefit			
// 	Taxable Portion	x 100%	 $ -   	
//  $ -   	Non-Taxable	x 125%	 $ -   	
// 	# months	Total Income	 $ -   	 