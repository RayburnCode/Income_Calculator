use dioxus::prelude::*;

#[derive(Clone)]
pub struct AccordionItem {
    /// Unique identifier for the accordion item
    pub id: String,
    /// Title/heading for the accordion item
    pub title: String,
    /// Content to display when expanded (as Element)
    pub content: Element,
    /// Whether the item is initially expanded
    pub initially_open: bool,
    /// Whether to show a checkbox for this item
    pub show_checkbox: bool,
    /// Initial checkbox state
    pub checkbox_checked: bool,
    /// Callback when checkbox state changes
    pub on_checkbox_change: Option<EventHandler<bool>>,
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// List of accordion items
    pub items: Vec<AccordionItem>,
    /// Whether only one item can be open at a time (collapse others)
    #[props(default = true)]
    pub collapse_others: bool,
    /// Additional CSS classes for the container
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for accordion buttons
    #[props(default)]
    pub button_class: String,
    /// Additional CSS classes for accordion content
    #[props(default)]
    pub content_class: String,
}

impl PartialEq for AccordionItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.title == other.title
    }
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let AccordionProps {
        items,
        collapse_others,
        class,
        button_class,
        content_class,
    } = props;

    // Store items to extend lifetime
    let items_vec = use_signal(|| items);

    // Track which items are open (using their indices)
    let mut open_items = use_signal(|| {
        items_vec.read()
            .iter()
            .enumerate()
            .filter(|(_, item)| item.initially_open)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    });

    let base_container_class = "space-y-4";
    let base_button_class = "flex items-center justify-between w-full p-5 font-medium text-left text-gray-700 hover:bg-gray-50 transition";
    let base_content_class = "p-6 border-t border-gray-200";

    // Track checkbox states
    let mut checkbox_states = use_signal(|| {
        items_vec.read().iter().map(|item| item.checkbox_checked).collect::<Vec<bool>>()
    });

    rsx! {
        div {
            class: if !class.is_empty() { "{class}" } else { "{base_container_class}" },
            "data-accordion": if collapse_others { "collapse" } else { "open" },
            for (idx , item) in items_vec.read().iter().enumerate() {
                {
                    let item_id = item.id.clone();
                    let item_title = item.title.clone();
                    let is_open = open_items.read().contains(&idx);
                    let show_checkbox = item.show_checkbox;
                    let checkbox_checked = checkbox_states.read().get(idx).copied().unwrap_or(false);
                    let on_checkbox_change = item.on_checkbox_change.clone();
                    rsx! {
                        div { key: "{item_id}", class: "bg-white rounded-lg shadow overflow-hidden",
                            div { class: "border-b border-gray-200",
                                button {
                                    r#type: "button",
                                    class: if !button_class.is_empty() { "{button_class}" } else { "{base_button_class}" },
                                    onclick: move |_| {
                                        let mut current_open = open_items();
                                        if current_open.contains(&idx) {
                                            current_open.retain(|&i| i != idx);
                                        } else {
                                            if collapse_others {
                                                current_open.clear();
                                            }
                                            current_open.push(idx);
                                        }
                                        open_items.set(current_open);
                                    },
                                    div { class: "flex items-center gap-4",
                                        if show_checkbox {
                                            input {
                                                r#type: "checkbox",
                                                class: "w-4 h-4 border border-gray-300 rounded",
                                                checked: checkbox_checked,
                                                onclick: move |evt: Event<MouseData>| {
                                                    evt.stop_propagation();
                                                },
                                                onchange: move |_| {
                                                    let mut states = checkbox_states();
                                                    if let Some(state) = states.get_mut(idx) {
                                                        *state = !*state;
                                                        if let Some(handler) = &on_checkbox_change {
                                                            handler.call(*state);
                                                        }
                                                    }
                                                    checkbox_states.set(states);
                                                },
                                            }
                                        }
                                        span { class: "text-lg font-semibold", "{item_title}" }
                                    }
                                    svg {
                                        class: "w-5 h-5 transition-transform",
                                        class: if is_open { "rotate-180" } else { "" },
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "m19 9-7 7-7-7",
                                        }
                                    }
                                }
                            }
                            if is_open {
                                div { class: if !content_class.is_empty() { "{content_class}" } else { "{base_content_class}" },
                                    {item.content.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}