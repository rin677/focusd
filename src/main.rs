mod config;
mod timer;
mod tui;
use std::io;

use crate::{config::settings::create_config_file, timer::state::TimerState};

fn main() -> io::Result<()> {
  let mut state = TimerState::default();
  create_config_file();
  tui::app::main(&mut state)
}
