use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_clipboard::hooks::use_clipboard;
use futures::TryStreamExt;
use sync_wrapper::SyncStream;

use crate::{
  comm::Communication,
  components::{button::Button, input::Input},
};

#[component]
pub fn Home() -> Element {
  let conn = use_resource(Communication::init);

  rsx! {
    div { class: "flex items-center w-full justify-center p-4",
      div { class: "w-full",
        p { class: "text-2xl font-bold text-white pb-2", "Share a file" }
        if let Some(Ok(conn)) = &*conn.read() {
          Upload { conn: conn.clone() }
        } else if let Some(Err(e)) = &*conn.read() {
          p { class: "text-lg text-red-500", "Error: {e}" }
        } else {
          p { class: "text-lg text-white", "Connecting..." }
        }
      }
    }
  }
}

#[component]
fn Upload(conn: Arc<Communication>) -> Element {
  let mut ticket = use_signal(|| "Empty".to_string());
  let mut file = use_signal(|| "Empty".to_string());
  let mut clipboard = use_clipboard();

  rsx! {
    Input {
      r#type: "file",
      class: "input w-full",
      onchange: {
        let conn = conn.clone();
        move |evt: Event<FormData>| {
          let conn = conn.clone();
          async move {
            let files = evt.files();
            let Some(file) = files.first() else { return };
            let stream = SyncStream::new(file.byte_stream())
              .map_err(|e| std::io::Error::other(e.to_string()));
            ticket.set(conn.send_message(stream).await.unwrap());
          }
        }
      },
    }
    Button {
      class: "mt-2",
      onclick: move |_| {
        clipboard.set(ticket()).unwrap();
      },
       "Copy Ticket"
    }
    p { class: "text-lg text-white", {ticket} }
    Input {
      r#type: "text",
      onchange: move |evt: Event<FormData>| {
        let conn = conn.clone();
        async move {
          let value = evt.value();
          file.set(
            String::from_utf8(conn.receive_message(&value).await.unwrap().to_vec())
                .unwrap(),
          );
        }
      },
    }
    p { class: "text-lg text-white", {file} }
  }
}
