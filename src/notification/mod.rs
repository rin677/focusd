use notify_rust::Notification;

use crate::{
  config::settings::get_config,
  stats::calculate::{DurType, get_total_time},
  timer::state::{SessionType, TimerState},
};

/// Show notification after sessioni is complete
pub fn show_complete_notification(state: &TimerState) {
  let config = get_config();
  let session_name = state.session_type.name();
  if config.show_notifications {
    if state.time_remaining.is_zero() {
      // Timer completed notification
      Notification::new()
        .summary(&format!("Timer for {} finished", session_name))
        .body(&format!(
          "Congrulations in completing timer for {} minutes, Now it's time for {}",
          state.session_type.get_time().as_secs() / 60,
          state.session_type.next(state.session_number).name()
        ))
        .show()
        .ok();
    }

    // Goal completed notification
    let goal_min = get_config().daily_goal_minutes;
    let goal = (goal_min * 60) as isize;
    let focused_today = get_total_time(DurType::Today);

    if focused_today >= goal && state.session_type == SessionType::Work {
      // check if goal was completed in this session
      let focused_this_session = state.session_type.get_time() - state.time_remaining;
      let completed_in_this_session =
        (focused_today - focused_this_session.as_secs() as isize) < goal;
      if !completed_in_this_session {
        return;
      }

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
