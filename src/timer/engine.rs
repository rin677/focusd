use crate::timer::state::{SessionType, TimerState};

pub fn render_time(state: &mut TimerState) {
  let session_type = match &state.sessioin_type {
    SessionType::Work => "Work",
    SessionType::ShortBreak => "Short Break",
    SessionType::LongBreak => "Long Break",
  };
  let sec_left = state.time_remaining.as_secs();
  let min_left = sec_left / 60;
  let sec_to_show = sec_left % 60;
  let icon = if state.running { "" } else { "" };
  println!("{icon} - {session_type} - {min_left}:{sec_to_show}");

  // TODO: I will do this soon;
}

pub fn start_session() {
  // TODO: I will do this soon
}

pub fn pause_session() {
  // TODO: I will do this soon
}

pub fn resume_session() {
  // TODO: I will do this soon
}

pub fn toggle_session() {
  // if (session)
}

pub fn skip_sesion() {
  // TODO: I will do this soon
}
