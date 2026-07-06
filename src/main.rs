mod timer;
mod tui;
use std::{io, time::Duration};

use crate::timer::state::{SessionType, TimerState};

fn main() -> io::Result<()> {
  let mut state = TimerState {
    running: false,
    sessioin_type: SessionType::Work,
    time_remaining: Duration::from_mins(25),
  };
  tui::app::main(&mut state)
}
