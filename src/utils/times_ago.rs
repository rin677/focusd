use chrono::{DateTime, Local};

pub fn times_ago(time: &DateTime<Local>) -> String {
  let now = Local::now();
  let diff = now - time;
  let diff_sec = diff.num_seconds();

  if diff.is_zero() || diff_sec < 60 {
    return "Just Now".to_string();
  } else if diff_sec < (60 * 60) {
    return format!("{} minutes ago", diff.num_minutes());
  } else if diff_sec < (60 * 60 * 24) {
    return format!("{} hours ago", diff.num_hours());
  }
  format!("{} days ago", diff.num_days())
}
