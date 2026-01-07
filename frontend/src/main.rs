// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;



mod components;
mod views;
mod routes;
use routes::Route;
 
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    let client_result = use_resource(|| async {
        client::Client::new().await
    });

    match &*client_result.value().read() {
        Some(Ok(client)) => {
            provide_context(client.clone());
            rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                Router::<Route> {}
            }
        },
        Some(Err(error_msg)) => {
            rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
                    div { class: "max-w-md mx-auto bg-white rounded-lg shadow-lg p-6",
                        div { class: "flex items-center mb-4",
                            div { class: "flex-shrink-0",
                                svg {
                                    class: "h-8 w-8 text-red-500",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z",
                                    }
                                }
                            }
                            div { class: "ml-3",
                                h3 { class: "text-lg font-medium text-gray-900",
                                    "Database Connection Error"
                                }
                            }
                        }
                        div { class: "mb-4",
                            p { class: "text-sm text-gray-600",
                                "Unable to connect to the database. Please check your database setup and try again."
                            }
                        }
                        div { class: "bg-gray-50 p-3 rounded-md",
                            pre { class: "text-xs text-gray-800 whitespace-pre-wrap",
                                "{error_msg}"
                            }
                        }
                        div { class: "mt-4",
                            button {
                                class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md transition duration-200",
                                onclick: // For desktop apps, we can't reload the page like in web browsers
                                // The user needs to manually restart the application
                                move |_| {},
                                "Restart Application"
                            }
                            p { class: "text-sm text-gray-500 mt-2 text-center",
                                "Please restart the application manually to retry the connection."
                            }
                        }
                    }
                }
            }
        },
        None => {
            rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
                    div { class: "text-center",
                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4" }
                        p { class: "text-gray-600", "Connecting to database..." }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
