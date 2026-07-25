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
  timer::{state::SessionType, utils::name_for_session},
  tui::app::Pages,
  utils::print::Print,
};
use clap::{Parser, Subcommand};

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
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Launch TUI in stats page
  Stats,
  /// Launch TUI in history page
  History,
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
  /// Start the session
  Start,
  /// Start work session directly
  Work,
  /// Start short break session directly
  ShortBreak,
  /// Start long break session directly
  LongBreak,
}

// TODO: CLI arguments to start work/break directly
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

    Some(Command::Pause) => send_message(Message::PauseSession).print(),
    Some(Command::Resume) => send_message(Message::ResumeSession).print(),
    Some(Command::Toggle) => send_message(Message::ToggleSession).print(),
    Some(Command::Reset) | Some(Command::Stop) => send_message(Message::ResetSession).print(),
    Some(Command::Next) | Some(Command::Skip) => send_message(Message::NextSession).print(),
    Some(Command::Start) => send_message(Message::StartSession).print(),

    Some(Command::Work) => start_session(SessionType::Work),
    Some(Command::ShortBreak) => start_session(SessionType::ShortBreak),
    Some(Command::LongBreak) => start_session(SessionType::LongBreak),

    None => launch_tui(Pages::Timer),
  }
}

fn start_session(session_type: SessionType) -> io::Result<()> {
  send_message_with_payload(
    PayloadMessage::SelectSession,
    name_for_session(session_type).into(),
  )
  .print()
}
