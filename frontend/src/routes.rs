
use dioxus::prelude::*;

use crate::views::{ Home, Help};
use crate::views::dashboard::MainDashboard;
use crate::views::dashboard::income_worksheet::Worksheet;
use crate::views::dashboard::options_template::OptionsTemplate;
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
        #[route("/income-worksheet")]
        Worksheet {},
        #[route("/options-template")]
        OptionsTemplate {},
        #[route("/help")]
        Help {},

}