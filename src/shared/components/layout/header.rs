use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
      div { id: "header", class: "flex justify-between items-center  p-4" }
    }
}
