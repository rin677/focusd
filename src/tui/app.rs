use crate::{
  daemon::commands::{Message, get_timer_state, send_command},
  timer::{engine::render_time, state::TimerState, utils::name_for_session},
  tui::pages::{
    history::{get_history_index, scroll_history_down, scroll_history_up, show_history},
    settings::show_settings,
    stats::show_stats,
    timer::show_timer,
  },
  utils::ignore::IgnoreType,
};
use std::{
  io,
  time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  layout::{Constraint, Layout},
  text::{Line, Text},
  widgets::Block,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pages {
  Timer,
  Stats,
  History,
  Settings,
}

pub struct AppState {
  exit: bool,
  current_page: Pages,
  pub max_history_index: usize,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      exit: false,
      current_page: Pages::Timer,
      max_history_index: 10,
    }
  }
}

struct App {
  app_state: AppState,
  timer_state: TimerState,
  all_pages: Vec<Pages>,
}

pub fn main(page: Pages) -> io::Result<()> {
  let state = get_timer_state()?;
  let mut app_state = AppState::default();
  app_state.current_page = page;
  let mut app = App {
    app_state,
    timer_state: state,
    all_pages: vec![Pages::Timer, Pages::Stats, Pages::History, Pages::Settings],
  };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
}

impl App {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    while !self.app_state.exit {
      let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or(Duration::from_secs(0));

      if last_tick.elapsed() >= tick_rate {
        self.timer_state = get_timer_state()?;
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
    let icon = if self.timer_state.running {
      ""
    } else {
      ""
    };
    let right_header_text = format!(
      "Current session: {} - {icon} {}",
      name_for_session(self.timer_state.session_type),
      render_time(&self.timer_state)
    );
    let top_layout = Layout::horizontal([
      Constraint::Fill(0),
      Constraint::Length(right_header_text.len() as u16),
    ])
    .spacing(2);

    let outer_layout = Layout::vertical([
      Constraint::Length(1), // Header
      Constraint::Fill(1),   // Main area
    ])
    .spacing(0);

    let [top, main_area] = frame.area().layout(&outer_layout);

    let [left_space, right_space] = top.layout(&top_layout);
    frame.render_widget(Text::from("Focusd"), left_space);
    frame.render_widget(Text::from(right_header_text), right_space);

    let main_block = Block::bordered().title_bottom(
      Line::from(":Press q to quit: Space to toggle timer : N to skip : R to reset").centered(),
    );
    frame.render_widget(&main_block, main_area);

    let inner_area = main_block.inner(main_area);
    match self.app_state.current_page {
      Pages::Timer => show_timer(&self.timer_state, inner_area, frame),
      Pages::History => show_history(inner_area, frame, &mut self.app_state),
      Pages::Stats => show_stats(&self.timer_state, inner_area, frame.buffer_mut()),
      Pages::Settings => show_settings(&self.timer_state, inner_area, frame.buffer_mut()),
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
      KeyCode::Char(' ') => send_command(Message::ToggleSession).ignore_type(),
      KeyCode::Char('n') => send_command(Message::NextSession).ignore_type(),
      KeyCode::Char('r') => send_command(Message::ResetSession).ignore_type(),
      KeyCode::Char('[') => self.select_page(-1),
      KeyCode::Char(']') => self.select_page(1),
      KeyCode::Char('j') => self.history_down(),
      KeyCode::Char('k') => self.history_up(),
      _ => {}
    }
  }

  fn quit(&mut self) {
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

  fn history_up(&self) {
    if self.app_state.current_page == Pages::History && get_history_index() > 0 {
      scroll_history_up();
    }
  }
  fn history_down(&self) {
    if self.app_state.current_page == Pages::History
      && get_history_index() < self.app_state.max_history_index
    {
      scroll_history_down();
    }
  }
}
