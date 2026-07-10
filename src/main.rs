mod config;
mod daemon;
mod database;
mod notification;
mod stats;
mod timer;
mod tui;
mod utils;
use std::{env, io};

use crate::{
  config::settings::create_config_file,
  daemon::{ensure_daemon_active, run_daemon},
  stats::calculate::print_stats,
  timer::state::TimerState,
  utils::ignore::Ignore,
};

fn main() -> io::Result<()> {
  let mut state = TimerState::default();
  create_config_file();

  let args: Vec<String> = env::args().collect();

  if args.iter().any(|a| a == "--daemon") {
    run_daemon();
    return Ok(());
  }
  ensure_daemon_active(true).ignore();

  if args.len() > 1 {
    match args[1].as_str() {
      // FIX: Stats should open stats page not just print the stats
      "stats" | "stat" => {
        print_stats();
        Ok(())
      }
      "tui" => tui::app::main(&mut state),
      // TODO: Other commands like pause/play/resume to control when playing from daemon
      // TODO: Commands to directly go to specifig tab like history/settings
      _ => tui::app::main(&mut state),
    }
  } else {
    tui::app::main(&mut state)
  }
}
