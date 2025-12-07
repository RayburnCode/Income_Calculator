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
    let base_label_class = "block mb-2.5 text-sm font-medium text-heading";
    let base_input_class = "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body";

    rsx! {
        div {
            if !label.is_empty() {
                label {
                    class: if !label_class.is_empty() { "{label_class}" } else { "{base_label_class}" },
                    r#for: "{input_id}",
                    "{label}"
                    if required {
                        span { class: "text-red-500 ml-1", "*" }
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