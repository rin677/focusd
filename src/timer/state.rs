use crate::timer::utils::time_for_session;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SessionType {
  Work,
  ShortBreak,
  LongBreak,
}

#[derive(Serialize, Deserialize)]
pub struct TimerState {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
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
