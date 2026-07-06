mod timer;

use std::time::Duration;
use std::{io, sync::mpsc, thread};
use timer::engine::{render_time, skip_sesion, toggle_session};
use timer::state::{SessionType, TimerState};

fn get_input(v: &mut String) {
  io::stdin().read_line(v).expect("Failed to read input");
  *v = v.trim().to_string()
}

fn clear() {
  print!("\x1B[2J\x1B[1;1H");
}

fn main() {
  let mut timer_state = TimerState {
    running: false,
    time_remaining: Duration::from_mins(25),
    sessioin_type: SessionType::Work,
  };
  let (tx, rx) = mpsc::channel::<String>();

  thread::spawn(move || {
    loop {
      clear();
      render_time(&timer_state);
      println!();
      println!("What do you want to do?");
      let toggle_action = if timer_state.running {
        "Pause"
      } else {
        "Continue"
      };
      println!("1. {toggle_action} Sessioin");
      println!("2. Next Sessioin");
      println!("3. Quit App");
      println!();
      let one_sec = Duration::from_secs(1);
      thread::sleep(one_sec);
      if timer_state.running {
        let new_time = timer_state.time_remaining - one_sec;
        if new_time.is_zero() {
          timer_state.running = false
        }
        timer_state.time_remaining = new_time
      }
    }
  });

  thread::spawn(move || {
    loop {
      let mut selection: String = String::new();
      get_input(&mut selection);
      tx.send(selection).unwrap();
    }
  });

  loop {
    let selection = rx.recv().unwrap();
    // thread::sleep(Duration::from_secs(1));
    // let mut selection: String = String::new();
    // get_input(&mut selection);
    match selection.to_lowercase().as_str() {
      "1" => toggle_session(&mut timer_state),
      "2" => skip_sesion(&mut timer_state),
      "q" | "3" | "quit" | "exit" => break,
      _ => println!("not valid command"),
    }
  }
}
