use crate::{
  components::{header::Header, separator::Separator},
  Route,
};
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
  rsx! {
    Header {  }
    Separator { class: "separator bg-border!" }

    Outlet::<Route> {}
  }
}
