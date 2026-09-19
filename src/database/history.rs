use crate::{
  throw,
  timer::{
    engine::get_elapsed_time,
    state::{SessionType, TimerState},
  },
  utils::{
    profile::Profile,
    timer::{render_duration, times_ago},
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
  pub is_completed: bool,
  pub category: String,
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

  let user_version: i32 = cnn
    .query_row("PRAGMA user_version", [], |row| row.get(0))
    .unwrap_or(0);

  if user_version < 1 {
    // TODO: Session tags
    let s = cnn.execute(
      "CREATE TABLE IF NOT EXISTS history(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            end_time TEXT NOT NULL,
            planned_duration INTEGER NOT NULL,
            completed_duration INTEGER NOT NULL,
            session_type TEXT NOT NULL,
            is_completed BOOLEAN NOT NULL DEFAULT 0,
            category TEXT NOT NULL DEFAULT 'General'
    )
        ",
      [],
    );
    if let Err(e) = s {
      println!("Got error while creating database {e}");
    }

    let has_is_completed = cnn
      .prepare("SELECT is_completed FROM history LIMIT 1")
      .is_ok();

    if !has_is_completed {
      if let Err(e) = cnn.execute(
        "ALTER TABLE history ADD COLUMN is_completed BOOLEAN NOT NULL DEFAULT 0",
        [],
      ) {
        println!("Got error while altering database {e}");
      }
      let _ = cnn.execute(
        "UPDATE history SET is_completed = 1 WHERE completed_duration >= planned_duration",
        [],
      );
    }

    let _ = cnn.execute("PRAGMA user_version = 1", []);
  }

  if user_version < 2 {
    let has_category = cnn.prepare("SELECT category FROM history LIMIT 1").is_ok();
    if !has_category {
      cnn
        .execute(
          "ALTER TABLE history ADD COLUMN category TEXT NOT NULL DEFAULT 'General'",
          [],
        )
        .map_err(io::Error::other)?;
    }
    cnn
      .execute("PRAGMA user_version = 2", [])
      .map_err(io::Error::other)?;
  }

  Ok(cnn)
}

/// Add the given session to history database
pub fn add_session_to_db(state: &TimerState) -> Result<(), Box<dyn std::error::Error>> {
  let completed_duration = get_elapsed_time(state).as_secs() as i64;
  if completed_duration < 20 {
    println!("Not adding session shorter than 20 seconds to database");
    return Ok(());
  };
  let cnn = get_db()?;
  let now = Local::now();
  let end_time = now.format(TIME_PATTERN).to_string();
  let planned_duration = state.session_type.get_time().as_secs() as i64;
  let session_type = state.session_type.name();
  let is_completed = state.time_remaining.is_zero() || completed_duration >= planned_duration;
  cnn.execute(
    "INSERT INTO history 
    (end_time, planned_duration, completed_duration, session_type, is_completed, category) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    (end_time, planned_duration, completed_duration, session_type, is_completed, &state.category),
  )?;
  println!("Session added to database");
  Ok(())
}

/// Gives full history from the database
pub fn get_full_history() -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
  let db = get_db()?;
  let mut stmt = db
    .prepare("SELECT end_time, planned_duration, completed_duration, session_type, is_completed, category FROM history ORDER BY datetime(end_time) DESC")
    ?;

  let mut history: Vec<HistoryEntry> = Vec::new();
  let mut rows = stmt.query([])?;
  while let Some(row) = rows.next()? {
    let s: String = row.get(3)?;
    let e: String = row.get(0)?;
    let time = NaiveDateTime::parse_from_str(&e, TIME_PATTERN)
      .unwrap()
      .and_local_timezone(Local)
      .single()
      .unwrap();
    let is_completed: bool = row.get(4)?;
    history.push(HistoryEntry {
      end_time: time,
      planned_duration: row.get(1)?,
      completed_duration: row.get(2)?,
      session_type: SessionType::from_string(&s),
      is_completed,
      category: row.get(5)?,
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
    let status = if history.is_completed {
      "completed"
    } else {
      "incomplete"
    };
    println!(
      "{} [{}] ({}) - target: {}, completed: {}, {}.",
      history.session_type.name(),
      history.category,
      status,
      render_duration(history.planned_duration),
      render_duration(history.completed_duration),
      times_ago(&history.end_time)
    );
  }
}
