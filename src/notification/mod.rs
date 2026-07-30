use notify_rust::Notification;

use crate::{
  config::settings::get_config,
  stats::calculate::{DurType, get_total_time},
  timer::{
    state::TimerState,
    utils::{name_for_session, next_session as next_session_name, time_for_session},
  },
};

/// Show notification after sessioni is complete
pub fn show_complete_notification(state: &TimerState) {
  let config = get_config();
  let session_name = name_for_session(state.session_type);
  if config.show_notifications {
    if state.time_remaining.is_zero() {
      // Timer completed notification
      Notification::new()
        .summary(&format!("Timer for {} finished", session_name))
        .body(&format!(
          "Congrulations in completing timer for {} minutes, Now it's time for {}",
          time_for_session(state.session_type).as_secs() / 60,
          name_for_session(next_session_name(state.session_type, state.session_number))
        ))
        .show()
        .ok();
    }

    // Goal completed notification
    let goal_min = get_config().daily_goal_minutes;
    let goal = (goal_min * 60) as isize;
    let focused_today = get_total_time(DurType::Today);
    if focused_today >= goal {
      Notification::new()
        .summary("Daily Goal completed")
        .body(&format!(
          "Congrulations in completing your daily focus goal of {goal_min} minutes"
        ))
        .show()
        .ok();
    }
  }
}
