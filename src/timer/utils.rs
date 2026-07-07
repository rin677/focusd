use std::time::Duration;

use crate::timer::state::SessionType;

pub fn name_for_session(session: SessionType) -> String {
  match session {
    SessionType::Work => String::from("Work"),
    SessionType::ShortBreak => String::from("Short Break"),
    SessionType::LongBreak => String::from("Long Break"),
  }
}

pub fn time_for_session(session: SessionType) -> Duration {
  match session {
    SessionType::Work => Duration::from_mins(25),
    SessionType::ShortBreak => Duration::from_mins(5),
    SessionType::LongBreak => Duration::from_mins(10),
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
