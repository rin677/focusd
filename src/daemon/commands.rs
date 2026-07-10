use crate::{
  daemon::SOCKET_PATH,
  timer::state::{TimerSnapShot, TimerState},
};
use std::{
  io::{BufRead, BufReader, Result, Write},
  os::unix::net::UnixStream,
  sync::{Arc, Mutex},
};

pub fn handle_stream(mut stream: UnixStream, state: Arc<Mutex<TimerState>>) -> Result<()> {
  let mut reader = BufReader::new(stream.try_clone()?);
  let mut line = String::new();
  reader.read_line(&mut line)?;

  let s = state.lock().unwrap();
  let response = {
    match line.trim() {
      "RUNNING" => "YES".to_string(),
      "GET_STATUS" => {
        let snapshot = TimerSnapShot::from(&*s);
        serde_json::to_string(&snapshot).unwrap()
      }
      _ => "Command not found".to_string(),
    }
  };
  stream.write_all(format!("{}\n", response).as_bytes())?;
  Ok(())
}

// TODO: Send command to the daemon
pub fn send_command(cmd: &str) -> Result<String> {
  let mut stream = UnixStream::connect(SOCKET_PATH)?;
  stream.write_all(format!("{}\n", cmd).as_bytes())?;

  let mut reader = BufReader::new(stream);
  let mut line = String::new();
  reader.read_line(&mut line)?;
  Ok(line)
}

pub fn get_timer_status() -> Result<()> {
  // TODO: Get timer status from the daemon
  Ok(())
}
