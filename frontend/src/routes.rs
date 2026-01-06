
use dioxus::prelude::*;

use crate::views::{ Home, Help};
use crate::views::dashboard::MainDashboard;
use crate::views::dashboard::by_id::income_worksheet::Worksheet;
use crate::views::dashboard::by_id::options_template::OptionsTemplate;
use crate::views::dashboard::by_id::client::ClientDetails;
use crate::components::layout::AppLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/dashboard")]
        MainDashboard {},
        #[route("/dashboard/client/:id")]
        ClientDetails { id: i32 },
        #[route("/income-worksheet")]
        Worksheet {},
        #[route("/options-template")]
        OptionsTemplate {},
        #[route("/help")]
        Help {},

}