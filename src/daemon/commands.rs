use crate::{
  daemon::SOCKET_PATH,
  timer::state::{TimerSnapShot, TimerState},
};
use std::{
  io::{BufRead, BufReader, Result, Write},
  os::unix::net::UnixStream,
  sync::{Arc, Mutex},
};

pub enum Message {
  // Sessions
  StartSession,
  PauseSession,
  ResumeSession,
  ToggleSession,
  NextSession,
  GetSession,

  // Test
  Running,
}

pub fn parse_message(message: Message) -> String {
  match message {
    Message::StartSession => String::from("StartSession"),
    Message::PauseSession => String::from("PauseSession"),
    Message::ResumeSession => String::from("ResumeSession"),
    Message::ToggleSession => String::from("ToggleSession"),
    Message::NextSession => String::from("NextSession"),
    Message::GetSession => String::from("GetSession"),
    Message::Running => String::from("RUNNING"),
  }
}

pub fn handle_stream(mut stream: UnixStream, state: Arc<Mutex<TimerState>>) -> Result<()> {
  let mut reader = BufReader::new(stream.try_clone()?);
  let mut line = String::new();
  reader.read_line(&mut line)?;

  let s = state.lock().unwrap();
  let response = if line.trim() == parse_message(Message::GetSession) {
    "YES".to_string()
  } else if line.trim() == parse_message(Message::Running) {
    let snapshot = TimerSnapShot::from(&*s);
    serde_json::to_string(&snapshot).unwrap()
  } else {
    "Command not found".to_string()
  };
  stream.write_all(format!("{}\n", response).as_bytes())?;
  Ok(())
}

// TODO: Send command to the daemon
pub fn send_command(message: Message) -> Result<String> {
  let cmd = parse_message(message);
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
