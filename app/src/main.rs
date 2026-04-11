#[cfg(feature = "desktop")]
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::{document::eval, prelude::*};

use views::Home;

use crate::layout::AppLayout;

mod components;
mod layout;
mod utils;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
  #[layout(AppLayout)]
  #[route("/")]
  Home {},
}

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const DX_COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  #[cfg(feature = "mobile")]
  utils::set_android_flags();

  #[cfg(feature = "desktop")]
  dioxus::LaunchBuilder::new()
    .with_cfg(
      Config::default()
        .with_menu(None)
        .with_window(WindowBuilder::new().with_decorations(false)),
    )
    .launch(App);
  #[cfg(feature = "mobile")]
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  eval("document.body.classList.add('dark')");
  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link { rel: "stylesheet", href: DX_COMPONENTS_CSS }
    document::Link { rel: "stylesheet", href: TAILWIND_CSS }

    Router::<Route> {}
  }
}
