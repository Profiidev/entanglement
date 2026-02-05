use ratatui::{
  Frame,
  layout::{Constraint, Layout},
  style::{Color, Style},
  widgets::{Block, Padding, Paragraph},
};

use crate::input::InputField;

pub fn app_url(frame: &mut Frame, field: &InputField, error: &Option<String>) {
  let block = Block::bordered();
  let area = block.inner(frame.area());
  frame.render_widget(block, frame.area());

  let [text_area, input_area, error_area] = Layout::vertical([
    Constraint::Length(1),
    Constraint::Length(3),
    Constraint::Length(1),
  ])
  .areas(area);

  let text_block = Block::new().padding(Padding::left(1));
  let text_block_area = text_block.inner(text_area);
  frame.render_widget(text_block, text_area);
  let text = Paragraph::new("Enter the URL of your Entanglement app:");
  frame.render_widget(text, text_block_area);

  field.render(frame, input_area);

  if let Some(error) = error {
    let error_block = Block::new().padding(Padding::left(1));
    let error_block_area = error_block.inner(error_area);
    frame.render_widget(error_block, error_area);
    let error_text = Paragraph::new(error.clone()).style(Style::default().fg(Color::Red));
    frame.render_widget(error_text, error_block_area);
  }
}

pub fn wait_for_auth(frame: &mut Frame, app_url: &str) {
  let block = Block::bordered();
  let area = block.inner(frame.area());
  frame.render_widget(block, frame.area());

  let text_block = Block::new().padding(Padding::left(1));
  let text_block_area = text_block.inner(area);
  frame.render_widget(text_block, area);

  let [info_area, url_area, waiting_area] = Layout::vertical([
    Constraint::Length(2),
    Constraint::Length(3),
    Constraint::Length(2),
  ])
  .areas(text_block_area);

  let info_text = Paragraph::new(
    "Press Enter to open the browser and authenticate with your app.\nOr visit the following URL:",
  );
  frame.render_widget(info_text, info_area);

  let info_block = Block::bordered().padding(Padding::left(1));
  let info_block_area = info_block.inner(url_area);
  frame.render_widget(info_block, url_area);
  let url_text =
    Paragraph::new(format!("{}auth/cli", app_url)).style(Style::default().fg(Color::Blue));
  frame.render_widget(url_text, info_block_area);

  let waiting_text = Paragraph::new(
    "If the URL is not correct press Backspace to edit it.\nWaiting for authentication...",
  );
  frame.render_widget(waiting_text, waiting_area);
}
