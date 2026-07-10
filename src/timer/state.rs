use crate::timer::utils::time_for_session;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SessionType {
  Work,
  ShortBreak,
  LongBreak,
}

pub struct TimerState {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
}

#[derive(Serialize, Deserialize)]
pub struct TimerSnapShot {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
}

impl From<&TimerState> for TimerSnapShot {
  fn from(t: &TimerState) -> Self {
    Self {
      running: t.running,
      time_remaining: t.time_remaining,
      session_type: t.session_type,
    }
  }
}

impl Default for TimerState {
  fn default() -> Self {
    TimerState {
      running: false,
      time_remaining: time_for_session(SessionType::Work),
      session_type: SessionType::Work,
    }
  }
}
