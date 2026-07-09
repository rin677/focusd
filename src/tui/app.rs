use std::{
  io,
  time::{Duration, Instant},
};

use crate::{
  database::history::add_session_to_db,
  timer::{
    engine::{decrease_sec, next_session, toggle_session},
    state::TimerState,
  },
  tui::{history::show_history, settings::show_settings, stats::show_stats, timer::show_timer},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  layout::{Constraint, Layout, Margin},
  text::Line,
  widgets::Block,
};

#[derive(Clone, Copy, PartialEq, Eq)]
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

struct App<'a> {
  app_state: AppState,
  timer_state: &'a mut TimerState,
  all_pages: Vec<Pages>,
}

pub fn main(state: &mut TimerState) -> io::Result<()> {
  let mut app = App {
    app_state: AppState::default(),
    timer_state: state,
    all_pages: vec![Pages::Timer, Pages::Stats, Pages::History, Pages::Settings],
  };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
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
    let outer = Block::bordered();
    let inner = frame.area().inner(Margin {
      vertical: 1,
      horizontal: 0,
    });

    frame.render_widget(outer, frame.area());

    let layout = Layout::vertical([
      Constraint::Length(1), // Header
      Constraint::Length(1), // Seperator
      Constraint::Fill(1),   // Main content
      Constraint::Length(1), // Another seperator
      Constraint::Length(1), // Bottom hints
    ])
    .spacing(0);

    let [top, sep1, mid, sep2, bot] = inner.layout(&layout);

    let title = Line::from(":TODO: Add this later. Maybe small timer or icons");
    let bottom_text = Line::from(":Press q to quit: Space to toggle timer : and N to skip :");
    frame.render_widget(title.centered(), top);

    frame.render_widget(bottom_text.centered(), bot);
    let width = sep1.width.saturating_sub(2) as usize;
    let divider = format!("├{}┤", "─".repeat(width));
    frame.render_widget(Line::from(divider.clone()), sep1);
    frame.render_widget(Line::from(divider), sep2);

    match self.app_state.current_page {
      Pages::Timer => show_timer(self.timer_state, mid, frame.buffer_mut()),
      Pages::History => show_history(self.timer_state, mid, frame.buffer_mut()),
      Pages::Stats => show_stats(self.timer_state, mid, frame.buffer_mut()),
      Pages::Settings => show_settings(self.timer_state, mid, frame.buffer_mut()),
    }
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
      KeyCode::Char('q') => self.quit(),
      KeyCode::Char(' ') => toggle_session(self.timer_state),
      KeyCode::Char('n') => next_session(self.timer_state),
      KeyCode::Char('[') => self.select_page(-1),
      KeyCode::Char(']') => self.select_page(1),
      _ => {}
    }
  }

  fn quit(&mut self) {
    add_session_to_db(self.timer_state);
    self.app_state.exit = true;
  }

  fn select_page(&mut self, offset: isize) {
    if let Some(i) = self
      .all_pages
      .iter()
      .position(|x| *x == self.app_state.current_page)
    {
      let total_pages = self.all_pages.len();
      let new_index = i as isize + offset;
      let m = new_index % total_pages as isize;
      let abs_new_index: usize = (total_pages as isize + m) as usize % total_pages;
      self.app_state.current_page = self.all_pages[abs_new_index]
    }
  }
}
