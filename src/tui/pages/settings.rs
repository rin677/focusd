use crate::{timer::state::TimerState, utils::figlet::big_text};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget},
};

pub fn show_settings(timer_state: &TimerState, area: Rect, buffer: &mut Buffer) {
  // TODO: Implement this
  let text = big_text("Settings will be here");
  Paragraph::new(text).centered().render(area, buffer);
}
