use crate::{
  throw,
  timer::{
    state::{SessionType, TimerState},
    utils::{get_session_type, name_for_session, time_for_session},
  },
  utils::{
    ignore::Ignore,
    profile::Profile,
    times_ago::{render_duration, times_ago},
  },
};
use chrono::{DateTime, Local, NaiveDateTime};
use rusqlite::Connection;
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

const TIME_PATTERN: &str = "%Y-%m-%d %H:%M:%S";

/// An item in database for history
pub struct HistoryEntry {
  pub end_time: DateTime<Local>,
  pub planned_duration: i64,
  pub completed_duration: i64,
  pub session_type: SessionType,
}

fn history_db_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;

  let profile = Profile::current();
  let path = profile.db_path();
  Some(PathBuf::from(home).join(path))
}

/// Returns path to database
///
/// Gives differnt path based on dev/prod profile
pub fn get_db() -> io::Result<Connection> {
  let Some(path) = history_db_path() else {
    throw!("Path for database not found");
  };

  if let Some(parent) = path.parent()
    && let Err(_e) = fs::create_dir_all(parent)
  {
    throw!("Failed to create parent directory");
  }

  let cnn = match Connection::open(&path) {
    Ok(cnn) => cnn,
    Err(_) => throw!("Database could not be loaded"),
  };

  // TODO: Session tags
  let s = cnn.execute(
    "CREATE TABLE IF NOT EXISTS history(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          end_time TEXT NOT NULL,
          planned_duration INTEGER NOT NULL,
          completed_duration INTEGER NOT NULL,
          session_type TEXT NOT NULL
  )
      ",
    [],
  );
  match s {
    Ok(_) => {}
    Err(e) => {
      println!("Got error while creating database {e}")
    }
  }
  Ok(cnn)
}

/// Add the given session to history database
pub fn add_session_to_db(state: &TimerState) -> Result<(), Box<dyn std::error::Error>> {
  let completed_duration =
    (time_for_session(state.session_type) - state.time_remaining).as_secs() as i64;
  if completed_duration < 20 {
    println!("Not adding session shorter than 20 seconds to database");
    return Ok(());
  };
  let cnn = get_db()?;
  let now = Local::now();
  let end_time = now.format(TIME_PATTERN).to_string();
  let planned_duration = time_for_session(state.session_type).as_secs() as i64;
  let session_type = name_for_session(state.session_type);
  cnn.execute(
    "INSERT INTO history 
    (end_time, planned_duration, completed_duration, session_type) VALUES (?1, ?2, ?3, ?4)",
    (end_time, planned_duration, completed_duration, session_type),
  )?;
  println!("Session added to database");
  Ok(())
}

/// Gives full history from the database
pub fn get_full_history() -> io::Result<Vec<HistoryEntry>> {
  let db = get_db()?;
  let mut stmt = db
    .prepare("SELECT end_time, planned_duration, completed_duration, session_type FROM history ORDER BY datetime(end_time) DESC")
    .ignore();

  let mut history: Vec<HistoryEntry> = Vec::new();
  let mut rows = stmt.query([]).ignore();
  while let Some(row) = rows.next().ignore() {
    let s: String = row.get(3).ignore();
    let e: String = row.get(0).ignore();
    let time = NaiveDateTime::parse_from_str(&e, TIME_PATTERN)
      .unwrap()
      .and_local_timezone(Local)
      .single()
      .unwrap();
    history.push(HistoryEntry {
      end_time: time,
      planned_duration: row.get(1).ignore(),
      completed_duration: row.get(2).ignore(),
      session_type: get_session_type(&s),
    });
  }

  Ok(history)
}

/// Returns full history
///
/// Returns empty vector in case of any errors
pub fn get_full_history_no_err() -> Vec<HistoryEntry> {
  match get_full_history() {
    Ok(h) => h,
    Err(_) => Vec::new() as Vec<HistoryEntry>,
  }
}

/// Prints history in friendly and readable format
pub fn print_history() {
  let all_history = get_full_history_no_err();
  for history in all_history {
    println!(
      "{} - target: {}, completed: {}, {}.",
      name_for_session(history.session_type),
      render_duration(history.planned_duration),
      render_duration(history.completed_duration),
      times_ago(&history.end_time)
    );
  }
}
