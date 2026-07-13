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
  daemon::{
    commands::{Message, send_command},
    run::{ensure_daemon_active, run_daemon},
  },
  database::history::print_history,
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
  if args.iter().any(|a| a == "stop-daemon") {
    send_command(Message::StopDaemon)
      .map(|m| println!("{m}"))
      .ignore();
    return Ok(());
  }
  // Don't enable daemon if just help tag
  if args.iter().any(|a| a == "--help") {
    println!(
      "Usage: focusd [command]

  Commands:
    TUI commands
      tui|timer|home     Lunch TUI in timer page (just running `focusd` also opens this page
      stats|stat         Lunch TUI in stats page
      history            Lunch TUI in history page

    Commands to change timer state:
      pause|pause-session                      Pause timer if running
      resume|resume-session                    Resume session if paused
      toggle|toggle-session                    Toggle between running and paused state
      reset|stop|reset-session|stop-session    Stop session if running
      next|next-session|skip|skip-session      Skip to next session
      start|start-session                      Start the session
    

    Commands to print:
      print-stats
      print-history
      state|status        Show timer status in JSON format

    Daemon commands:
      --daemon                 Run daemon in background
      stop-daemon              Stop daemon if running"
    );
    return Ok(());
  }

  ensure_daemon_active(true).ignore();

  let lunch_tui = |page| tui::app::main(page);
  if args.len() > 1 {
    match args[1].as_str() {
      // Commands to print stuff
      "print-stats" => {
        print_stats();
        Ok(())
      }
      "print-history" => {
        print_history();
        Ok(())
      }
      "state" | "status" => send_command(Message::GetSession).map(|m| println!("{m}")),

      // Commands to open TUI in specific pages
      "tui" | "timer" | "home" => lunch_tui(Pages::Timer),
      "stats" | "stat" => lunch_tui(Pages::Stats),
      "history" => lunch_tui(Pages::History),
      // "settings" | "config" => lunch_tui(Pages::Settings),

      // Commands to change state of timer
      "pause" | "pause-session" => send_command(Message::PauseSession).map(|m| println!("{m}")),
      "resume" | "resume-session" => send_command(Message::ResumeSession).map(|m| println!("{m}")),
      "toggle" | "toggle-session" => send_command(Message::ToggleSession).map(|m| println!("{m}")),
      "reset" | "stop" | "reset-session" | "stop-session" => {
        send_command(Message::ResetSession).map(|m| println!("{m}"))
      }
      "next" | "next-session" | "skip" | "skip-session" => {
        send_command(Message::NextSession).map(|m| println!("{m}"))
      }
      "start" | "start-session" => send_command(Message::StartSession).map(|m| println!("{m}")),
      _ => lunch_tui(Pages::Timer),
    }
  } else {
    lunch_tui(Pages::Timer)
  }
}
