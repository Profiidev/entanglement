use axum::{Extension, Router};
use centaurus::{
  db::init::init_db,
  init::{
    axum::{listener_setup, run_app_connect_info},
    logging::init_logging,
    router::base_router,
  },
};
#[cfg(debug_assertions)]
use dotenv::dotenv;
use tracing::info;

use crate::{config::Config, rate_limit::RateLimiter};

mod auth;
mod config;
mod db;
mod gravatar;
mod rate_limit;
mod user;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let config = Config::parse();
  init_logging(&config.base);

  let listener = listener_setup(config.base.port).await;
  let mut rate_limiter = RateLimiter::default();

  let mut router = api_router(&mut rate_limiter);
  router = base_router(router, &config.base, &config.metrics).await;
  let app = state(router, config).await;

  rate_limiter.init();

  info!("Starting application");
  run_app_connect_info(listener, app).await;
}

fn api_router(rate_limiter: &mut RateLimiter) -> Router {
  Router::new()
    .nest("/auth", auth::router(rate_limiter))
    .nest("/user", user::router())
}

async fn state(router: Router, config: Config) -> Router {
  let db = init_db::<migration::Migrator>(&config.db, &config.db_url).await;
  let router = auth::state(router, &config, &db).await;

  router.layer(Extension(db)).layer(Extension(config))
}
