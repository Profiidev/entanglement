use dioxus::html::bytes::Bytes;
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
  pub async fn init() -> Self {
    //let mdns = MdnsAddressLookup::builder();
    let endpoint = Endpoint::builder(presets::N0)
      //.address_lookup(mdns)
      .bind()
      .await
      .unwrap();
    let store = MemStore::new();

    let blobs = BlobsProtocol::new(&store, None);

    let router = Router::builder(endpoint.clone())
      .accept(iroh_blobs::ALPN, blobs)
      .spawn();

    let downloader = store.downloader(&endpoint);

    Communication {
      endpoint,
      store,
      router,
      downloader,
    }
  }

  pub async fn send_message(
    &self,
    msg: impl Stream<Item = std::io::Result<Bytes>> + Send + Sync + 'static,
  ) -> String {
    let tag = self.store.blobs().add_stream(msg).await.await.unwrap();
    format!("{}:{}", tag.hash, self.endpoint.id())
  }

  pub async fn receive_message(&self, tag: &str) -> Bytes {
    let (hash, id) = tag.split_once(':').unwrap();
    let hash: Hash = hash.parse().unwrap();
    let id: EndpointId = id.parse().unwrap();

    self.downloader.download(hash, Some(id)).await.unwrap();

    self.store.blobs().get_bytes(hash).await.unwrap()
  }

  pub async fn shutdown(&self) {
    self.router.shutdown().await.unwrap();
  }
}
