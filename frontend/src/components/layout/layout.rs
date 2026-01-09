use dioxus::prelude::*;
use crate::Route;
use crate::components::layout::{Navbar};

// Wrapper types to distinguish different f64 contexts
#[derive(Clone, Copy)]
pub struct TotalIncome(pub Signal<f64>);

#[derive(Clone, Copy)]
pub struct TotalDebt(pub Signal<f64>);

#[derive(Clone, Copy)]
pub struct TotalHousing(pub Signal<f64>);


#[component]
pub fn AppLayout() -> Element {
    let reset_signal = use_signal(|| 0usize);
    use_context_provider(|| reset_signal);

    // Toast signal for brief notifications with auto-clear
    let mut toast = use_signal(|| None::<String>);
    use_context_provider(|| toast);
    
    // Auto-clear toast after 2.5 seconds
    use_resource(move || async move {
        if toast().is_some() {
            #[cfg(target_arch = "wasm32")]
            {
                use gloo_timers::future::TimeoutFuture;
                TimeoutFuture::new(2500).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                tokio::time::sleep(tokio::time::Duration::from_millis(2500)).await;
            }
            toast.set(None);
        }
    });
    
    // DTI calculation context signals with distinct wrapper types
    let total_income = use_signal(|| 0.0f64);
    use_context_provider(|| TotalIncome(total_income));
    
    let total_debt = use_signal(|| 0.0f64);
    use_context_provider(|| TotalDebt(total_debt));
    
    let total_housing = use_signal(|| 0.0f64);
    use_context_provider(|| TotalHousing(total_housing));
    


    rsx! {
        div { class: "min-h-screen bg-theme-bg-primary text-theme-text-primary flex flex-col",
            // Header
            Navbar {}
            // Main Content Area
            div { class: "pt-20 px-4 sm:px-6 py-8",
                div { class: "", Outlet::<Route> {} }
            }
            // Toast overlay (bottom-right)
            if let Some(msg) = toast.read().as_ref() {
                div { class: "fixed bottom-6 right-6 z-50",
                    div { class: "bg-theme-bg-tertiary text-theme-text-primary rounded-md px-4 py-2 shadow-lg border border-theme-text-secondary/20",
                        "{msg}"
                    }
                }
            }
        }
    }
} 