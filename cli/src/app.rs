use std::time::Duration;

use centaurus::error::Result;
use ratatui::{
  DefaultTerminal, Frame,
  crossterm::event::{self, Event, KeyCode, KeyModifiers},
};
use url::Url;

use crate::{
  api::ApiClient,
  auth::CodeServer,
  config::Config,
  input::{InputField, InputMode},
  ui,
};

pub enum App {
  EnterAppUrl {
    token: Option<String>,
    field: InputField,
    error: Option<String>,
  },
  WaitForAuth {
    config: Config,
    api: ApiClient,
    error: Option<String>,
    code_server: CodeServer,
  },
  #[allow(dead_code)]
  Main { config: Config, api: ApiClient },
}

impl App {
  pub async fn new(config: Option<Config>) -> Result<Self> {
    let Some(config) = config else {
      return Ok(Self::enter_app_url(None, None, None));
    };

    let Ok(api) = ApiClient::new(config.app_url.clone(), config.token.clone()).await else {
      return Ok(Self::enter_app_url(
        config.token,
        Some("Failed to reach server. Check URL and try again.".into()),
        Some(config.app_url.to_string()),
      ));
    };

    if !api.is_authenticated() {
      Self::wait_for_auth(config).await
    } else {
      Ok(Self::main(api, config))
    }
  }

  pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    terminal.clear()?;
    loop {
      terminal.draw(|frame| self.render(frame))?;

      // This limits the fps to 10
      if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        self.handle_event(event).await;
      }

      self.update().await;
    }
  }

  fn render(&mut self, frame: &mut Frame) {
    match self {
      App::EnterAppUrl { field, error, .. } => ui::app_url(frame, field, error),
      App::WaitForAuth { config, .. } => ui::wait_for_auth(frame, config.app_url.as_str()),
      App::Main { .. } => ui::main(frame),
    }
  }

  async fn update(&mut self) {
    #[allow(clippy::single_match)]
    match self {
      App::WaitForAuth {
        code_server,
        api,
        config,
        ..
      } => {
        if let Some(token) = code_server.get_code() {
          code_server.cleanup();
          api.set_token(token.clone());
          config.token = Some(token);
          config.save().await.ok();
          *self = Self::main(api.clone(), config.clone());
        }
      }
      _ => {}
    }
  }

  async fn handle_event(&mut self, event: Event) {
    match self {
      App::EnterAppUrl {
        field,
        error,
        token,
      } => {
        field.handle_event(&event);

        if let Event::Key(key) = &event
          && key.code == KeyCode::Enter
        {
          let app_url = field.value();
          if let Ok(mut url) = Url::parse(&app_url) {
            url.set_path("");
            let config = Config::new(url.clone(), token.clone());
            config.save().await.ok();

            match Self::wait_for_auth(config).await {
              Ok(next) => *self = next,
              Err(e) => {
                *error = Some(format!("Failed to connect to app: {e}"));
              }
            }
          } else {
            *error = Some("Invalid URL".into());
          }
        }
      }
      App::WaitForAuth {
        config,
        error,
        code_server,
        ..
      } => {
        if let Event::Key(key) = &event {
          match key.code {
            KeyCode::Enter => {
              if opener::open(format!("{}auth/cli", config.app_url)).is_err() {
                *error = Some("Failed to open browser".into());
                return;
              }
            }
            KeyCode::Backspace => {
              code_server.cleanup();
              *self =
                Self::enter_app_url(config.token.clone(), None, Some(config.app_url.to_string()));
            }
            _ => {}
          }
        }
      }
      _ => {}
    }

    if let Event::Key(key) = &event {
      match key.code {
        KeyCode::Esc => {
          std::process::exit(0);
        }
        KeyCode::Char(c) if c == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) => {
          std::process::exit(0);
        }
        _ => {}
      }
    }
  }

  fn enter_app_url(token: Option<String>, error: Option<String>, value: Option<String>) -> Self {
    let mut field = InputField::new(
      "App URL".into(),
      Some("https://entanglement.example.com".into()),
      value,
    );
    field.set_mode(InputMode::Editing);

    Self::EnterAppUrl {
      token,
      field,
      error,
    }
  }

  async fn wait_for_auth(config: Config) -> Result<Self> {
    let api = ApiClient::new(config.app_url.clone(), config.token.clone()).await?;

    if api.is_authenticated() {
      Ok(Self::main(api, config))
    } else {
      Ok(Self::WaitForAuth {
        code_server: CodeServer::new(config.app_url.clone()).await?,
        config,
        api,
        error: None,
      })
    }
  }

  fn main(api: ApiClient, config: Config) -> Self {
    Self::Main { api, config }
  }
}
