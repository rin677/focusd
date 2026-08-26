//! A beautiful terminal pomodoro timer with daemon, waybar integration and interactive TUI.

mod config;
mod daemon;
mod database;
mod notification;
mod stats;
mod timer;
mod tui;
mod utils;
mod waybar;
use crate::{
  config::settings::create_config_file,
  daemon::{
    commands::{Message, PayloadMessage},
    run::{ensure_daemon_active, run_daemon},
  },
  database::history::print_history,
  stats::calculate::print_stats,
  tui::app::Pages,
  utils::print::Print,
};
use clap::{Parser, Subcommand, ValueEnum};
use std::io;

#[derive(ValueEnum, Clone, Debug)]
enum StartSession {
  Work,
  ShortBreak,
  LongBreak,
}

#[derive(Parser, Debug)]
#[command(name = "focusd")]
#[command(
  version,
  about,
  long_about = "A beautiful and feature rich terminal pomodoro timer.
Use focusd command to just open the timer."
)]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>,

  /// Start daemon
  #[arg(long, short)]
  daemon: bool,

  /// Stop deamon if already running
  #[arg(long)]
  stop_daemon: bool,

  /// Select specific preset
  #[arg(short, long)]
  preset: Option<String>,

  /// Add time to running timer
  #[arg(short, long, value_name = "MINUTES")]
  add_minutes: Option<u64>,

  /// Start a specific session
  ///
  /// work, short-break, long-break or current if empty
  #[arg(short, long, num_args = 0..=1, hide_possible_values = true, value_name = "SESSION")]
  start: Option<Option<StartSession>>,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Launch TUI in stats page
  Stats,
  /// Launch TUI in history page
  History,
  /// Launch TUI in settings page
  Settings,
  /// Print all stats
  PrintStats,
  /// Print all history
  PrintHistory,
  /// Get status (this is for waybar)
  Status,
  /// Pause the running session
  Pause,
  /// Resume the paused session
  Resume,
  /// Toggle the session
  Toggle,
  /// Reset the session if running
  #[command(alias = "stop")]
  Reset,
  /// Skip to next session
  #[command(alias = "skip")]
  Next,
}

fn main() -> io::Result<()> {
  create_config_file();

  let cli = Cli::parse();
  if cli.daemon {
    run_daemon();
    return Ok(());
  }
  if cli.stop_daemon {
    Message::StopDaemon.send().map(|m| println!("{m}"))?;
    return Ok(());
  }

  ensure_daemon_active(true)?;

  if let Some(p) = cli.preset {
    PayloadMessage::SelectPreset.send(p.into()).print()?;
    return Ok(());
  }
  if let Some(a) = cli.add_minutes {
    PayloadMessage::AddMinutes.send(a.into()).print()?;
    return Ok(());
  }
  if let Some(session) = cli.start {
    match session {
      None => {
        Message::StartSession.send().print()?;
      }
      Some(s) => {
        let name = match s {
          StartSession::Work => "Work",
          StartSession::ShortBreak => "Short Break",
          StartSession::LongBreak => "Long Break",
        };
        PayloadMessage::SelectSession.send(name.into()).print()?;
      }
    }
    return Ok(());
  }

  let launch_tui = |page| tui::app::main(page);

  match cli.command {
    Some(Command::PrintStats) => {
      print_stats();
      Ok(())
    }
    Some(Command::PrintHistory) => {
      print_history();
      Ok(())
    }
    Some(Command::Status) => {
      waybar::status();
      Ok(())
    }
    Some(Command::Stats) => launch_tui(Pages::Stats),
    Some(Command::History) => launch_tui(Pages::History),
    Some(Command::Settings) => launch_tui(Pages::Settings),

    Some(Command::Pause) => Message::PauseSession.send().print(),
    Some(Command::Resume) => Message::ResumeSession.send().print(),
    Some(Command::Toggle) => Message::ToggleSession.send().print(),
    Some(Command::Reset) => Message::ResetSession.send().print(),
    Some(Command::Next) => Message::NextSession.send().print(),
    None => launch_tui(Pages::Timer),
  }
}
