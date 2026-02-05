use ratatui::{
  Frame,
  crossterm::event::Event,
  layout::Rect,
  style::{Color, Style},
  text::Span,
  widgets::{Block, Paragraph},
};
use tui_input::{Input, backend::crossterm::EventHandler};

pub struct InputField {
  input: Input,
  input_model: InputMode,
  title: String,
  placeholder: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum InputMode {
  #[default]
  Normal,
  Editing,
}

impl InputField {
  pub fn new(title: String, placeholder: Option<String>) -> Self {
    Self {
      input: Input::default(),
      input_model: InputMode::Normal,
      title,
      placeholder,
    }
  }

  pub fn set_mode(&mut self, mode: InputMode) {
    self.input_model = mode;
  }

  pub fn value(&self) -> String {
    self.input.value().trim().to_string()
  }

  pub fn render(&self, frame: &mut Frame, area: Rect) {
    let width = area.width.max(3) - 3;
    let scroll = self.input.visual_scroll(width as usize);

    let style: Style = match self.input_model {
      InputMode::Normal => Style::default(),
      InputMode::Editing => Color::Blue.into(),
    };

    let text = if self.input.value().is_empty() {
      if let Some(placeholder) = &self.placeholder {
        Span::styled(placeholder, Style::default().fg(Color::DarkGray))
      } else {
        Span::raw("")
      }
    } else {
      Span::styled(self.input.value(), Style::default().fg(Color::White))
    };

    let input = Paragraph::new(text)
      .style(style)
      .scroll((0, scroll as u16))
      .block(Block::bordered().title(self.title.clone()));
    frame.render_widget(input, area);

    if self.input_model == InputMode::Editing {
      let x = self.input.visual_cursor().max(scroll) - scroll + 1;
      frame.set_cursor_position((area.x + x as u16, area.y + 1));
    }
  }

  pub fn handle_event(&mut self, event: &Event) {
    if self.input_model == InputMode::Editing {
      self.input.handle_event(event);
    }
  }
}
