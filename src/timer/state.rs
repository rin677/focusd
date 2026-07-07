use std::time::Duration;

use crate::timer::utils::time_for_session;

#[derive(Clone, Copy)]
pub enum SessionType {
  Work,
  ShortBreak,
  LongBreak,
}

pub struct TimerState {
  pub running: bool,
  pub time_remaining: Duration,
  pub sessioin_type: SessionType,
}

impl Default for TimerState {
  fn default() -> Self {
    TimerState {
      running: false,
      time_remaining: time_for_session(SessionType::Work),
      sessioin_type: SessionType::Work,
    }
  }
}
