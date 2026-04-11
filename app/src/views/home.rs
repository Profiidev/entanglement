use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
    p {
      class: "text-2xl font-bold text-white",
      "Welcome to Entanglement!"
    }
  }
}
