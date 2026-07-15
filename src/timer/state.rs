use crate::timer::utils::time_for_session;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SessionType {
  Work,
  ShortBreak,
  LongBreak,
}

/// Main state of the timer
pub struct TimerState {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
  pub session_number: u64,
}

/// Duplicate of timer state just to take snapshot to serialize it to communicate between processes
#[derive(Serialize, Deserialize)]
pub struct TimerSnapShot {
  pub running: bool,
  pub time_remaining: Duration,
  pub session_type: SessionType,
  pub session_number: u64,
}

impl From<&TimerState> for TimerSnapShot {
  fn from(t: &TimerState) -> Self {
    Self {
      running: t.running,
      time_remaining: t.time_remaining,
      session_type: t.session_type,
      session_number: t.session_number,
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
    }
  }
}

impl Default for TimerState {
  fn default() -> Self {
    TimerState {
      running: false,
      time_remaining: time_for_session(SessionType::Work),
      session_type: SessionType::Work,
      session_number: 1,
    }
  }
}
