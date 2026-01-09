use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct SearchProps {
    /// Input ID attribute
    #[props(default)]
    pub id: String,
    /// Input name attribute
    #[props(default)]
    pub name: String,
    /// Input value
    #[props(default)]
    pub value: String,
    /// Placeholder text
    #[props(default = "Search...".to_string())]
    pub placeholder: String,
    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Additional CSS classes for the input
    #[props(default)]
    pub class: String,
    /// Input event handler
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Change event handler
    pub onchange: Option<EventHandler<FormEvent>>,
}

/// A search input component with a search icon
#[component]
pub fn Search(props: SearchProps) -> Element {
    let SearchProps {
        id,
        name,
        value,
        placeholder,
        disabled,
        class,
        oninput,
        onchange,
    } = props;

    let input_id = if !id.is_empty() { id } else { name.clone() };
    let base_input_class = "bg-theme-bg-secondary border-2 border-theme-text-secondary/30 text-theme-text-primary text-sm rounded-lg focus:ring-2 focus:ring-accent focus:border-accent block w-full pl-10 pr-4 py-3 shadow-sm placeholder:text-theme-text-secondary transition-all duration-200 hover:border-theme-text-secondary/50";

    rsx! {
        div { class: "relative",
            // Search icon
            div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                svg {
                    class: "w-5 h-5 text-theme-text-secondary",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        d: "m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
            // Input field
            input {
                class: if !class.is_empty() { "{class}" } else { "{base_input_class}" },
                id: "{input_id}",
                name: "{name}",
                placeholder: "{placeholder}",
                r#type: "text",
                value: "{value}",
                disabled: if disabled { "true" } else { "false" },
                oninput: move |evt| {
                    if let Some(handler) = oninput.as_ref() {
                        handler.call(evt);
                    }
                },
                onchange: move |evt| {
                    if let Some(handler) = onchange.as_ref() {
                        handler.call(evt);
                    }
                },
            }
        }
    }
}