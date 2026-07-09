use crate::{throw, timer::state::TimerState, utils::ignore::Ignore};
use std::{
  fs,
  io::{BufRead, BufReader, Result, Write},
  os::unix::net::{UnixListener, UnixStream},
  path::Path,
  process::{Command, Stdio},
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

const SOCKET_PATH: &str = "/tmp/focusd.sock";

pub fn daemon_active() -> bool {
  match UnixStream::connect(SOCKET_PATH) {
    Ok(mut stream) => {
      stream.write_all(b"RUNNING\n").ignore();
      let mut reader = BufReader::new(stream);
      let mut line = String::new();
      reader.read_line(&mut line).is_ok() && line.starts_with("YES")
    }
    Err(_) => false,
  }
}

pub fn ensure_daemon_active(first_try: bool) -> Result<()> {
  if daemon_active() {
    return Ok(());
  }

  Command::new(std::env::current_exe()?)
    .arg("--daemon")
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?;

  if first_try {
    for _ in 1..20 {
      if daemon_active() {
        return Ok(());
      } else {
        thread::sleep(Duration::from_millis(50));
        ensure_daemon_active(false).ignore();
      }
    }
  }
  throw!("Daemon not active")
}

fn handle_stream(mut stream: UnixStream, state: Arc<Mutex<TimerState>>) -> Result<()> {
  let mut reader = BufReader::new(stream.try_clone()?);
  let mut line = String::new();
  reader.read_line(&mut line)?;

  let response = {
    match line.trim() {
      "hello" => "hi",
      "what" => "what",
      "RUNNING" => "YES",
      _ => "Sorry i don't know what are you saying",
    }
  };
  stream.write_all(format!("{}\n", response).as_bytes())?;
  Ok(())
}

pub fn run_daemon() {
  // Delete the socket path if already exists
  if Path::new(SOCKET_PATH).exists() {
    let _ = fs::remove_file(SOCKET_PATH);
  }

  let listener = UnixListener::bind(SOCKET_PATH).unwrap();
  let timer_state = Arc::new(Mutex::new(TimerState::default()));

  // Execute every command in different thread so that multiple operations could be done
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let state = Arc::clone(&timer_state);
        thread::spawn(move || match handle_stream(stream, state) {
          Ok(_) => {}
          Err(e) => eprintln!("Error occurred {e}"),
        });
      }
      Err(e) => eprintln!("Error occurred {e}"),
    }
  }
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
