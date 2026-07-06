use crate::timer::state::{SessionType, TimerState};

pub fn render_time(state: &TimerState) {
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

pub fn start_session(state: &mut TimerState) {
  state.running = true
}

pub fn pause_session(state: &mut TimerState) {
  state.running = false
}

pub fn resume_session(state: &mut TimerState) {
  state.running = true
}

pub fn toggle_session(state: &mut TimerState) {
  state.running = !state.running
}

pub fn skip_sesion(state: &mut TimerState) {
  // TODO: I will do this soon
  pause_session(state);
}
