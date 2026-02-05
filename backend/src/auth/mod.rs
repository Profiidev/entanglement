use axum::{Extension, Router};
use centaurus::db::init::Connection;

use crate::{
  auth::{
    jwt_state::{JwtInvalidState, JwtState},
    oidc::OidcState,
  },
  config::Config,
  rate_limit::RateLimiter,
};

pub mod jwt_auth;
pub mod jwt_state;
mod logout;
mod oidc;
mod res;
mod test_token;

pub fn router(rate_limiter: &mut RateLimiter) -> Router {
  Router::new()
    .nest("/logout", logout::router())
    .nest("/test_token", test_token::router())
    .nest("/oidc", oidc::router(rate_limiter))
}

pub async fn state(router: Router, config: &Config, db: &Connection) -> Router {
  let jwt_state = JwtState::init(config, db).await;
  let oidc_state = OidcState::new(config).await;
  let jwt_invalid_state = JwtInvalidState::default();

  router
    .layer(Extension(jwt_state))
    .layer(Extension(oidc_state))
    .layer(Extension(jwt_invalid_state))
}
