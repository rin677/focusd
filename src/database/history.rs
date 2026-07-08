use crate::{
  throw,
  timer::utils::{name_for_session, time_for_session},
};
use chrono::Local;
use rusqlite::{Connection, ToSql};
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

  let s = cnn.execute(
    "CREATE TABLE IF NOT EXISTS history(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          end_time TEXT NOT NULL,
          planned_duration INTEGER NOT NULL,
          actual_duration INTEGER NOT NULL,
          completion_status TEXT NOT NULL,
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

pub fn add_session_to_db(state: &TimerState) {
  match get_db() {
    Ok(cnn) => {
      let actual_duration =
        (time_for_session(state.sessioin_type) - state.time_remaining).as_secs() as i64;
      if actual_duration == 0 {
        return;
      };
      let now = Local::now();
      if let Ok(end_time) = now.to_sql() {
        let planned_duration = time_for_session(state.sessioin_type).as_secs() as i64;
        let sessioin_type = name_for_session(state.sessioin_type);
        let completion_status = if state.time_remaining.is_zero() {
          "Completed".to_string()
        } else {
          "Incomplete".to_string()
        };

        let s = cnn.execute(
            "INSERT INTO history (end_time, planned_duration, actual_duration, completion_status, session_type) VALUES (?1, ?2, ?3, ?4, ?5)",
            (end_time, planned_duration, actual_duration ,completion_status ,sessioin_type ),
        );
        match s {
          Ok(_) => {}
          Err(e) => {
            println!("Got error while adding history to database {e}")
          }
        }
      }
    }
    Err(e) => println!("{e}"),
  }
}
