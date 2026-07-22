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
  let Some(new_time) = state.time_remaining.checked_sub(Duration::from_secs(1)) else {
    next_session(state);
    on_finish();
    return;
  };
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

/// Start timer while going to specific session
pub fn start_specific_session(state: &mut TimerState, session_type: SessionType) {
  let is_current_session = state.session_type == session_type;
  state.session_type = session_type;
  state.time_remaining = time_for_session(session_type);
  state.running = true;
  if !is_current_session {
    state.session_number = match session_type {
      SessionType::Work => get_next_session_number(state.session_number),
      SessionType::LongBreak => get_crr_preset().sessions_before_long_break,
      SessionType::ShortBreak => state.session_number,
    }
  }
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
    state.session_number = get_next_session_number(state.session_number);
  }

  state.time_remaining = time_for_session(state.session_type);
}

fn get_next_session_number(crr_session_number: u64) -> u64 {
  let preset = get_crr_preset();
  if preset.sessions_before_long_break == crr_session_number {
    1
  } else {
    crr_session_number + 1
  }
}
