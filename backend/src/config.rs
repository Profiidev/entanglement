use centaurus::{
  config::{BaseConfig, MetricsConfig},
  db::config::DBConfig,
};
use figment::{
  Figment,
  providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use url::Url;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
  #[serde(flatten)]
  pub base: BaseConfig,
  #[serde(flatten)]
  pub db: DBConfig,
  #[serde(flatten)]
  pub metrics: MetricsConfig,

  pub oidc_issuer: Url,
  pub oidc_client_id: String,
  pub oidc_client_secret: String,
  pub oidc_scopes: String,
  pub app_url: Url,

  pub db_url: String,

  pub auth_issuer: String,
  pub auth_jwt_expiration: i64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      base: BaseConfig::default(),
      db: DBConfig::default(),
      db_url: "".to_string(),
      oidc_issuer: "http://localhost".parse().unwrap(),
      oidc_client_id: "".to_string(),
      oidc_client_secret: "".to_string(),
      oidc_scopes: "openid email profile".to_string(),
      metrics: MetricsConfig {
        metrics_name: "entanglement".to_string(),
        ..Default::default()
      },
      app_url: "http://localhost:5173".parse().unwrap(),
      auth_issuer: "smaug_auth".to_string(),
      auth_jwt_expiration: 60 * 60 * 24 * 7, // 7 days
    }
  }
}

impl Config {
  #[instrument]
  pub fn parse() -> Self {
    let config = Figment::new()
      .merge(Serialized::defaults(Self::default()))
      .merge(Env::raw().global());

    let config: Self = config.extract().expect("Failed to parse configuration");

    if config.db_url.is_empty() {
      panic!("Database URL is not set");
    }

    if config.oidc_client_id.is_empty() || config.oidc_client_secret.is_empty() {
      panic!("OIDC client ID or secret is not set");
    }

    config
  }
}
