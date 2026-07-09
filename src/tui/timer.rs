use figlet_rs::FIGlet;

use crate::timer::{engine::render_time, state::TimerState, utils::time_for_session};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::Modifier,
  widgets::{Gauge, Paragraph},
};

pub fn show_timer(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let layout = Layout::vertical([Constraint::Length(20), Constraint::Max(2)]).spacing(2);
  let [first, second] = area.layout(&layout);
  let percent = (timer_state.time_remaining.as_secs()) as f64
    / (time_for_session(timer_state.session_type).as_secs()) as f64
    * 100_f64;
  let t = render_time(timer_state);
  let font = FIGlet::from_content(include_str!("../../resources/terminus.flf")).unwrap();
  let text = font.convert(&t).unwrap().to_string();
  frame.render_widget(Paragraph::new(text).centered(), first);
  let progress = Gauge::default()
    .style(Modifier::BOLD)
    .percent((100_f64 - percent) as u16);
  frame.render_widget(progress, second);
}
