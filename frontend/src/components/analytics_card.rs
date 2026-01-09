use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct AnalyticsCardProps {
    /// The title of the card
    pub title: String,
    /// The main value to display
    pub value: String,
    /// Optional subtitle or additional info
    #[props(default)]
    pub subtitle: Option<String>,
    /// Optional icon (can be an emoji or icon class)
    #[props(default)]
    pub icon: Option<String>,
    /// Color theme for the card
    #[props(default = "blue".to_string())]
    pub color: String,
    /// Optional trend indicator (e.g., "+5.2%", "-2.1%")
    #[props(default)]
    pub trend: Option<String>,
    /// Whether the trend is positive (green) or negative (red)
    #[props(default = true)]
    pub trend_positive: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
}

/// A reusable analytics card component for displaying metrics and KPIs
#[component]
pub fn AnalyticsCard(props: AnalyticsCardProps) -> Element {
    let AnalyticsCardProps {
        title,
        value,
        subtitle,
        icon,
        color,
        trend,
        trend_positive,
        class,
    } = props;

    // Color classes based on the color prop
    let color_classes = match color.as_str() {
        "blue" | "primary" => "text-blue-600 dark:text-blue-400",
        "green" | "success" => "text-green-600 dark:text-green-400",
        "purple" | "secondary" => "text-purple-600 dark:text-purple-400",
        "red" | "error" => "text-red-600 dark:text-red-400",
        "yellow" | "warning" => "text-yellow-600 dark:text-yellow-400",
        "indigo" => "text-indigo-600 dark:text-indigo-400",
        "pink" => "text-pink-600 dark:text-pink-400",
        "gray" => "text-gray-600 dark:text-gray-400",
        _ => "text-blue-600 dark:text-blue-400",
    };

    let trend_color = if trend_positive { "text-green-600 dark:text-green-400" } else { "text-red-600 dark:text-red-400" };
    let trend_icon = if trend_positive { "↗" } else { "↘" };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-4 sm:p-6 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 {class}",
            div { class: "flex items-center justify-between",
                div { class: "flex-1",
                    div { class: "flex items-center",
                        if let Some(icon_str) = icon {
                            div { class: "mr-3 {color_classes}",
                                span { class: "text-2xl", "{icon_str}" }
                            }
                        }
                        div {
                            h3 { class: "text-sm sm:text-lg font-semibold text-gray-900 dark:text-white",
                                "{title}"
                            }
                            if let Some(sub) = subtitle {
                                p { class: "text-xs sm:text-sm text-gray-600 dark:text-gray-300 mt-1",
                                    "{sub}"
                                }
                            }
                        }
                    }
                    p { class: "text-2xl sm:text-3xl font-bold {color_classes} mt-2",
                        "{value}"
                    }
                    if let Some(trend_value) = trend {
                        div { class: "flex items-center mt-2",
                            span { class: "text-sm {trend_color} font-medium mr-1",
                                "{trend_icon} {trend_value}"
                            }
                        }
                    }
                }
            }
        }
    }
}