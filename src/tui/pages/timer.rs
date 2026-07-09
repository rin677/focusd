use crate::{
  timer::{engine::render_time, state::TimerState, utils::time_for_session},
  utils::figlet::big_text,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::Modifier,
  widgets::{LineGauge, Paragraph},
};

pub fn show_timer(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let centered = area.centered(Constraint::Max(50), Constraint::Max(7));
  let layout = Layout::vertical([Constraint::Length(6), Constraint::Max(1)]);
  let [first, second] = centered.layout(&layout);
  let percent = (timer_state.time_remaining.as_secs()) as f64
    / (time_for_session(timer_state.session_type).as_secs()) as f64;
  let t = render_time(timer_state);
  let text = big_text(&t);
  frame.render_widget(Paragraph::new(text).centered(), first);
  let progress = LineGauge::default()
    .style(Modifier::BOLD)
    .filled_symbol("█")
    .unfilled_symbol("░")
    .label("")
    .ratio(1_f64 - percent);

  // TODO: Include other things as well, ASCCI art or dashboard.

  frame.render_widget(progress, second);
}
