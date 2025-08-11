pub mod route;
pub mod shared;
pub mod views;

use dioxus::prelude::*;

use route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.min.css");

const MAIN_CSS: Asset = asset!("/assets/main.min.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: TAILWIND_CSS }
      document::Link { rel: "stylesheet", href: MAIN_CSS }
      div { class: "flex", "11" }
      Router::<Route> {}
    }
}
