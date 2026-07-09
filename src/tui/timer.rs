use figlet_rs::FIGlet;

use crate::timer::{engine::render_time, state::TimerState};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget},
};

pub fn show_timer(timer_state: &TimerState, area: Rect, buffer: &mut Buffer) {
  let t = render_time(timer_state);
  let font = FIGlet::from_content(include_str!("../../resources/terminus.flf")).unwrap();
  // TODO: Also show session type and paused play (removed currently to show figlet)
  let text = font.convert(&t).unwrap().to_string();
  Paragraph::new(text).centered().render(area, buffer);
}
