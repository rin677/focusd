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
  daemon::run::{ensure_daemon_active, run_daemon},
  stats::calculate::print_stats,
  tui::app::Pages,
  utils::ignore::Ignore,
};

fn main() -> io::Result<()> {
  create_config_file();

  let args: Vec<String> = env::args().collect();

  if args.iter().any(|a| a == "--daemon") {
    run_daemon();
    return Ok(());
  }
  ensure_daemon_active(true).ignore();

  let lunch_tui = |page| tui::app::main(page);
  if args.len() > 1 {
    match args[1].as_str() {
      "print-stats" => {
        print_stats();
        Ok(())
      }
      "tui" | "timer" | "home" => lunch_tui(Pages::Timer),
      "stats" | "stat" => lunch_tui(Pages::Stats),
      "history" => lunch_tui(Pages::History),
      "settings" | "config" => lunch_tui(Pages::Settings),
      // TODO: Other commands like pause/play/resume to control when playing from daemon
      _ => lunch_tui(Pages::Timer),
    }
  } else {
    lunch_tui(Pages::Timer)
  }
}
