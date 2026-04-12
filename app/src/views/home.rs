use dioxus::prelude::*;
use futures::TryStreamExt;
use sync_wrapper::SyncStream;

use crate::{comm::Communication, components::input::Input};

#[component]
pub fn Home() -> Element {
  let conn = use_resource(Communication::init);
  let mut ticket = use_signal(|| "Empty".to_string());
  let mut file = use_signal(|| "Empty".to_string());

  rsx! {
    p {
      class: "text-2xl font-bold text-white",
      "Welcome to Entanglement!"
    }
    Input {
      type: "file",
      onchange: move |evt: Event<FormData>| async move {
        let files = evt.files();
        let Some(file) = files.first() else { return };
        let stream = SyncStream::new(file.byte_stream()).map_err(|e| std::io::Error::other(e.to_string()));
        let conn = conn.read();
        ticket.set(conn.as_ref().unwrap().send_message(stream).await);
      }
    }
    p {
      class: "text-lg text-white",
      {ticket}
    }
    Input {
      type: "text",
      onchange: move |evt: Event<FormData>| async move {
        let value = evt.value();
        let conn = conn.read();
        file.set(String::from_utf8(conn.as_ref().unwrap().receive_message(&value).await.to_vec()).unwrap());
      }
    }
    p {
      class: "text-lg text-white",
      {file}
    }
  }
}
