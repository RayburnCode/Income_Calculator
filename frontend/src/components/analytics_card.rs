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
        "blue" | "primary" => "text-theme-primary-600 dark:text-theme-primary-400",
        "green" | "success" => "text-theme-success-600 dark:text-theme-success-400",
        "purple" | "secondary" => "text-theme-primary-700 dark:text-theme-primary-300",
        "red" | "error" => "text-theme-error-600 dark:text-theme-error-400",
        "yellow" | "warning" => "text-theme-warning-600 dark:text-theme-warning-400",
        "indigo" => "text-theme-primary-800 dark:text-theme-primary-200",
        "pink" => "text-theme-error-500 dark:text-theme-error-300",
        "gray" => "text-theme-text-600 dark:text-theme-text-300",
        _ => "text-theme-primary-600 dark:text-theme-primary-400",
    };

    let trend_color = if trend_positive { "text-theme-success-600 dark:text-theme-success-400" } else { "text-theme-error-600 dark:text-theme-error-400" };
    let trend_icon = if trend_positive { "↗" } else { "↘" };

    rsx! {
        div { class: "bg-theme-surface-50 dark:bg-theme-surface-900 p-4 sm:p-6 rounded-lg shadow-md {class}",
            div { class: "flex items-center justify-between",
                div { class: "flex-1",
                    div { class: "flex items-center",
                        if let Some(icon_str) = icon {
                            div { class: "mr-3 {color_classes}",
                                span { class: "text-2xl", "{icon_str}" }
                            }
                        }
                        div {
                            h3 { class: "text-sm sm:text-lg font-semibold text-theme-text-700 dark:text-theme-text-200",
                                "{title}"
                            }
                            if let Some(sub) = subtitle {
                                p { class: "text-xs sm:text-sm text-theme-text-500 dark:text-theme-text-400 mt-1",
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