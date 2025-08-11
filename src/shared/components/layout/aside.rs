use dioxus::prelude::*;

#[component]
pub fn Aside() -> Element {
    rsx! {
      div { id: "aside", class: "bg-black text-white p-4", Menu {} }
    }
}

#[component]
fn Menu() -> Element {
    let items: Vec<Element> = (0..5)
        .map(|_| {
            rsx! {
              MenuItem {}
            }
        })
        .collect();

    rsx! {
      div { id: "menu", class: "flex flex-col", {items.iter().map(|item| item.clone())} }
    }
}

#[component]
fn MenuItem() -> Element {
    rsx! {
      div { class: "menu-item", "Menu Item" }
    }
}
