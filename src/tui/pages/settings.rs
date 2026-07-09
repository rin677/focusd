use figlet_rs::FIGlet;

use crate::timer::state::TimerState;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget},
};

pub fn show_settings(timer_state: &TimerState, area: Rect, buffer: &mut Buffer) {
  // TODO: Implement this
  let font = FIGlet::from_content(include_str!("../../../resources/terminus.flf")).unwrap();
  let text = font.convert("Settings will be here").unwrap().to_string();
  Paragraph::new(text).centered().render(area, buffer);
}
