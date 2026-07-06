use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  widgets::{Block, Paragraph, Widget},
};

pub fn main() -> io::Result<()> {
  ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
struct App {
  exit: bool,
}

impl App {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn draw(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      }
      _ => {}
    }
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit = true,
      KeyCode::Char('s') => self.exit = true,
      _ => {}
    }
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered();
    Paragraph::new("hello")
      .centered()
      .block(block)
      .render(area, buf);
  }
}
