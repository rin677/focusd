use crate::{
  config::settings::{get_config, get_crr_preset},
  daemon::commands::get_timer_state,
  database::history::get_full_history_no_err,
  stats::calculate::{DurType, get_current_streak, get_todays_sessions, get_total_time},
  timer::{engine::render_time, state::SessionType},
  utils::timer::render_duration,
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
  sessions_today: usize,
  focused_today: String,
  daily_goal: String,
  current_streak: usize,
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

  let total_time = state.session_type.get_time();
  let percentage = (100.0
    - (100_f64 * state.time_remaining.as_secs() as f64 / total_time.as_secs() as f64))
    as usize;

  let current_session = state.session_type.name();
  let session_n = state.session_number;
  let total_sessions = preset.sessions_before_long_break;
  let next_session = state.session_type.next(state.session_number);
  let next_session_name = next_session.name();
  let time_for_next_sessoin = render_duration(next_session.get_time().as_secs());
  let completed_today = render_duration(get_total_time(DurType::Today));
  let sessions_today = get_todays_sessions();
  let daily_goal = render_duration(get_config().daily_goal_minutes * 60);
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
    sessions_today: sessions_today.try_into().unwrap_or(0),
    focused_today: completed_today,
    daily_goal,
    current_streak: current_streak.try_into().unwrap_or(0),
  };

  println!("{}", serde_json::to_string(&module).unwrap());
}
