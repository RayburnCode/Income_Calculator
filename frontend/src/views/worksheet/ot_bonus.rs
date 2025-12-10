use dioxus::prelude::*;

#[component]
pub fn OTBonus() -> Element {
    rsx! {
        p { "Overime and Bonus" }
    }

}


									
// 	 $ -   	YTD Overtime / Bonus*			0	# months		  	Income
// 	 $ -   	Past year OT breakout			0	# months		  	Income
// 	 $ -   	Additional year OT / Bonus			0	# months		  	Income
									
// 	  	YTD Avg			FALSE	"*If DU requires only a YTD paystub, OT/Bonus must be annualized. 
//  Divide YTD OT/Bonus by 12 months; for qualifying purposes, typically a two year history of receipt is required."			
// 	  	YTD + 1 year Avg			FALSE				
// 	  	YTD + 2 Year Avg			FALSE				
									
// 	 $ -   	Use lower of calculations		or check the income you wish to use				 $ -   	 $ -   
									
									