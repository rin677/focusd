use crate::{
  config::{
    settings::get_config,
    themes::{get_current_theme, themed_block},
  },
  daemon::commands::{PayloadMessage, send_message_with_payload},
  database::history::get_full_history_no_err,
  stats::calculate::{DurType, get_current_streak, get_todays_sessions, get_total_time},
  timer::{
    engine::render_time,
    state::{SessionType, TimerState},
  },
  tui::layout::split_vertical,
  utils::{cycle::cycle_index, figlet::big_text, ignore::IgnoreType, timer::render_duration},
};

use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect, Spacing},
  prelude::*,
  widgets::{LineGauge, Paragraph},
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tui_widget_list::{ListBuilder, ListState, ListView};

#[derive(Default)]
pub struct TimerPage {
  pub preset_selected_index: usize,
}

impl TimerPage {
  pub fn render(&mut self, timer_state: &TimerState, area: Rect, frame: &mut Frame) {
    let cfg = get_config();
    if cfg.tui_show_stats {
      let layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(6)]).spacing(Spacing::Overlap(1));
      let [first, second] = area.layout(&layout);
      self.render_timer_no_stats(timer_state, first, frame);
      self.render_stats_and_presets(second, frame);
    } else {
      self.render_timer_no_stats(timer_state, area, frame);
    }
  }

  pub fn up(&mut self) {
    self.up_or_down(-1);
  }
  pub fn down(&mut self) {
    self.up_or_down(1);
  }
  fn up_or_down(&mut self, offset: isize) {
    let count = get_config().presets.len();
    self.preset_selected_index = cycle_index(self.preset_selected_index, count, offset)
  }

  pub fn select_preset(&mut self) {
    let cfg = get_config();
    let presets = cfg.presets;
    let mut names: Vec<&str> = presets.keys().map(|s| s.as_str()).collect();
    names.sort();
    let preset = names.get(self.preset_selected_index);
    if let Some(p) = preset {
      send_message_with_payload(PayloadMessage::SelectPreset, p.to_string().into()).ignore_type();
    }
  }

  fn render_timer_no_stats(&self, timer_state: &TimerState, area: Rect, frame: &mut Frame) {
    let t = render_time(timer_state);
    let text = big_text(&t);

    let cfg = get_config();
    let show_art = cfg.tui_show_ascii_art;
    let show_progress = cfg.tui_show_progress;
    let mut constrains: Vec<Constraint> = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let text_height = lines.len() as u16;
    let text_width = lines
      .iter()
      .map(|line| line.chars().count())
      .max()
      .unwrap_or(4) as u16;
    let mut h = text_height + 1;
    if show_art {
      h += 23;
      constrains.push(Constraint::Length(23));
    }
    constrains.push(Constraint::Length(text_height + 1));
    if show_progress {
      h += 1;
      constrains.push(Constraint::Length(1));
    }
    let centered = area.centered(Constraint::Max(text_width.max(38)), Constraint::Max(h));
    let layout = Layout::vertical(constrains);
    let areas = centered.layout_vec(&layout);

    if show_art {
      self.render_ascssi_art(timer_state, areas[0], frame);
    }

    let text_area = if show_art { areas[1] } else { areas[0] };
    frame.render_widget(Paragraph::new(text).centered(), text_area);

    if show_progress {
      let gauge_area = if show_art { areas[2] } else { areas[1] };
      self.render_gauge(timer_state, gauge_area, frame);
    }
  }

  fn render_stats_and_presets(&mut self, area: Rect, frame: &mut Frame) {
    let (preset_area, stats_area) = split_vertical(area);
    self.render_presets(preset_area, frame);
    self.render_stats(stats_area, frame);
  }

  fn render_stats(&self, area: Rect, frame: &mut Frame) {
    let block = themed_block().title(" Today's stats ");

    frame.render_widget(&block, area);
    let inner = block.inner(area);
    let history = get_full_history_no_err();
    let focused = format!(
      "Focused: {}",
      render_duration(get_total_time(DurType::Today))
    );
    let goal = format!(
      "Goal: {}",
      render_duration(get_config().daily_goal_minutes * 60)
    );
    let streak = format!("Streak: {}", get_current_streak(history));
    let sessions = format!("Sessions: {}", get_todays_sessions());
    let text = format!(" {focused}\n {goal}\n {sessions}\n {streak}");
    frame.render_widget(Paragraph::new(text), inner);
  }

  fn render_presets(&mut self, area: Rect, frame: &mut Frame) {
    let theme = get_current_theme();
    let block = themed_block().title(" Presets ");

    frame.render_widget(&block, area);
    let area = block.inner(area);
    let cfg = get_config();
    let presets = cfg.presets;

    let mut names: Vec<&str> = presets.keys().map(|s| s.as_str()).collect();
    names.sort();
    let count = names.len();

    let active = cfg.active_preset.clone();
    let builder = ListBuilder::new(|context| {
      let text = match names.get(context.index) {
        Some(name) => {
          let item = presets.get(*name).copied().unwrap();
          format!(
            "{} {} {}/{}/{} x {}",
            if *name == active { ">" } else { " " },
            name,
            item.work_minutes,
            item.short_break_minutes,
            item.long_break_minutes,
            item.sessions_before_long_break
          )
        }
        None => String::new(),
      };
      let mut item = Line::from(text);
      if context.is_selected {
        item = item.style(Style::default().bg(theme.selection));
      }
      (item, 1)
    });

    if self.preset_selected_index >= count && count > 0 {
      self.preset_selected_index = count - 1;
    }
    let mut state = ListState::default();
    state.select(Some(self.preset_selected_index));
    let list = ListView::new(builder, count);
    list.render(area, frame.buffer_mut(), &mut state);
  }

  fn render_gauge(&self, timer_state: &TimerState, area: Rect, frame: &mut Frame) {
    let percent = (timer_state.time_remaining.as_secs()) as f64
      / (timer_state.session_type.get_time().as_secs()) as f64;
    let theme = get_current_theme();
    let progress = LineGauge::default()
      .style(Style::default().fg(theme.secondary))
      .filled_symbol("█")
      .unfilled_symbol("░")
      .label("")
      .ratio(1f64 - percent.clamp(0f64, 1f64));
    frame.render_widget(progress, area);
  }

  fn render_ascssi_art(&self, timer_state: &TimerState, area: Rect, frame: &mut Frame) {
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

    let lines: Vec<&str> = crr_frame.lines().collect();
    let frame_height = lines.len() as u16;
    let frame_width = lines
      .iter()
      .map(|line| line.chars().count())
      .max()
      .unwrap_or(4) as u16;

    let centered = area.centered(Constraint::Max(frame_width), Constraint::Max(frame_height));
    frame.render_widget(Paragraph::new(crr_frame), centered);
  }
}
