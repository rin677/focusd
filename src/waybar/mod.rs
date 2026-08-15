use crate::{
  config::settings::get_crr_preset,
  daemon::commands::get_timer_state,
  database::history::get_full_history_no_err,
  stats::calculate::{DurType, get_current_streak, get_total_time},
  timer::{
    engine::render_time,
    state::SessionType,
    utils::{name_for_session, next_session, time_for_session},
  },
  utils::times_ago::render_duration,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WaybarModule {
  text: String,
  alt: String,
  class: Vec<String>,
  percentage: usize,
  tooltip: String,
  session: String,
  next_session: String,
}

/// Gives status in JSON format mainly to be used in waybar
pub fn status() {
  let state = get_timer_state().unwrap();
  let text = render_time(&state);

  let mut class: Vec<String> = Vec::new();
  let mut alt = match state.session_type {
    SessionType::Work => "work".to_string(),
    SessionType::ShortBreak => "short-break".to_string(),
    SessionType::LongBreak => "long-break".to_string(),
  };
  class.push(alt.clone());
  if !state.running {
    class.push("paused".to_string());
    alt = format!("{alt}-paused");
  }
  let preset = get_crr_preset();

  let total_time = time_for_session(state.session_type);
  let percentage = (100.0
    - (100_f64 * state.time_remaining.as_secs() as f64 / total_time.as_secs() as f64))
    as usize;

  let current_session = name_for_session(state.session_type);
  let session_n = state.session_number;
  let total_sessions = preset.sessions_before_long_break;
  let next_session = next_session(state.session_type, state.session_number);
  let next_session_name = name_for_session(next_session);
  let time_for_next_sessoin = render_duration(time_for_session(next_session).as_secs());
  let completed_today = render_duration(get_total_time(DurType::Today));
  let history = get_full_history_no_err();
  let current_streak = get_current_streak(history);
  let streak_unit = if current_streak <= 1 { "day" } else { "days" };

  let tooltip = format!(
    "{current_session} Session,
Cycle {session_n}/{total_sessions}

Next Session: {next_session_name} ({time_for_next_sessoin}),

Focused today: {completed_today},
Current Streak: {current_streak} {streak_unit}
"
  );

  let module = WaybarModule {
    text,
    alt,
    class,
    percentage,
    tooltip,
    session: current_session.to_string(),
    next_session: next_session_name.to_string(),
  };

  println!("{}", serde_json::to_string(&module).unwrap());
}
