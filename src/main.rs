mod config;
mod timer;
mod tui;
use std::{io, time::Duration};

use crate::{
  config::settings::create_config_file,
  timer::state::{SessionType, TimerState},
};

fn main() -> io::Result<()> {
  let mut state = TimerState {
    running: false,
    sessioin_type: SessionType::Work,
    time_remaining: Duration::from_mins(25),
  };
  create_config_file();
  tui::app::main(&mut state)
}
