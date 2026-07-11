use crate::{
  throw,
  timer::{
    state::{SessionType, TimerState},
    utils::{get_session_type, name_for_session, time_for_session},
  },
  utils::ignore::Ignore,
};
use chrono::{DateTime, Local};
use rusqlite::Connection;
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

pub struct HistoryEntry {
  pub end_time: DateTime<Local>,
  pub planned_duration: i64,
  pub completed_duration: i64,
  pub session_type: SessionType,
}

fn history_db_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;

  #[cfg(feature = "dev-build")]
  let path = ".local/share/focusd/db/history-dev.db";

  #[cfg(not(feature = "dev-build"))]
  let path = ".local/share/focusd/db/history.db";

  Some(PathBuf::from(home).join(path))
}

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

pub fn add_session_to_db(state: &TimerState) -> Result<(), Box<dyn std::error::Error>> {
  let completed_duration =
    (time_for_session(state.session_type) - state.time_remaining).as_secs() as i64;
  if completed_duration < 20 {
    println!("Not adding session shorter than 20 seconds to database");
    return Ok(());
  };
  let cnn = get_db()?;
  let now = Local::now();
  let end_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
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

pub fn get_full_history() -> io::Result<Vec<HistoryEntry>> {
  let db = get_db()?;
  let mut stmt = db
    .prepare("SELECT end_time, planned_duration, completed_duration, session_type FROM history")
    .ignore();

  let history_itr = stmt
    .query_map([], |row| {
      let s: String = row.get(3)?;
      Ok(HistoryEntry {
        end_time: row.get(0)?,
        planned_duration: row.get(1)?,
        completed_duration: row.get(2)?,
        session_type: get_session_type(&s),
      })
    })
    .ignore();
  let mut history: Vec<HistoryEntry> = Vec::new();
  for history_item in history_itr {
    let h = history_item.ignore();
    history.push(h);
  }

  Ok(history)
}

pub fn get_full_history_no_err() -> Vec<HistoryEntry> {
  match get_full_history() {
    Ok(h) => h,
    Err(_) => Vec::new() as Vec<HistoryEntry>,
  }
}

pub fn print_history() {
  let all_history = get_full_history_no_err();
  for history in all_history {
    println!(
      "{}, target: {} mins, completed: {} mins",
      name_for_session(history.session_type),
      history.planned_duration / 60,
      history.completed_duration / 60
    );
  }
}
