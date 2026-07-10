use crate::{
  daemon::commands::get_timer_state,
  throw,
  timer::{
    state::TimerState,
    utils::{name_for_session, time_for_session},
  },
};
use chrono::Local;
use rusqlite::Connection;
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

#[derive(Debug)]
pub struct HistoryEntry {
  pub end_time: u64,
  pub planned_duration: u64,
  pub completed_duration: u64,
  pub session_type: u64,
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
    return Ok(());
  };
  let cnn = get_db()?;
  let now = Local::now();
  let end_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
  let planned_duration = time_for_session(state.session_type).as_secs() as i64;
  let sessioin_type = name_for_session(state.session_type);
  cnn.execute(
    "INSERT INTO history 
    (end_time, planned_duration, completed_duration, session_type) VALUES (?1, ?2, ?3, ?4)",
    (
      end_time,
      planned_duration,
      completed_duration,
      sessioin_type,
    ),
  )?;
  Ok(())
}
