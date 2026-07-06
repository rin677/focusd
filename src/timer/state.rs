use std::time::Duration;

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
