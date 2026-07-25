use std::time::Duration;

use crate::{config::settings::get_crr_preset, timer::state::SessionType};

pub fn name_for_session<'a>(session: SessionType) -> &'a str {
  match session {
    SessionType::Work => "Work",
    SessionType::ShortBreak => "Short Break",
    SessionType::LongBreak => "Long Break",
  }
}

pub fn get_session_type(name: &str) -> SessionType {
  match name.trim().to_lowercase().as_str() {
    "work" => SessionType::Work,
    "short break" => SessionType::ShortBreak,
    _ => SessionType::LongBreak,
  }
}

pub fn time_for_session(session: SessionType) -> Duration {
  let preset = get_crr_preset();

  match session {
    SessionType::Work => Duration::from_mins(preset.work_minutes),
    SessionType::ShortBreak => Duration::from_mins(preset.short_break_minutes),
    SessionType::LongBreak => Duration::from_mins(preset.long_break_minutes),
  }
}

pub fn next_session(session: SessionType, prev_session_number: u64) -> SessionType {
  match session {
    SessionType::Work => {
      let preset = get_crr_preset();
      if preset.sessions_before_long_break == prev_session_number {
        SessionType::LongBreak
      } else {
        SessionType::ShortBreak
      }
    }
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
