use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    /// The type of input (text, email, password, number, etc.)
    #[props(default = "text".to_string())]
    pub r#type: String,
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
    #[props(default)]
    pub placeholder: String,
    /// Label text
    #[props(default)]
    pub label: String,
    /// Whether the input is required
    #[props(default = false)]
    pub required: bool,
    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Additional CSS classes for the input
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the label
    #[props(default)]
    pub label_class: String,
    /// Input event handler
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Change event handler
    pub onchange: Option<EventHandler<FormEvent>>,
}

/// A flexible input component with TailwindCSS styling
#[component]
pub fn Input(props: InputProps) -> Element {
    let InputProps {
        r#type,
        id,
        name,
        value,
        placeholder,
        label,
        required,
        disabled,
        class,
        label_class,
        oninput,
        onchange,
    } = props;

    let input_id = if !id.is_empty() { id } else { name.clone() };
    let base_label_class = "block mb-2.5 text-sm font-semibold text-theme-text-primary";
    let base_input_class = "bg-theme-bg-secondary border-2 border-theme-text-secondary/30 text-theme-text-primary text-sm rounded-lg focus:ring-2 focus:ring-accent focus:border-accent block w-full px-4 py-3 shadow-sm placeholder:text-theme-text-secondary transition-all duration-200 hover:border-theme-text-secondary/50";

    rsx! {
        div {
            if !label.is_empty() {
                label {
                    class: if !label_class.is_empty() { "{label_class}" } else { "{base_label_class}" },
                    r#for: "{input_id}",
                    "{label}"
                    if required {
                        span { class: "text-red-500 ml-1 font-bold", "*" }
                    }
                }
            }
            input {
                class: if !class.is_empty() { "{class}" } else { "{base_input_class}" },
                id: "{input_id}",
                name: "{name}",
                placeholder: "{placeholder}",
                r#type: "{r#type}",
                value: "{value}",
                required: if required { "true" } else { "false" },
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