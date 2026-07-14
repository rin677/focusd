use crate::{
  daemon::{
    commands::{Message, handle_stream, parse_message, send_command},
    socket_path,
  },
  throw,
  timer::{
    engine::{decrease_sec, render_time},
    state::TimerState,
    utils::name_for_session,
  },
  utils::ignore::Ignore,
};
use std::{
  fs,
  io::{BufRead, BufReader, Result, Write},
  os::unix::net::{UnixListener, UnixStream},
  path::Path,
  process::{self, Command, Stdio},
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

pub fn daemon_active() -> bool {
  match UnixStream::connect(socket_path()) {
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

pub fn stop_daemon() {
  println!("Stopping the daemon");
  let sp = socket_path();
  if Path::new(sp).exists() {
    fs::remove_file(sp).ignore();
  }
  process::exit(0);
}

pub fn run_daemon() {
  println!("Starting Daemon");
  if daemon_active() {
    println!("Daemon already running stopping it");
    send_command(Message::StopDaemon).ignore();
  }
  // Delete the socket path if already exists
  let sp = socket_path();
  if Path::new(sp).exists() {
    println!("Socket already available removing it");
    fs::remove_file(sp).ignore();
  }

  let listener = UnixListener::bind(sp).unwrap();
  let timer_state = Arc::new(Mutex::new(TimerState::default()));

  let s = Arc::clone(&timer_state);
  thread::spawn(move || {
    let tick_rate = Duration::from_secs(1);
    loop {
      thread::sleep(tick_rate);
      {
        let mut state = s.lock().unwrap();
        if state.running {
          decrease_sec(&mut state);
          println!(
            "Clock is ticking, {} remain in {}",
            render_time(&state),
            name_for_session(state.session_type)
          );
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
          Err(e) => eprintln!("Error occurred in thread {e}"),
        });
      }
      Err(e) => eprintln!("Error occurred in stream {e}"),
    }
  }
}
