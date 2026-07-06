mod timer;

use std::io;
use std::time::Duration;
use timer::engine::{render_time, skip_sesion, toggle_session};
use timer::state::{SessionType, TimerState};

fn get_input(v: &mut String) {
  io::stdin().read_line(v).expect("Failed to read input");
  *v = v.trim().to_string()
}

fn main() {
  let mut timer_state = TimerState {
    running: false,
    time_remaining: Duration::from_mins(25),
    sessioin_type: SessionType::Work,
  };

  loop {
    render_time(&mut timer_state);
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

    let mut selection: String = String::new();
    get_input(&mut selection);
    match selection.to_lowercase().as_str() {
      "1" => toggle_session(),
      "2" => skip_sesion(),
      "q" | "3" | "quit" | "exit" => break,
      _ => println!("not valid command"),
    }
  }
}
