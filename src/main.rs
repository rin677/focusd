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
use std::io;

use crate::{
  config::settings::create_config_file,
  daemon::{
    commands::{Message, PayloadMessage, send_message, send_message_with_payload},
    run::{ensure_daemon_active, run_daemon},
  },
  database::history::print_history,
  stats::calculate::print_stats,
  tui::app::Pages,
  utils::print::Print,
};
use clap::{Parser, Subcommand, ValueEnum};

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
  #[arg(short, long, value_name="MINUTES")]
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
  Reset,
  /// Reset the session if running
  Stop,
  /// Skip to next session
  Next,
  /// Skip to next session
  Skip,
}

fn main() -> io::Result<()> {
  create_config_file();

  let cli = Cli::parse();
  if cli.daemon {
    run_daemon();
    return Ok(());
  }
  if cli.stop_daemon {
    send_message(Message::StopDaemon).map(|m| println!("{m}"))?;
    return Ok(());
  }

  ensure_daemon_active(true)?;

  if let Some(p) = cli.preset {
    send_message_with_payload(PayloadMessage::SelectPreset, p.into()).print()?;
    return Ok(());
  }
  if let Some(a) = cli.add_minutes {
    send_message_with_payload(PayloadMessage::AddMinutes, a.into()).print()?;
    return Ok(());
  }
  if let Some(session) = cli.start {
    match session {
      None => {
        send_message(Message::StartSession).print()?;
      }
      Some(s) => {
        let name = match s {
          StartSession::Work => "Work",
          StartSession::ShortBreak => "Short Break",
          StartSession::LongBreak => "Long Break",
        };
        send_message_with_payload(PayloadMessage::SelectSession, name.into()).print()?;
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

    Some(Command::Pause) => send_message(Message::PauseSession).print(),
    Some(Command::Resume) => send_message(Message::ResumeSession).print(),
    Some(Command::Toggle) => send_message(Message::ToggleSession).print(),
    Some(Command::Reset) | Some(Command::Stop) => send_message(Message::ResetSession).print(),
    Some(Command::Next) | Some(Command::Skip) => send_message(Message::NextSession).print(),
    None => launch_tui(Pages::Timer),
  }
}
