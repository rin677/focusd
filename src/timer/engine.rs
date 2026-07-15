use crate::{
  config::settings::get_crr_preset,
  database::history::add_session_to_db,
  notification::show_complete_notification,
  timer::{
    state::{SessionType, TimerState},
    utils::{min_2_digit, next_session as next_session_name, time_for_session},
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

/// Returns readable time given state of timer
pub fn render_time(state: &TimerState) -> String {
  let sec_left = state.time_remaining.as_secs();
  let min_left = min_2_digit(sec_left / 60);
  let s = sec_left % 60;
  let sec_to_show = min_2_digit(s);
  format!("{min_left}:{sec_to_show}")
}

pub fn start_session(state: &mut TimerState) {
  state.time_remaining = time_for_session(state.session_type);
  state.running = true
}

pub fn reset_session(state: &mut TimerState) {
  state.running = false;
  state.time_remaining = time_for_session(state.session_type);
}

/// Goes next sesson
///
/// Adds current session to db
/// and shows notification
pub fn next_session(state: &mut TimerState) {
  add_session_to_db(state).ok();
  show_complete_notification(state);
  state.session_type = next_session_name(state.session_type, state.session_number);
  if state.session_type == SessionType::Work {
    let preset = get_crr_preset();
    state.session_number = if preset.sessions_before_long_break == state.session_number {
      1
    } else {
      state.session_number + 1
    }
  }

  state.time_remaining = time_for_session(state.session_type);
}
