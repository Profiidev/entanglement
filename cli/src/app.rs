use centaurus::error::Result;
use ratatui::{
  DefaultTerminal, Frame,
  crossterm::event::{self, Event, KeyCode, KeyModifiers},
};
use url::Url;

use crate::{
  api::ApiClient,
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
  },
  Main {
    config: Config,
  },
}

impl App {
  pub async fn new(config: Option<Config>) -> Result<Self> {
    let Some(config) = config else {
      return Ok(Self::enter_app_url());
    };

    let api = ApiClient::new(config.app_url.clone(), config.token.clone()).await?;

    if !api.is_authenticated() {
      Self::wait_for_auth(config).await
    } else {
      Ok(Self::Main { config })
    }
  }

  pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    terminal.clear()?;

    loop {
      terminal.draw(|frame| self.render(frame))?;
      let event = event::read()?;
      self.handle_event(event).await;
    }
  }

  fn render(&mut self, frame: &mut Frame) {
    match self {
      App::EnterAppUrl { field, error, .. } => ui::app_url(frame, field, error),
      App::WaitForAuth { config, .. } => ui::wait_for_auth(frame, config.app_url.as_str()),
      _ => unimplemented!(),
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
      App::WaitForAuth { config, api, error } => {
        if let Event::Key(key) = &event {
          match key.code {
            KeyCode::Enter => {
              if opener::open(format!("{}auth/cli", config.app_url)).is_err() {
                *error = Some("Failed to open browser".into());
                return;
              }
            }
            KeyCode::Backspace => {
              *self = Self::enter_app_url();
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

  fn enter_app_url() -> Self {
    let mut field = InputField::new(
      "App URL".into(),
      Some("https://entanglement.example.com".into()),
    );
    field.set_mode(InputMode::Editing);

    Self::EnterAppUrl {
      token: None,
      field,
      error: None,
    }
  }

  async fn wait_for_auth(config: Config) -> Result<Self> {
    let api = ApiClient::new(config.app_url.clone(), config.token.clone()).await?;

    Ok(Self::WaitForAuth {
      config,
      api,
      error: None,
    })
  }
}
