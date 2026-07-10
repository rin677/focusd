use crate::{
  daemon::{SOCKET_PATH, run::stop_daemon},
  timer::{
    engine::{next_session, render_time, reset_session, start_session},
    state::{TimerSnapShot, TimerState},
    utils::name_for_session,
  },
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
  ResetSession,

  // Test
  Running,

  //Other
  StopDaemon,
}

pub fn parse_message(message: Message) -> String {
  match message {
    Message::StartSession => String::from("StartSession"),
    Message::PauseSession => String::from("PauseSession"),
    Message::ResumeSession => String::from("ResumeSession"),
    Message::ToggleSession => String::from("ToggleSession"),
    Message::NextSession => String::from("NextSession"),
    Message::GetSession => String::from("GetSession"),
    Message::ResetSession => String::from("ResetSession"),
    Message::StopDaemon => String::from("StopDaemon"),
    Message::Running => String::from("RUNNING"),
  }
}

pub fn handle_stream(mut stream: UnixStream, state: Arc<Mutex<TimerState>>) -> Result<()> {
  let mut reader = BufReader::new(stream.try_clone()?);
  let mut line = String::new();
  reader.read_line(&mut line)?;

  let mut s = state.lock().unwrap();
  let l = line.trim().to_string();
  println!("Got message {l}");
  let response = if l == parse_message(Message::Running) {
    "YES".to_string()
  } else if l == parse_message(Message::GetSession) {
    let snapshot = TimerSnapShot::from(&*s);
    serde_json::to_string(&snapshot).unwrap()
  } else if l == parse_message(Message::StartSession) {
    start_session(&mut s);
    format!("{} Session started", name_for_session(s.session_type))
  } else if l == parse_message(Message::PauseSession)
    || l == parse_message(Message::ResumeSession)
    || l == parse_message(Message::ToggleSession)
  {
    let new_running = if l == parse_message(Message::ResumeSession) {
      true
    } else if l == parse_message(Message::ToggleSession) {
      !s.running
    } else {
      false
    };
    s.running = new_running;

    let status = if new_running { "resumed" } else { "paused" };
    format!(
      "{} {} time remaining: {}",
      name_for_session(s.session_type),
      status,
      render_time(&s)
    )
  } else if l == parse_message(Message::NextSession) {
    next_session(&mut s);
    format!("{} Session started", name_for_session(s.session_type))
  } else if l == parse_message(Message::StopDaemon) {
    stop_daemon();
    "OK".to_string()
  } else if l == parse_message(Message::ResetSession) {
    reset_session(&mut s);
    format!("{} Session restarted", name_for_session(s.session_type))
  } else {
    "Command not found".to_string()
  };
  stream.write_all(format!("{}\n", response).as_bytes())?;
  Ok(())
}

pub fn send_command(message: Message) -> Result<String> {
  let cmd = parse_message(message);
  let mut stream = UnixStream::connect(SOCKET_PATH)?;
  stream.write_all(format!("{}\n", cmd).as_bytes())?;

  let mut reader = BufReader::new(stream);
  let mut line = String::new();
  reader.read_line(&mut line)?;
  Ok(line)
}

pub fn get_timer_state() -> Result<TimerState> {
  let m = send_command(Message::GetSession)?;
  let state: TimerSnapShot = serde_json::from_str(&m)?;
  Ok(TimerState::from(&state))
}
