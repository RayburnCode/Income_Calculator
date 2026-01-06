use dioxus::prelude::*;

#[component]
pub fn Commission() -> Element {
    rsx! {
		p { "Commission" }
	} 

}


										
	//  $ -   	YTD Commission			*minus Expenses		 $ -   	expenses (based upon 2106 expenses)		
	//  $ -   	Past year commission			*minus Expenses		 $ -   	2106 Expenses		
	//  $ -   	Additional year commission			*minus Expenses		 $ -   	2106 expenses		
	// 				*Follow investor guidelines for unreimbursed expense policy.					
										
	//  $ -   	 = Net income			0	# months	  	monthly income		
	//  $ -   	 = Net Income			0	# months	  	monthly income		
	//  $ -   	 = Net Income			0	# months	  	monthly income		
										
	//   	YTD Avg using net income			FALSE		2106 YTD Expense Estimate			
	//   	YTD + 1 year using Net Income			FALSE		2 year Commission Vs 2106 Expenses			
	//   	YTD + 2 year using net income			FALSE		Commission		 $ -   	
	// 						Expenses		 $ -   	
	// 						Expense factor		 $ -   	
										
	//  $ -   	Use Lower of calculations		or check the income you wish to use				 $ -   	 $ -   	
										
										