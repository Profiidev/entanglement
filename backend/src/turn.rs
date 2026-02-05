use std::{
  net::{IpAddr, SocketAddr},
  str::FromStr,
  sync::Arc,
  time::Duration,
};

use tokio::net::UdpSocket;
use turn_server::{
  api::start_server,
  config::{Api, Auth, Config, Interface, Log, LogLevel, Transport, Turn},
};

pub async fn start() {
  let port = 3478;
  let config = Config {
    log: Log {
      level: LogLevel::Debug,
    },
    turn: Turn {
      realm: "default".into(),
      interfaces: vec![Interface {
        bind: SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap(),
        external: SocketAddr::from_str(&format!("192.168.178.22:{}", port)).unwrap(),
        transport: Transport::UDP,
      }],
    },
    api: Api {
      ..Default::default()
    },
    auth: Auth {
      static_auth_secret: Some("1234".into()),
      ..Default::default()
    },
  };

  turn_server::startup(Arc::new(config)).await.unwrap();
}
