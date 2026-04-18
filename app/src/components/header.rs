use crate::components::{
  button::{Button, ButtonVariant},
  separator::Separator,
};
use dioxus::{desktop::use_window, prelude::*};
use dioxus_free_icons::{
  Icon,
  icons::ld_icons::{LdMinus, LdSquare, LdX},
};

const APP_ICON: Asset = asset!("/assets/icon.svg");

#[component]
pub fn Header() -> Element {
  let window = use_window();

  rsx! {
    header { id: "navbar", class: "h-10 items-center flex",
      img {
        alt: "Entanglement Logo",
        class: "mr-1 h-full p-1 select-none",
        src: APP_ICON,
      }
      p { class: "mr-4", "Entanglement" }
      Separator { horizontal: false, class: "separator bg-border!" }
      Button {
        class: "p-1! rounded-full! ml-auto size-8 flex items-center justify-center",
        variant: ButtonVariant::Ghost,
        onclick: {
          let window = window.clone();
          move |_| window.set_minimized(true)
        },
        Icon { class: "size-5", icon: LdMinus {} }
      }
      Button {
        class: "p-1! rounded-full! size-8 flex items-center justify-center",
        variant: ButtonVariant::Ghost,
        onclick: {
          let window = window.clone();
          move |_| window.set_maximized(true)
        },
        Icon { class: "size-4", icon: LdSquare {} }
      }
      Button {
        class: "p-1! rounded-full! size-8 flex items-center justify-center hover:bg-destructive! mr-1",
        variant: ButtonVariant::Ghost,
        onclick: move |_| window.close(),
        Icon { class: "size-5", icon: LdX {} }
      }
    }
  }
}
