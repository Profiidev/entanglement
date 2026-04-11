#[cfg(feature = "desktop")]
use crate::components::{header::Header, separator::Separator};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
  #[cfg(feature = "mobile")]
  let header: Option<Element> = None;
  #[cfg(feature = "desktop")]
  let header = rsx! {
    Header {  }
      Separator { class: "separator bg-border!" }
  };

  rsx! {
    {header}

    Outlet::<Route> {}
  }
}
