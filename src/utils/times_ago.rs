use chrono::{DateTime, Local};
use std::convert::TryInto;

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

pub fn render_duration<T>(seconds: T) -> String
where
  T: TryInto<u64>,
  <T as TryInto<u64>>::Error: std::fmt::Debug,
{
  let s: u64 = seconds.try_into().unwrap();

  if s < 60 {
    format!("{} second{}", s, if s == 1 { "" } else { "s" })
  } else if s < 60 * 60 {
    let m = s / 60;
    let sec = s % 60;
    format!(
      "{} minute{}, {} second{}",
      m,
      if m == 1 { "" } else { "s" },
      sec,
      if sec == 1 { "" } else { "s" }
    )
  } else {
    let h = s / 3600;
    let m = (s % 3600) / 60;
    format!(
      "{} hour{}, {} minute{}",
      h,
      if h == 1 { "" } else { "s" },
      m,
      if m == 1 { "" } else { "s" }
    )
  }
}
