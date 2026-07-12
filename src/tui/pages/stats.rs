use crate::utils::figlet::big_text;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget},
};

pub fn show_stats(area: Rect, buffer: &mut Buffer) {
  // TODO: Implement this
  let text = big_text("Stats will be here");
  Paragraph::new(text).centered().render(area, buffer);
}
