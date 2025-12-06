
use dioxus::prelude::*;

use crate::views::{ Home, Help};
use crate::components::layout::AppLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
    #[route("/help")]
        Help {},

}