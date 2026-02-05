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
