use std::time::Duration;

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
