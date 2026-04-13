use std::sync::Arc;

use dioxus::html::bytes::Bytes;
use eyre::{ContextCompat, ErrReport};
use futures_core::Stream;
use iroh::{
  address_lookup::MdnsAddressLookup, endpoint::presets, protocol::Router, Endpoint, EndpointId,
};
use iroh_blobs::{api::downloader::Downloader, store::mem::MemStore, BlobsProtocol, Hash};

pub struct Communication {
  endpoint: Endpoint,
  store: MemStore,
  router: Router,
  downloader: Downloader,
}

impl Communication {
  pub async fn init() -> Result<Arc<Self>, ErrReport> {
    let mdns = MdnsAddressLookup::builder();
    let endpoint = Endpoint::builder(presets::N0)
      .address_lookup(mdns)
      .bind()
      .await?;
    let store = MemStore::new();

    let blobs = BlobsProtocol::new(&store, None);

    let router = Router::builder(endpoint.clone())
      .accept(iroh_blobs::ALPN, blobs)
      .spawn();

    let downloader = store.downloader(&endpoint);

    Ok(Arc::new(Communication {
      endpoint,
      store,
      router,
      downloader,
    }))
  }

  pub async fn send_message(
    &self,
    msg: impl Stream<Item = std::io::Result<Bytes>> + Send + Sync + 'static,
  ) -> Result<String, ErrReport> {
    let tag = self.store.blobs().add_stream(msg).await.await?;
    Ok(format!("{}:{}", tag.hash, self.endpoint.id()))
  }

  pub async fn receive_message(&self, tag: &str) -> Result<Bytes, ErrReport> {
    let (hash, id) = tag.split_once(':').context("Invalid Tag")?;
    let hash: Hash = hash.parse()?;
    let id: EndpointId = id.parse()?;

    self.downloader.download(hash, Some(id)).await?;

    Ok(self.store.blobs().get_bytes(hash).await?)
  }

  pub async fn shutdown(&self) {
    self.router.shutdown().await.unwrap();
  }
}

impl PartialEq for Communication {
  fn eq(&self, other: &Self) -> bool {
    self.endpoint.id() == other.endpoint.id()
  }
}
