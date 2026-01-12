use crate::Route;
use dioxus::prelude::*;
use crate::components::Search;
const LOGO: Asset = asset!("/assets/percent.svg");
 
#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        nav { class: "text-theme-text-primary fixed w-full z-20 top-0 start-0 border-b border-theme-text-secondary/20 bg-theme-bg-primary/95 backdrop-blur-md shadow-sm",
            div { class: "flex flex-wrap items-center justify-between p-4",
                // Left section - Logo
                Link {
                    to: Route::MainDashboard {},
                    class: "flex items-center space-x-3 rtl:space-x-reverse",
                    onclick: move |_| menu_open.set(false),
                    img { alt: "Debt to Income Logo", class: "h-7", src: LOGO }
                    span { class: "self-center text-xl text-theme-text-primary font-semibold whitespace-nowrap hover:text-accent transition-colors",
                        "Income Calculator"
                    }
                }

                // Center section - Search bar
                div { class: "flex-1 max-w-md mx-4 hidden md:block",
                    Search {
                        placeholder: "Search clients, loans, income...",
                        class: "w-full",
                    }
                }

                // Right section - Menu button
                div { class: "flex items-center",
                    button {
                        aria_controls: "navbar-default",
                        aria_expanded: "{menu_open()}",
                        class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-theme-text-secondary rounded-lg md:hidden hover:bg-theme-bg-secondary hover:text-theme-text-primary focus:outline-none focus:ring-2 focus:ring-accent/50",
                        "data-collapse-toggle": "navbar-default",
                        r#type: "button",
                        onclick: move |_| menu_open.toggle(),
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
                }
                div {
                    class: if menu_open() { "w-full md:block md:w-auto" } else { "hidden w-full md:block md:w-auto" },
                    id: "navbar-default",
                    ul { class: "font-medium flex flex-col p-4 md:p-0 mt-4 border border-theme-text-secondary/20 rounded-lg bg-theme-bg-secondary md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-transparent",

                        li {
                            Link {
                                to: Route::Analytics {},
                                class: "block py-2 px-3 text-theme-text-secondary rounded hover:bg-theme-bg-tertiary hover:text-theme-text-primary md:hover:bg-transparent md:border-0 md:p-0 transition-colors",
                                onclick: move |_| menu_open.set(false),
                                "Analytics"
                            }
                        }

                        li {
                            Link {
                                to: Route::Settings {},
                                class: "block py-2 px-3 text-theme-text-secondary rounded hover:bg-theme-bg-tertiary hover:text-theme-text-primary md:hover:bg-transparent md:border-0 md:p-0 transition-colors",
                                onclick: move |_| menu_open.set(false),
                                "Settings"
                            }
                        }
                        li {
                            Link {
                                to: Route::Help {},
                                class: "block py-2 px-3 text-theme-text-secondary rounded hover:bg-theme-bg-tertiary hover:text-theme-text-primary md:hover:bg-transparent md:border-0 md:p-0 transition-colors",
                                onclick: move |_| menu_open.set(false),
                                "Help"
                            }
                        }
                    
                    }
                }
            }
        }
    }
}