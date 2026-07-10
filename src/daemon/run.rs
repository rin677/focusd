use crate::{
  daemon::{
    SOCKET_PATH,
    commands::{Message, handle_stream, parse_message},
  },
  throw,
  timer::{engine::decrease_sec, state::TimerState},
  utils::ignore::Ignore,
};
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

pub fn daemon_active() -> bool {
  match UnixStream::connect(SOCKET_PATH) {
    Ok(mut stream) => {
      let cmd = parse_message(Message::Running);
      stream.write_all(format!("{cmd}\n").as_bytes()).ignore();
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

pub fn run_daemon() {
  println!("Starting Daemon");
  // Delete the socket path if already exists
  if Path::new(SOCKET_PATH).exists() {
    println!("Socket already available removing it");
    let _ = fs::remove_file(SOCKET_PATH);
  }

  let listener = UnixListener::bind(SOCKET_PATH).unwrap();
  let timer_state = Arc::new(Mutex::new(TimerState::default()));

  let s = Arc::clone(&timer_state);
  thread::spawn(move || {
    let tick_rate = Duration::from_secs(1);
    loop {
      thread::sleep(tick_rate);
      {
        let mut state = s.lock().unwrap();
        if state.running {
          println!("Clock is ticking");
          decrease_sec(&mut state);
        }
      }
    }
  });

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
