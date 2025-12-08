use dioxus::prelude::*;

#[derive(Clone)]
pub struct IncomeAccordionItem {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub initially_open: bool,
    pub include_in_calc: bool,
}

#[derive(Props, PartialEq, Clone)]
pub struct IncomeAccordionProps {
    pub items: Vec<IncomeAccordionItem>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub on_include_change: Option<EventHandler<(String, bool)>>,
}

impl PartialEq for IncomeAccordionItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.title == other.title
    }
}

#[component]
pub fn IncomeAccordion(props: IncomeAccordionProps) -> Element {
    let IncomeAccordionProps {
        items,
        class,
        on_include_change,
    } = props;

    let items_vec = use_signal(|| items);
    
    let mut open_items = use_signal(|| {
        items_vec.read()
            .iter()
            .enumerate()
            .filter(|(_, item)| item.initially_open)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()
    });

    let mut include_states = use_signal(|| {
        items_vec.read()
            .iter()
            .map(|item| item.include_in_calc)
            .collect::<Vec<bool>>()
    });

    rsx! {
        div {
            class: if !class.is_empty() { "{class}" } else { "space-y-4" },
            for (idx , item) in items_vec.read().iter().enumerate() {
                {
                    let item_id = item.id.clone();
                    let item_title = item.title.clone();
                    let is_open = open_items.read().contains(&idx);
                    let is_included = include_states.read().get(idx).copied().unwrap_or(false);
                    
                    rsx! {
                        div {
                            key: "{item_id}",
                            class: "bg-white rounded-lg shadow overflow-hidden",
                            div { class: "border-b border-gray-200",
                                button {
                                    r#type: "button",
                                    class: "flex items-center justify-between w-full p-5 font-medium text-left text-gray-700 hover:bg-gray-50 transition",
                                    onclick: move |_| {
                                        let mut current_open = open_items();
                                        if current_open.contains(&idx) {
                                            current_open.retain(|&i| i != idx);
                                        } else {
                                            current_open.push(idx);
                                        }
                                        open_items.set(current_open);
                                    },
                                    div { class: "flex items-center gap-4",
                                        input {
                                            r#type: "checkbox",
                                            class: "w-4 h-4 border border-gray-300 rounded",
                                            checked: is_included,
                                            onclick: move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                            },
                                            onchange: move |_| {
                                                let mut states = include_states();
                                                if let Some(state) = states.get_mut(idx) {
                                                    *state = !*state;
                                                    if let Some(handler) = &on_include_change {
                                                        handler.call((item_id.clone(), *state));
                                                    }
                                                }
                                                include_states.set(states);
                                            },
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
                                div {
                                    class: "p-6 border-t border-gray-200",
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
