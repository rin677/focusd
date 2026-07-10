use crate::{
  database::history::add_session_to_db,
  notification::show_complete_notification,
  timer::{
    state::TimerState,
    utils::{min_2_digit, name_for_session, next_session as next_session_name, time_for_session},
  },
};
use std::time::Duration;

pub fn decrease_sec(state: &mut TimerState) {
  let new_time = state.time_remaining - Duration::from_secs(1);
  state.time_remaining = new_time;
  if new_time.is_zero() {
    next_session(state);
    on_finish();
  }
}

pub fn on_finish() {
  // TODO: Maybe notification or some hook
}

pub fn render_time(state: &TimerState) -> String {
  // TODO: Also show session type and paused play (removed currently to show figlet)
  // let session_type = name_for_session(state.session_type);
  let sec_left = state.time_remaining.as_secs();
  let min_left = min_2_digit(sec_left / 60);
  let s = sec_left % 60;
  let sec_to_show = min_2_digit(s);
  // let icon = if state.running { "" } else { "" };
  format!("{min_left}:{sec_to_show}")
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

pub fn next_session(state: &mut TimerState) {
  add_session_to_db();
  show_complete_notification(state);
  state.session_type = next_session_name(state.session_type);
  state.time_remaining = time_for_session(state.session_type);
}
