use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
  timer::{
    engine::render_time,
    state::{SessionType, TimerState},
    utils::time_for_session,
  },
  utils::figlet::big_text,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::Modifier,
  widgets::{LineGauge, Paragraph},
};

pub fn show_timer(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let centered = area.centered(Constraint::Max(38), Constraint::Max(30));
  let layout = Layout::vertical([
    Constraint::Length(23),
    Constraint::Length(6),
    Constraint::Max(1),
  ]);
  let [art, first, second] = centered.layout(&layout);
  let percent = (timer_state.time_remaining.as_secs()) as f64
    / (time_for_session(timer_state.session_type).as_secs()) as f64;
  let t = render_time(timer_state);
  render_ascssi(timer_state, art, frame);

  let text = big_text(&t);
  frame.render_widget(Paragraph::new(text).centered(), first);
  let progress = LineGauge::default()
    .style(Modifier::BOLD)
    .filled_symbol("█")
    .unfilled_symbol("░")
    .label("")
    .ratio(1_f64 - percent);

  // TODO: Include other things as well, dashboard with presets, and today's stats.

  frame.render_widget(progress, second);
}

fn render_ascssi(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let clock_frames = vec![
    include_str!("../../../resources/clock/1.txt"),
    include_str!("../../../resources/clock/2.txt"),
    include_str!("../../../resources/clock/3.txt"),
    include_str!("../../../resources/clock/4.txt"),
  ];
  let coffee_frames = vec![
    include_str!("../../../resources/coffee/1.txt"),
    include_str!("../../../resources/coffee/2.txt"),
    include_str!("../../../resources/coffee/3.txt"),
    include_str!("../../../resources/coffee/4.txt"),
    include_str!("../../../resources/coffee/5.txt"),
  ];

  let session_frames = match timer_state.session_type {
    SessionType::Work => clock_frames,
    SessionType::LongBreak => coffee_frames,
    SessionType::ShortBreak => coffee_frames,
  };

  let millis = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or(Duration::ZERO)
    .as_millis();

  let frame_index = if timer_state.running {
    (millis / 400) as usize % session_frames.len()
  } else {
    0
  };

  let crr_frame = session_frames
    .get(frame_index)
    .copied()
    .unwrap_or(session_frames[0]);
  frame.render_widget(Paragraph::new(crr_frame), area);
}
