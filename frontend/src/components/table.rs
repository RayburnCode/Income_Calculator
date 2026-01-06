use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableProps<T: Clone + PartialEq + std::fmt::Display + 'static> {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<T>>,
}

#[component]
pub fn Table<T: Clone + PartialEq + std::fmt::Display + 'static>(props: TableProps<T>) -> Element {
    rsx! {
        div { class: "overflow-x-auto bg-white shadow-md rounded-lg",
            table { class: "min-w-full table-auto",
                thead { class: "bg-gray-50",
                    tr {
                        for header in &props.headers {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "{header}"
                            }
                        }
                    }
                }
                tbody { class: "bg-white divide-y divide-gray-200",
                    for row in &props.rows {
                        tr { class: "hover:bg-gray-50",
                            for cell in row {
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
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