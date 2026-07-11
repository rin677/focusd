// :TODO:
// - [x] Calculate focus time today
// - [x] Calculate focus time this week
// - [x] Calculate focus time this month
// - [x] Calculate total focus time
// - [x] Calculate completed sessions
// - [x] Calculate completion rate
// - [x] Calculate current streak
// - [ ] Calculate longest streak

use crate::database::history::{HistoryEntry, get_db, get_full_history_no_err};
use chrono::{Duration, Local};

#[derive(PartialEq)]
pub enum DurType {
  Today,
  Week,
  Month,
  All,
}

fn get_sql_condition(dur: DurType, work: bool) -> String {
  if work && dur == DurType::All {
    return "WHERE session_type='Work'".to_string();
  }
  let conditional = match dur {
    DurType::Today => "WHERE date(end_time) = date('now', 'localtime')",
    DurType::Week => {
      "WHERE date(end_time) >= date('now', 'localtime', 'weekday 1', '-7 days')
       AND date(end_time) <  date('now', 'localtime', 'weekday 1')"
    }
    DurType::Month => "WHERE strftime('%Y-%m', end_time) = strftime('%Y-%m', 'now', 'localtime')",
    DurType::All => "",
  };

  if work {
    format!("{conditional} AND session_type='Work'")
  } else {
    conditional.to_string()
  }
}

fn get_total_time(dur: DurType) -> isize {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0,
  };
  let r#where = get_sql_condition(dur, true);
  let sql = format!("SELECT SUM(completed_duration) FROM history {where}");
  db.query_row(&sql, [], |row| row.get(0)).unwrap_or(0) / 60
}

fn get_completed_sessions() -> i32 {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0,
  };

  db.query_row("SELECT COUNT(*) FROM history WHERE session_type='Work' AND completed_duration==planned_duration", [], |row| row.get(0)).unwrap_or(0)
}

fn get_completion_rate() -> f32 {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0_f32,
  };

  db.query_row("SELECT 100.0 * (SELECT COUNT(*) FROM history WHERE session_type='Work' AND completed_duration==planned_duration) / (SELECT COUNT(*) FROM history WHERE session_type='Work')", [], |row| row.get(0)).unwrap_or(0_f32)
}

fn get_current_streak(all_history: Vec<HistoryEntry>) -> i32 {
  let mut streak = 0;
  let mut expected_day = Local::now().date_naive();

  // All history should sorted from newest to oldest
  for history in all_history {
    let day = history.end_time.date_naive();

    if day == expected_day {
      streak += 1;
      expected_day -= Duration::days(1);
    } else if day < expected_day {
      break;
    }
  }

  streak
}

pub fn print_stats() {
  println!("Total focused {} mins", get_total_time(DurType::All));
  println!(
    "Total focused (this month) {} mins",
    get_total_time(DurType::Month)
  );
  println!(
    "Total focused (this week) {} mins",
    get_total_time(DurType::Week)
  );
  println!(
    "Total focused (today) {} mins",
    get_total_time(DurType::Today)
  );
  println!("Completted sessions {}", get_completed_sessions());
  println!("Completion rate {}%", get_completion_rate());

  let history = get_full_history_no_err();
  println!("Current streek {}", get_current_streak(history));
}
