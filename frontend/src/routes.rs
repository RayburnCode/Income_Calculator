
use dioxus::prelude::*;

use crate::views::{ Welcome, Help};
use crate::views::dashboard::{MainDashboard, Analytics, Settings};
use crate::views::dashboard::by_id::{Worksheet, ClientDetails, OptionsTemplate, Timeline, OutreachTemplates, UploadDocuments, ClientDocuments, ClientNotes};
use crate::components::layout::AppLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(AppLayout)]
        #[route("/")]
        MainDashboard {},
        #[route("/welcome")]
        Welcome {},
        #[route("/dashboard/analytics")]
        Analytics {},

        #[route("/dashboard/settings")]
        Settings {},
        #[route("/:id/client")]
        ClientDetails { id: i32 },
        #[route("/:id/income-worksheet")]
        Worksheet {id: i32},
        #[route("/:id/options-template")]
        OptionsTemplate {id: i32},

        #[route("/:id/outreach/templates")]
        OutreachTemplates {id: i32},

        #[route("/:id/outreach/timeline")]
        Timeline {id: i32},

        #[route("/:id/upload-documents")]
        UploadDocuments {id: i32},

        #[route("/:id/documents")]
        ClientDocuments {id: i32},  

        #[route("/:id/notes")]
        ClientNotes {id: i32},  

        #[route("/help")]
        Help {},

}