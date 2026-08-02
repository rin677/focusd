use crate::{config::settings::get_config, timer::state::SessionType};
use std::process::Command;

fn run_command(command: Option<String>) {
  let Some(cmd) = command else { return };
  let parts: Vec<&str> = cmd.split_whitespace().collect();
  if parts.is_empty() {
    return;
  }

  let mut cmd = Command::new(parts[0]);
  if parts.len() > 1 {
    cmd.args(&parts[1..]);
  }

  let _ = cmd.output();
}

pub fn session_start_hook(session: SessionType) {
  let cfg = get_config();
  match session {
    SessionType::Work => run_command(cfg.hook_start_work),
    SessionType::LongBreak => run_command(cfg.hook_start_long_break),
    SessionType::ShortBreak => run_command(cfg.hook_start_short_break),
  }
}
