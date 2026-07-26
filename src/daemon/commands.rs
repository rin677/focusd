use crate::{
  daemon::{run::stop_daemon, socket_path},
  timer::{
    engine::{
      next_session, render_time, reset_session, select_preset, start_session,
      start_specific_session,
    },
    state::{TimerSnapShot, TimerState},
    utils::{get_session_type, name_for_session},
  },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

pub enum PayloadMessage {
  SelectSession,
  SelectPreset,
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

pub fn send_message_with_payload(message: PayloadMessage, payload: Value) -> Result<String> {
  let message = parse_payload_messages(message);
  let p = PayloadType { message, payload };
  let json = serde_json::to_string(&p)?;
  let command = format!("MESSAGE_PAYLOAD: {json}");
  send_command(command)
}

pub fn parse_payload_messages(message: PayloadMessage) -> String {
  match message {
    PayloadMessage::SelectPreset => String::from("SelectPreset"),
    PayloadMessage::SelectSession => String::from("SelectSession"),
  }
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

  let mut s = state
    .lock()
    .unwrap_or_else(|poisoned| poisoned.into_inner());
  let l = line.trim().to_string();
  println!("Got message {l}");

  let mut response = "".to_string();
  if !contains_payload(&l) {
    response = if l == parse_message(Message::Running) {
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
  } else {
    let p = seperate_message_and_payload(l)?;
    if p.message == parse_payload_messages(PayloadMessage::SelectSession) {
      let value = p.payload.as_str().unwrap();
      let session_type = get_session_type(value);
      start_specific_session(&mut s, session_type);
      response = format!("{} Session started", name_for_session(session_type));
    } else if p.message == parse_payload_messages(PayloadMessage::SelectPreset) {
      let value = p.payload.as_str().unwrap();
      match select_preset(&mut s, value) {
        Ok(_) => response = format!("Preset {} selected", value),
        Err(e) => response = e.to_string(),
      }
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

pub fn send_message(message: Message) -> Result<String> {
  let cmd = parse_message(message);
  send_command(cmd)
}

pub fn get_timer_state() -> Result<TimerState> {
  let m = send_message(Message::GetSession)?;
  let state: TimerSnapShot = serde_json::from_str(&m)?;
  Ok(TimerState::from(&state))
}
