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
    app_url: Url,
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
      Ok(Self::WaitForAuth {
        app_url: config.app_url,
      })
    } else {
      Ok(Self::Main { config })
    }
  }

  pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    terminal.clear()?;

    loop {
      terminal.draw(|frame| self.render(frame))?;
      let event = event::read()?;
      self.handle_event(event);
    }
  }

  fn render(&mut self, frame: &mut Frame) {
    match self {
      App::EnterAppUrl { field, error, .. } => ui::app_url(frame, field, error),
      _ => unimplemented!(),
    }
  }

  fn handle_event(&mut self, event: Event) {
    match self {
      App::EnterAppUrl { field, error, .. } => {
        field.handle_event(&event);

        if let Event::Key(key) = &event
          && key.code == KeyCode::Enter
        {
          let app_url = field.value();
          if let Ok(url) = Url::parse(&app_url) {
            *self = App::WaitForAuth { app_url: url };
          } else {
            *error = Some("Invalid URL".into());
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
}
