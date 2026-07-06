mod timer;
mod tui;
use std::io;

fn main() -> io::Result<()> {
  tui::app::main()
}
