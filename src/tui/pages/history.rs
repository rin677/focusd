use crate::{timer::state::TimerState, utils::figlet::big_text};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget},
};

pub fn show_history(timer_state: &TimerState, area: Rect, buffer: &mut Buffer) {
  // TODO: Implement this
  let text = big_text("History will be here");
  Paragraph::new(text).centered().render(area, buffer);
}
