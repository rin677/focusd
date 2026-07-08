use crate::throw;
use rusqlite::Connection;
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

use crate::timer::state::TimerState;

#[derive(Debug)]
pub struct HistoryEntry {
  pub end_time: u64,
  pub planned_duration: u64,
  pub actual_duration: u64,
  pub completion_status: u64,
  pub session_type: u64,
}

fn history_db_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;
  Some(PathBuf::from(home).join(".local/share/focusd/db/history.db"))
}

fn create_db() -> io::Result<()> {
  let Some(path) = history_db_path() else {
    throw!("Path for database not found");
  };

  if let Some(parent) = path.parent() {
    if let Err(_e) = fs::create_dir_all(parent) {
      throw!("Failed to create parent directory");
    }
  }

  let cnn = match Connection::open(&path) {
    Ok(cnn) => cnn,
    Err(_) => throw!("Database could not be loaded"),
  };

  cnn.execute(
    "CREATE TABLE IF NOT EXISTS history(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          end_time TEXT NOT NULL,
          planned_duration TEXT NOT NULL,
          completion_status TEXT NOT NULL,
          session_type TEXT NOT NULL
  )
      ",
    [],
  );
  return Ok(());
}

pub fn add_session_to_db(_state: &TimerState) {
  // :TODO:
}
