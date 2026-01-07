use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TabProps {
    /// List of tab items
    pub tabs: Vec<TabItem>,
    /// Currently active tab index
    #[props(default = 0)]
    pub active_tab: usize,
    /// Callback when a tab is clicked
    pub on_tab_change: Option<EventHandler<usize>>,
    /// Additional CSS classes for the container
    #[props(default)]
    pub class: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TabItem {
    /// Tab label/text
    pub label: String,
    /// Optional href/link
    pub href: Option<String>,
    /// Whether the tab is disabled
    pub disabled: bool,
    /// Optional icon component (could be string or Element)
    pub icon: Option<String>,
}

/// A flexible tab component with TailwindCSS styling
#[component]
pub fn Tab(props: TabProps) -> Element {
    let TabProps {
        tabs,
        active_tab,
        on_tab_change,
        class,
    } = props;

    let active_classes = "inline-block p-4 text-blue-600 bg-neutral-secondary-soft rounded-t-base active";
    let inactive_classes = "inline-block p-4 text-gray-700 rounded-t-base hover:text-gray-900 hover:bg-neutral-secondary-soft";
    let disabled_classes = "inline-block p-4 text-gray-400 rounded-t-base cursor-not-allowed";

    rsx! {
        ul {
            class: "flex flex-wrap text-sm font-medium text-center border-b border-default {class}",
            aria_label: "Tabs",
            for (index , tab) in tabs.iter().enumerate() {
                li {
                    key: "{index}",
                    class: if index < tabs.len() - 1 { "me-2" } else { "" },
                    if tab.disabled {
                        a {
                            class: "{disabled_classes}",
                            role: "button",
                            aria_disabled: "true",
                            tabindex: "-1",
                            if let Some(icon) = &tab.icon {
                                span { class: "mr-2", "{icon}" }
                            }
                            "{tab.label}"
                        }
                    } else {
                        a {
                            class: if index == active_tab { active_classes } else { inactive_classes },
                            href: tab.href.as_deref().unwrap_or("#"),
                            onclick: move |_| {
                                if let Some(handler) = on_tab_change.as_ref() {
                                    handler.call(index);
                                }
                            },
                            role: "tab",
                            aria_selected: if index == active_tab { "true" } else { "false" },
                            tabindex: "0",
                            if let Some(icon) = &tab.icon {
                                span { class: "mr-2", "{icon}" }
                            }
                            "{tab.label}"
                        }
                    }
                }
            }
        }
    }
}