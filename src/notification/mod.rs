use notify_rust::Notification;

use crate::{
  config::settings::get_config,
  timer::{
    state::TimerState,
    utils::{name_for_session, next_session as next_session_name, time_for_session},
  },
  utils::ignore::Ignore,
};

/// Show notification after sessioni is complete
pub fn show_complete_notification(state: &TimerState) {
  let config = get_config();
  let session_name = name_for_session(state.session_type);
  if state.time_remaining.is_zero() && config.show_notifications {
    Notification::new()
      .summary(&format!("Timer for {} finished", session_name))
      .body(&format!(
        "Congrulations in completing timer for {} minutes, Now it's time for {}",
        time_for_session(state.session_type).as_secs() / 60,
        name_for_session(next_session_name(state.session_type, state.session_number))
      ))
      .show()
      .ignore();
  }
}
