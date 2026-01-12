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
        class: _,
        oninput,
        onchange,
    } = props;

    rsx! {
        form { class: "max-w-md mx-auto",
            label {
                class: "block mb-2.5 text-sm font-medium text-heading sr-only",
                r#for: "search",
                "Search"
            }
            div { class: "relative",
                div { class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                    svg {
                        "aria-hidden": "true",
                        class: "w-4 h-4 text-body",
                        fill: "none",
                        height: "24",
                        view_box: "0 0 24 24",
                        width: "24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_width: "2",
                        }
                    }
                }
                input {
                    class: "block w-full p-3 ps-9 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body",
                    id: if !id.is_empty() { "{id}" } else { "{name}" },
                    placeholder: "{placeholder}",
                    r#type: "search",
                    required: "false",
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
                button {
                    class: "absolute end-1.5 bottom-1.5 text-white bg-brand hover:bg-brand-strong box-border border border-transparent focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded text-xs px-3 py-1.5 focus:outline-none",
                    r#type: "button",
                    "Search"
                }
            }
        }
    }
}