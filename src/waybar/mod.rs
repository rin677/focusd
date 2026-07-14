use crate::{
  config::settings::get_crr_preset,
  daemon::commands::get_timer_state,
  timer::{
    engine::render_time,
    state::SessionType,
    utils::{name_for_session, time_for_session},
  },
  utils::ignore::Ignore,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WaybarModule {
  text: String,
  alt: String,
  class: Vec<String>,
  percentage: f64,
  tooltip: String,
}

pub fn status() {
  let state = get_timer_state().ignore();
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

  let icon = if state.running { "" } else { "" };

  let total_time = time_for_session(state.session_type);
  let percentage = 100_f64 * state.time_remaining.as_secs() as f64 / total_time.as_secs() as f64;

  let tooltip = format!(
    "{} {}/{} \n {icon} {}",
    name_for_session(state.session_type),
    state.session_number,
    preset.sessions_before_long_break,
    text
  );

  let module = WaybarModule {
    text,
    alt,
    class,
    percentage,
    tooltip,
  };

  println!("{}", serde_json::to_string(&module).unwrap());
}
