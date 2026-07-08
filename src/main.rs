mod config;
mod database;
mod stats;
mod timer;
mod tui;
mod utils;
use std::{env, io};

use crate::{
  config::settings::create_config_file, stats::calculate::print_stats, timer::state::TimerState,
};

fn main() -> io::Result<()> {
  let mut state = TimerState::default();
  create_config_file();

  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    match args[1].as_str() {
      "stats" | "stat" => {
        print_stats();
        Ok(())
      }
      "tui" => tui::app::main(&mut state),
      _ => tui::app::main(&mut state),
    }
  } else {
    tui::app::main(&mut state)
  }
}
