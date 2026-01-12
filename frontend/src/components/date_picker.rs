use dioxus::prelude::*;
use chrono::NaiveDate;

#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    label: String,
    selected_date: Signal<NaiveDate>,
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let DatePickerProps {
        label,
        mut selected_date,
        min_date,
        max_date,
    } = props;

    let current_date = selected_date();

    rsx! {
        div { class: "flex flex-col",
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                "{label}"
            }
            input {
                r#type: "date",
                class: "px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                value: "{current_date.format(\"%Y-%m-%d\")}",
                min: min_date.map(|d| d.format("%Y-%m-%d").to_string()),
                max: max_date.map(|d| d.format("%Y-%m-%d").to_string()),
                onchange: move |evt| {
                    if let Ok(date) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
                        selected_date.set(date);
                    }
                },
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct DateRangePickerProps {
    start_date: Signal<NaiveDate>,
    end_date: Signal<NaiveDate>,
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    let DateRangePickerProps {
        start_date,
        end_date,
        min_date,
        max_date,
    } = props;

    rsx! {
        div { class: "flex flex-col sm:flex-row gap-4",
            DatePicker {
                label: "Start Date".to_string(),
                selected_date: start_date,
                min_date,
                max_date,
            }
            DatePicker {
                label: "End Date".to_string(),
                selected_date: end_date,
                min_date,
                max_date,
            }
        }
    }
}