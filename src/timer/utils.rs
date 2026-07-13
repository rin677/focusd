use std::time::Duration;

use crate::{
  config::settings::{POMODORO_PRESET, get_config},
  timer::state::SessionType,
};

pub fn name_for_session<'a>(session: SessionType) -> &'a str {
  match session {
    SessionType::Work => "Work",
    SessionType::ShortBreak => "Short Break",
    SessionType::LongBreak => "Long Break",
  }
}

pub fn get_session_type(name: &str) -> SessionType {
  match name.to_lowercase().as_str() {
    "work" => SessionType::Work,
    "short break" => SessionType::ShortBreak,
    _ => SessionType::LongBreak,
  }
}

pub fn time_for_session(session: SessionType) -> Duration {
  let config = get_config();
  let preset = config
    .presets
    .get(&config.active_preset)
    .copied()
    .unwrap_or(POMODORO_PRESET);

  match session {
    SessionType::Work => Duration::from_mins(preset.work_minutes),
    SessionType::ShortBreak => Duration::from_mins(preset.short_break_minutes),
    SessionType::LongBreak => Duration::from_mins(preset.long_breka_minutes),
  }
}

pub fn next_session(session: SessionType) -> SessionType {
  // TODO: handle long break later

  match session {
    SessionType::Work => SessionType::ShortBreak,
    SessionType::ShortBreak => SessionType::Work,
    SessionType::LongBreak => SessionType::Work,
  }
}

pub fn min_2_digit(time: u64) -> String {
  if time >= 10 {
    format!("{time}")
  } else {
    format!("0{time}")
  }
}
