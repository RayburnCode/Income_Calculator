use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/percent.svg");
 
#[component]
pub fn Navbar() -> Element {
    let mut reset = use_context::<Signal<usize>>();
    // optional toast context
    let mut toast = use_context::<Signal<Option<String>>>();

    rsx! {
        nav { class: "text-gray-900 fixed w-full z-20 top-0 start-0 border-b border-default bg-white/95 dark:bg-gray-900/95 backdrop-blur-md shadow-sm",
            div { class: "flex flex-wrap items-center justify-between p-4",
                a { class: "flex items-center space-x-3 rtl:space-x-reverse",
                    img { alt: "Debt to Income Logo", class: "h-7", src: LOGO }
                    Link {
                        to: Route::Home {},
                        class: "self-center text-xl text-heading font-semibold whitespace-nowrap",
                        "Income Calculator"
                    }
                }
                button {
                    aria_controls: "navbar-default",
                    aria_expanded: "false",
                    class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-body rounded-base md:hidden hover:bg-neutral-secondary-soft hover:text-heading focus:outline-none focus:ring-2 focus:ring-neutral-tertiary",
                    "data-collapse-toggle": "navbar-default",
                    r#type: "button",
                    span { class: "sr-only", "Open main menu" }
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        height: "24",
                        view_box: "0 0 24 24",
                        width: "24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M5 7h14M5 12h14M5 17h14",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_width: "2",
                        }
                    }
                }
                div {
                    class: "hidden w-full md:block md:w-auto",
                    id: "navbar-default",
                    ul { class: "font-medium flex flex-col p-4 md:p-0 mt-4 border border-default rounded-base bg-neutral-secondary-soft md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-neutral-primary",

                        li {
                            Link { to: Route::Help {}, "Help" }
                        }
                        li {
                            // Reset: increment the reset signal so child components can clear their local state, then navigate home
                            Link {
                                to: Route::Home {},
                                onclick: move |_| {
                                    reset.with_mut(|r| *r += 1usize);
                                    toast.set(Some("Reset complete".to_string()));
                                },
                                "Reset"
                            }
                        }
                    }
                }
            }
        }
    }
}