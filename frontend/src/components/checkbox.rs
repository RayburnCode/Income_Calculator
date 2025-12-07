use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    /// Checkbox ID attribute
    #[props(default)]
    pub id: String,
    /// Checkbox name attribute
    #[props(default)]
    pub name: String,
    /// Whether the checkbox is checked
    #[props(default = false)]
    pub checked: bool,
    /// Checkbox value attribute
    #[props(default)]
    pub value: String,
    /// Label text for the checkbox
    #[props(default)]
    pub label: String,
    /// Whether the checkbox is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Whether the checkbox is required
    #[props(default = false)]
    pub required: bool,
    /// Additional CSS classes for the checkbox input
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the label
    #[props(default)]
    pub label_class: String,
    /// Additional CSS classes for the container div
    #[props(default)]
    pub container_class: String,
    /// Change event handler
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Click event handler
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let CheckboxProps {
        id,
        name,
        checked,
        value,
        label,
        disabled,
        required,
        class,
        label_class,
        container_class,
        onchange,
        onclick,
    } = props;

    let checkbox_id = if !id.is_empty() { id } else { name.clone() };
    let base_container_class = "flex items-center mb-4";
    let base_checkbox_class = "w-4 h-4 border border-light rounded-xs bg-neutral-secondary-medium focus:ring-2 focus:ring-brand-soft";
    let base_label_class = "select-none ms-2 text-sm font-medium text-heading";
    let disabled_label_class = "select-none ms-2 text-sm font-medium text-fg-disabled";

    rsx! {
        div { class: if !container_class.is_empty() { "{container_class}" } else { "{base_container_class}" },
            input {
                class: if !class.is_empty() { "{class}" } else { "{base_checkbox_class}" },
                id: "{checkbox_id}",
                name: "{name}",
                r#type: "checkbox",
                value: "{value}",
                checked: if checked { "true" } else { "false" },
                disabled: if disabled { "true" } else { "false" },
                required: if required { "true" } else { "false" },
                onchange: move |evt| {
                    if let Some(handler) = onchange.as_ref() {
                        handler.call(evt);
                    }
                },
                onclick: move |evt| {
                    if let Some(handler) = onclick.as_ref() {
                        handler.call(evt);
                    }
                },
            }
            if !label.is_empty() {
                label {
                    class: if !label_class.is_empty() { "{label_class}" } else if disabled { "{disabled_label_class}" } else { "{base_label_class}" },
                    r#for: "{checkbox_id}",
                    "{label}"
                    if required {
                        span { class: "text-red-500 ml-1", "*" }
                    }
                }
            }
        }
    }
}
