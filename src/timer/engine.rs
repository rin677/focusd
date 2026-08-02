use crate::{
  config::settings::{get_config, get_crr_preset, set_config_value},
  database::history::add_session_to_db,
  notification::show_complete_notification,
  throw,
  timer::{
    hooks::session_start_hook,
    state::{SessionType, TimerState},
    utils::{min_2_digit, next_session as next_session_name, time_for_session},
  },
};
use std::time::Duration;
use toml_edit::value;

pub fn decrease_sec(state: &mut TimerState) {
  let Some(new_time) = state.time_remaining.checked_sub(Duration::from_secs(1)) else {
    on_finish(state);
    return;
  };
  state.time_remaining = new_time;
  if new_time.is_zero() {
    on_finish(state);
  }
}

pub fn on_finish(state: &mut TimerState) {
  next_session(state);
  session_start_hook(state.session_type);
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
  session_start_hook(session_type);
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

pub fn select_preset(state: &mut TimerState, preset_name: &str) -> std::io::Result<()> {
  let config = get_config();
  let preset = config.presets.get(preset_name);
  match preset {
    Some(_) => {
      set_config_value("active_preset", value(preset_name)).unwrap();
      state.session_number = 1;
      state.session_type = SessionType::Work;
      reset_session(state);
      Ok(())
    }
    None => {
      throw!(format!("Preset {preset_name} not found"));
    }
  }
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

  session_start_hook(state.session_type);
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
