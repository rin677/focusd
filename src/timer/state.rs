use crate::config::settings::get_crr_preset;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SessionType {
  Work,
  ShortBreak,
  LongBreak,
}

impl SessionType {
  pub fn name<'a>(self) -> &'a str {
    match self {
      Self::Work => "Work",
      Self::ShortBreak => "Short Break",
      Self::LongBreak => "Long Break",
    }
  }

  /// Session type from string
  pub fn from_string(name: &str) -> Self {
    match name.trim().to_lowercase().as_str() {
      "work" => Self::Work,
      "short break" => Self::ShortBreak,
      _ => Self::LongBreak,
    }
  }

  /// Total time for specific session
  pub fn get_time(self) -> Duration {
    let preset = get_crr_preset();

    match self {
      Self::Work => Duration::from_mins(preset.work_minutes),
      Self::ShortBreak => Duration::from_mins(preset.short_break_minutes),
      Self::LongBreak => Duration::from_mins(preset.long_break_minutes),
    }
  }

  /// Next session
  pub fn next(self, prev_session_number: u64) -> SessionType {
    match self {
      Self::Work => {
        let preset = get_crr_preset();
        if preset.sessions_before_long_break == prev_session_number {
          Self::LongBreak
        } else {
          Self::ShortBreak
        }
      }
      Self::ShortBreak => Self::Work,
      Self::LongBreak => Self::Work,
    }
  }
}

/// Main state of the timer
pub struct TimerState {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
  pub session_number: u64,
  pub added_time: Duration,
  pub category: String,
}

/// Duplicate of timer state just to take snapshot to serialize it to communicate between processes
#[derive(Serialize, Deserialize)]
pub struct TimerSnapShot {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
  pub session_number: u64,
  pub added_time: Duration,
  #[serde(default = "default_category")]
  pub category: String,
}

fn default_category() -> String {
  "General".to_string()
}

impl From<&TimerState> for TimerSnapShot {
  fn from(t: &TimerState) -> Self {
    Self {
      running: t.running,
      time_remaining: t.time_remaining,
      session_type: t.session_type,
      session_number: t.session_number,
      added_time: t.added_time,
      category: t.category.clone(),
    }
  }
}

impl From<&TimerSnapShot> for TimerState {
  fn from(t: &TimerSnapShot) -> Self {
    Self {
      running: t.running,
      time_remaining: t.time_remaining,
      session_type: t.session_type,
      session_number: t.session_number,
      added_time: t.added_time,
      category: t.category.clone(),
    }
  }
}

impl Default for TimerState {
  fn default() -> Self {
    TimerState {
      running: false,
      time_remaining: SessionType::Work.get_time(),
      session_type: SessionType::Work,
      session_number: 1,
      added_time: Duration::from_secs(0),
      category: crate::config::settings::get_config().active_category,
    }
  }
}
