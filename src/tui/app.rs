use std::{
  io,
  time::{Duration, Instant},
};

use crate::timer::{
  engine::{decrease_sec, render_time, toggle_session},
  state::TimerState,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  widgets::{Block, Paragraph, Widget},
};

pub fn main(state: &mut TimerState) -> io::Result<()> {
  let mut app = App {
    exit: false,
    timer_state: state,
  };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
}

struct App<'a> {
  exit: bool,
  timer_state: &'a mut TimerState,
}

impl<'a> App<'a> {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    while !self.exit {
      let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or(Duration::from_secs(0));

      if last_tick.elapsed() >= tick_rate {
        if self.timer_state.running {
          decrease_sec(self.timer_state);
        }
        last_tick = Instant::now();
      }

      terminal.draw(|frame| self.draw(frame))?;
      if event::poll(timeout)? {
        self.handle_events()?;
      }
    }
    Ok(())
  }

  fn draw(&mut self, frame: &mut Frame) {
    frame.render_widget(&*self, frame.area());
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
      KeyCode::Char(' ') => toggle_session(self.timer_state),
      _ => {}
    }
  }
}

impl Widget for &App<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered();
    let text = render_time(self.timer_state);
    Paragraph::new(text)
      .centered()
      .block(block)
      .render(area, buf);
  }
}
