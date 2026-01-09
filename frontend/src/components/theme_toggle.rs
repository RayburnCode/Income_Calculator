use dioxus::prelude::*;
use crate::components::theme::{use_theme, Theme};

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme_context = use_theme();
    // Make is_dark reactive by reading the signal in the component body
    let is_dark = matches!(*theme_context.theme.read(), Theme::Dark);

    rsx! {
        button {
            onclick: move |_| theme_context.toggle(),
            class: "p-2 rounded-lg bg-theme-surface-100 hover:bg-theme-surface-200 transition-colors duration-200 border border-theme-border-300",
            title: if is_dark { "Switch to Light Mode" } else { "Switch to Dark Mode" },
            svg {
                class: "w-5 h-5 text-theme-text-primary",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                if is_dark {
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
                    }
                } else {
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
                    }
                }
            }
        }
    }
}