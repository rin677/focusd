use crate::{
  database::history::{HistoryEntry, get_db, get_full_history_no_err},
  utils::times_ago::render_duration,
};
use chrono::{Duration, Local, NaiveDate};

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

pub fn get_total_time(dur: DurType) -> isize {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0,
  };
  let r#where = get_sql_condition(dur, true);
  let sql = format!("SELECT SUM(completed_duration) FROM history {where}");
  db.query_row(&sql, [], |row| row.get(0)).unwrap_or(0)
}

pub fn get_completed_sessions() -> i32 {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0,
  };

  db.query_row("SELECT COUNT(*) FROM history WHERE session_type='Work' AND completed_duration==planned_duration", [], |row| row.get(0)).unwrap_or(0)
}

pub fn get_completion_rate() -> f32 {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0_f32,
  };

  db.query_row("SELECT 100.0 * (SELECT COUNT(*) FROM history WHERE session_type='Work' AND completed_duration==planned_duration) / (SELECT COUNT(*) FROM history WHERE session_type='Work')", [], |row| row.get(0)).unwrap_or(0_f32)
}

pub fn get_current_streak(all_history: Vec<HistoryEntry>) -> i32 {
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

pub fn get_daily_work_durations_n_days(n: usize) -> Vec<(String, u64)> {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return vec![],
  };

  let days_ago = n - 1;
  let sql = format!(
    "SELECT date(end_time) AS day, SUM(completed_duration)
     FROM history
     WHERE session_type='Work'
       AND date(end_time) >= date('now', '-{days_ago} days', 'localtime')
       AND date(end_time) <= date('now', 'localtime')
     GROUP BY date(end_time)
     ORDER BY day"
  );

  let mut stmt = match db.prepare(&sql) {
    Ok(stmt) => stmt,
    Err(_) => return vec![],
  };

  let mut rows: Vec<(String, i64)> = Vec::new();
  if let Ok(iter) = stmt.query_map([], |row| {
    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
  }) {
    for row in iter.flatten() {
      rows.push(row);
    }
  }

  let today = Local::now().date_naive();
  let mut result = Vec::new();
  for i in (0..n).rev() {
    let day = today - Duration::days(i as i64);
    let day_str = day.format("%Y-%m-%d").to_string();
    let value = rows
      .iter()
      .find(|(d, _)| *d == day_str)
      .map(|(_, v)| *v as u64)
      .unwrap_or(0);
    result.push((day_str, value));
  }

  result
}

pub fn get_daily_work_durations_7_days() -> Vec<(String, u64)> {
  get_daily_work_durations_n_days(7)
    .into_iter()
    .map(|(d, v)| {
      let date = NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap_or(Local::now().date_naive());
      (date.format("%a").to_string(), v)
    })
    .collect()
}

pub fn get_session_type_distribution() -> Vec<(String, f64)> {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return vec![],
  };

  let total: f64 = db
    .query_row("SELECT COUNT(*) FROM history", [], |row| row.get(0))
    .unwrap_or(0) as f64;

  if total == 0.0 {
    return vec![];
  }

  let mut stmt = match db.prepare("SELECT session_type, COUNT(*) FROM history GROUP BY session_type ORDER BY session_type") {
    Ok(stmt) => stmt,
    Err(_) => return vec![],
  };

  let mut result = Vec::new();
  if let Ok(iter) = stmt.query_map([], |row| {
    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
  }) {
    for row in iter.flatten() {
      let pct = (row.1 as f64 / total) * 100.0;
      let name = match row.0.as_str() {
        "Work" => "Work".to_string(),
        "ShortBreak" => "Short Break".to_string(),
        "LongBreak" => "Long Break".to_string(),
        s => s.to_string(),
      };
      result.push((name, pct));
    }
  }

  result
}

pub fn get_longest_session() {
  // TODO:: Implement this
}

pub fn print_stats() {
  println!(
    "Total focused {}",
    render_duration(get_total_time(DurType::All))
  );
  println!(
    "Total focused (this month) {}",
    render_duration(get_total_time(DurType::Month))
  );
  println!(
    "Total focused (this week) {}",
    render_duration(get_total_time(DurType::Week))
  );
  println!(
    "Total focused (today) {}",
    render_duration(get_total_time(DurType::Today))
  );
  println!("Completted sessions {}", get_completed_sessions());
  println!("Completion rate {}%", get_completion_rate());

  let history = get_full_history_no_err();
  println!("Current streek {}", get_current_streak(history));
}
