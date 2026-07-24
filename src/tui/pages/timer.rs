use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
  config::settings::get_config,
  timer::{
    engine::render_time,
    state::{SessionType, TimerState},
    utils::time_for_session,
  },
  tui::merge_block::MergeBlock,
  utils::figlet::big_text,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::Modifier,
  widgets::{LineGauge, Paragraph},
};

pub fn show_timer(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let area = MergeBlock::new("").top().render(frame, area);
  let cfg = get_config();
  if cfg.tui_show_stats {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(9)]);
    let [first, second] = area.layout(&layout);
    render_timer_no_stats(timer_state, first, frame);
    render_stats(second, frame);
  } else {
    render_timer_no_stats(timer_state, area, frame);
  }
}

fn render_timer_no_stats(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let cfg = get_config();
  let show_art = cfg.tui_show_ascii_art;
  let show_progress = cfg.tui_show_progress;
  let mut constrains: Vec<Constraint> = Vec::new();
  let mut h = 6;
  if show_art {
    h += 23;
    constrains.push(Constraint::Length(23));
  }
  constrains.push(Constraint::Length(6));
  if show_progress {
    h += 1;
    constrains.push(Constraint::Length(1));
  }
  let centered = area.centered(Constraint::Max(38), Constraint::Max(h));
  let layout = Layout::vertical(constrains);
  let areas = centered.layout_vec(&layout);

  if show_art {
    render_ascssi_art(timer_state, areas[0], frame);
  }
  let text_area = if show_art { areas[1] } else { areas[0] };
  render_text(timer_state, text_area, frame);
  if show_progress {
    let gauge_area = if show_art { areas[2] } else { areas[1] };
    render_gauge(timer_state, gauge_area, frame);
  }

  // TODO: Include other things as well, dashboard with presets, and today's stats.
}

fn render_stats(area: Rect, frame: &mut Frame) {
  let inner = MergeBlock::new(" Today's stats ").top().render(frame, area);
  let text = big_text("Stats loading ...");
  frame.render_widget(Paragraph::new(text).centered(), inner);
}

fn render_text(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let t = render_time(timer_state);
  let text = big_text(&t);
  frame.render_widget(Paragraph::new(text).centered(), area);
}

fn render_gauge(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
  let percent = (timer_state.time_remaining.as_secs()) as f64
    / (time_for_session(timer_state.session_type).as_secs()) as f64;
  let progress = LineGauge::default()
    .style(Modifier::BOLD)
    .filled_symbol("█")
    .unfilled_symbol("░")
    .label("")
    .ratio(1_f64 - percent);
  frame.render_widget(progress, area);
}

fn render_ascssi_art(timer_state: &TimerState, area: Rect, frame: &mut Frame) {
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
