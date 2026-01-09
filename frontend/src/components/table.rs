use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableProps<T: Clone + PartialEq + std::fmt::Display + 'static> {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<T>>,
}

#[component]
pub fn Table<T: Clone + PartialEq + std::fmt::Display + 'static>(props: TableProps<T>) -> Element {
    rsx! {
        div { class: "overflow-x-auto bg-theme-surface-50 dark:bg-theme-surface-900 shadow-md rounded-lg",
            table { class: "min-w-full table-auto",
                thead { class: "bg-theme-surface-100 dark:bg-theme-surface-800",
                    tr {
                        for header in &props.headers {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-theme-text-500 dark:text-theme-text-400 uppercase tracking-wider",
                                "{header}"
                            }
                        }
                    }
                }
                tbody { class: "bg-theme-surface-50 dark:bg-theme-surface-900 divide-y divide-theme-border-200 dark:divide-theme-border-700",
                    for row in &props.rows {
                        tr { class: "hover:bg-theme-surface-100 dark:hover:bg-theme-surface-800",
                            for cell in row {
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-theme-text-900 dark:text-theme-text-100",
                                    "{cell}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}