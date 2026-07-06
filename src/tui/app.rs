use std::io;

use crate::timer::{
  engine::render_time,
  state::{SessionType, TimerState},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  widgets::{Block, Paragraph, Widget},
};

pub fn main(state: TimerState) -> io::Result<()> {
  let mut app = App { exit: false, state };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
}

struct App {
  exit: bool,
  state: TimerState,
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
    let text = render_time(&self.state);
    Paragraph::new(text)
      .centered()
      .block(block)
      .render(area, buf);
  }
}
