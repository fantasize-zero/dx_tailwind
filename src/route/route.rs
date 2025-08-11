use dioxus::prelude::*;

use crate::views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/{id}")]
    Blog { id: i32 },
}
