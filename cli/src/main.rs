use centaurus::{error::Result, init::logging::init_logging};

use crate::{app::App, config::Config};

mod api;
mod app;
mod config;
mod input;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(debug_assertions)]
  dotenv::dotenv().ok();

  let level = config::log_level();
  init_logging(level);

  let config = Config::load().await?;
  let app = App::new(config).await?;

  let terminal = ratatui::init();

  app.run(terminal).await?;

  ratatui::restore();
  Ok(())
}
