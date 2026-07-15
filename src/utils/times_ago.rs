use chrono::{DateTime, Local};
use std::convert::TryInto;

/// Returns string saying `x days/hours ago` given the date time object
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

/// Renders duration in format `2h 5m` given number of seconds
pub fn render_duration<T>(seconds: T) -> String
where
  T: TryInto<u64>,
  <T as TryInto<u64>>::Error: std::fmt::Debug,
{
  let s: u64 = seconds.try_into().unwrap();

  if s < 60 {
    format!("{s}s")
  } else if s < 60 * 60 {
    let m = s / 60;
    let sec = s % 60;
    if sec == 0 {
      return format!("{m}m");
    }
    format!("{}m, {}s", m, sec)
  } else {
    let h = s / 3600;
    let m = (s % 3600) / 60;
    if m == 0 {
      return format!("{h} h");
    }
    format!("{}h, {}m", h, m,)
  }
}
