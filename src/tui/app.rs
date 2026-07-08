use figlet_rs::FIGlet;
use std::{
  io,
  time::{Duration, Instant},
};

use crate::{
  database::history::add_session_to_db,
  timer::{
    engine::{decrease_sec, next_session, render_time, toggle_session},
    state::TimerState,
  },
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  widgets::{Block, Paragraph, Widget},
};

pub enum Pages {
  Timer,
  Stats,
  History,
  Settings,
}

struct AppState {
  exit: bool,
  current_page: Pages,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      exit: false,
      current_page: Pages::Timer,
    }
  }
}

pub fn main(state: &mut TimerState) -> io::Result<()> {
  let mut app = App {
    app_state: AppState::default(),
    timer_state: state,
  };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
}

struct App<'a> {
  app_state: AppState,
  timer_state: &'a mut TimerState,
}

impl<'a> App<'a> {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    while !self.app_state.exit {
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
    let block = Block::bordered();
    let t = render_time(self.timer_state);
    let font = FIGlet::from_content(include_str!("../../resources/terminus.flf")).unwrap();
    // TODO: Also show session type and paused play (removed currently to show figlet)
    let text = font.convert(&t).unwrap().to_string();
    Paragraph::new(text)
      .centered()
      .block(block)
      .render(frame.area(), frame.buffer_mut());
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

  fn quit(&mut self) {
    add_session_to_db(self.timer_state);
    self.app_state.exit = true;
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.quit(),
      KeyCode::Char(' ') => toggle_session(self.timer_state),
      KeyCode::Char('n') => next_session(self.timer_state),
      _ => {}
    }
  }
}
