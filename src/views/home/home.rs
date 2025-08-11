use dioxus::prelude::*;

use crate::shared::*;

#[component]
pub fn Home() -> Element {
    rsx! {
      // Echo {}
      // Hero {}
      Header {}
      div { class: "flex ",
        Aside {}
        Main {}
      }
    }
}
