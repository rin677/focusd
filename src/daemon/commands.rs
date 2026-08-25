use crate::{
  daemon::{run::stop_daemon, socket_path},
  timer::{
    engine::{
      add_time, next_session, render_time, reset_session, select_preset, start_session,
      start_specific_session,
    },
    hooks::{pause_hooks, resume_hooks, session_start_hook},
    state::{SessionType, TimerSnapShot, TimerState},
  },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
  io::{BufRead, BufReader, Result, Write},
  os::unix::net::UnixStream,
  sync::{Arc, Mutex},
  time::Duration,
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

pub enum PayloadMessage {
  SelectSession,
  SelectPreset,
  AddMinutes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadType {
  message: String,
  payload: Value,
}

pub fn seperate_message_and_payload(message: String) -> Result<PayloadType> {
  let p = message.strip_prefix("MESSAGE_PAYLOAD: ").unwrap();
  let payload: PayloadType = serde_json::from_str(p)?;
  Ok(payload)
}

pub fn contains_payload(message: &str) -> bool {
  message.trim().starts_with("MESSAGE_PAYLOAD:")
}

impl PayloadMessage {
  pub fn send(self, payload: Value) -> Result<String> {
    let message = self.to_string();
    let p = PayloadType { message, payload };
    let json = serde_json::to_string(&p)?;
    let command = format!("MESSAGE_PAYLOAD: {json}");
    send_command(command)
  }
}

impl Message {
  pub fn send(self) -> Result<String> {
    let cmd = self.to_string();
    send_command(cmd)
  }
}

impl std::fmt::Display for PayloadMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::SelectPreset => write!(f, "SelectPreset"),
      Self::SelectSession => write!(f, "SelectSession"),
      Self::AddMinutes => write!(f, "AddMinutes"),
    }
  }
}

impl std::fmt::Display for Message {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::StartSession => write!(f, "StartSession"),
      Self::PauseSession => write!(f, "PauseSession"),
      Self::ResumeSession => write!(f, "ResumeSession"),
      Self::ToggleSession => write!(f, "ToggleSession"),
      Self::NextSession => write!(f, "NextSession"),
      Self::GetSession => write!(f, "GetSession"),
      Self::ResetSession => write!(f, "ResetSession"),
      Self::StopDaemon => write!(f, "StopDaemon"),
      Self::Running => write!(f, "RUNNING"),
    }
  }
}

pub fn handle_stream(mut stream: UnixStream, state: Arc<Mutex<TimerState>>) -> Result<()> {
  let mut reader = BufReader::new(stream.try_clone()?);
  let mut line = String::new();
  reader.read_line(&mut line)?;

  let mut s = state
    .lock()
    .unwrap_or_else(|poisoned| poisoned.into_inner());
  let l = line.trim().to_string();
  println!("Got message {l}");

  let mut response = "".to_string();
  if !contains_payload(&l) {
    response = if l == Message::Running.to_string() {
      "YES".to_string()
    } else if l == Message::GetSession.to_string() {
      let snapshot = TimerSnapShot::from(&*s);
      serde_json::to_string(&snapshot).unwrap()
    } else if l == Message::StartSession.to_string() {
      start_session(&mut s);
      format!("{} Session started", s.session_type.name())
    } else if l == Message::PauseSession.to_string()
      || l == Message::ResumeSession.to_string()
      || l == Message::ToggleSession.to_string()
    {
      let new_running = if l == Message::ResumeSession.to_string() {
        true
      } else if l == Message::ToggleSession.to_string() {
        !s.running
      } else {
        false
      };
      s.running = new_running;

      let mut status = "resumed";
      if new_running {
        if s.time_remaining == s.session_type.get_time() {
          session_start_hook(s.session_type);
        } else {
          resume_hooks(s.session_type);
        }
      } else {
        pause_hooks(s.session_type);
        status = "paused";
      }
      format!(
        "{} {} time remaining: {}",
        s.session_type.name(),
        status,
        render_time(&s)
      )
    } else if l == Message::NextSession.to_string() {
      next_session(&mut s);
      format!("{} Session started", s.session_type.name())
    } else if l == Message::StopDaemon.to_string() {
      stop_daemon();
      "OK".to_string()
    } else if l == Message::ResetSession.to_string() {
      reset_session(&mut s);
      format!("{} Session restarted", s.session_type.name())
    } else {
      "Command not found".to_string()
    };
  } else {
    let p = seperate_message_and_payload(l)?;
    if p.message == PayloadMessage::SelectSession.to_string() {
      let value = p.payload.as_str().unwrap();
      let session_type = SessionType::from_string(value);
      start_specific_session(&mut s, session_type);
      response = format!("{} Session started", session_type.name());
    } else if p.message == PayloadMessage::SelectPreset.to_string() {
      let value = p.payload.as_str().unwrap();
      match select_preset(&mut s, value) {
        Ok(_) => response = format!("Preset {} selected", value),
        Err(e) => response = e.to_string(),
      }
    } else if p.message == PayloadMessage::AddMinutes.to_string() {
      let value = p.payload.as_u64().unwrap();
      add_time(&mut s, Duration::from_mins(value));
    }
  }

  stream.write_all(format!("{}\n", response).as_bytes())?;
  Ok(())
}

pub fn send_command(command: String) -> Result<String> {
  let mut stream = UnixStream::connect(socket_path())?;
  stream.write_all(format!("{}\n", command).as_bytes())?;
  let mut reader = BufReader::new(stream);
  let mut line = String::new();
  reader.read_line(&mut line)?;
  Ok(line)
}

pub fn get_timer_state() -> Result<TimerState> {
  let m = Message::GetSession.send()?;
  let state: TimerSnapShot = serde_json::from_str(&m)?;
  Ok(TimerState::from(&state))
}
