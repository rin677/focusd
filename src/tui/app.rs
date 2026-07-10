use crate::{
  daemon::commands::{Message, get_timer_state, send_command},
  timer::{engine::render_time, utils::name_for_session},
  tui::pages::{
    history::show_history, settings::show_settings, stats::show_stats, timer::show_timer,
  },
  utils::ignore::Ignore,
};
use std::io;

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

struct App {
  app_state: AppState,
  all_pages: Vec<Pages>,
}

pub fn main() -> io::Result<()> {
  let mut app = App {
    app_state: AppState::default(),
    all_pages: vec![Pages::Timer, Pages::Stats, Pages::History, Pages::Settings],
  };
  let mut terminal = ratatui::init();
  let result = app.run(&mut terminal);
  ratatui::restore();
  result
}

impl App {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    // let tick_rate = Duration::from_secs(1);
    // let mut last_tick = Instant::now();
    while !self.app_state.exit {
      // let timeout = tick_rate
      //   .checked_sub(last_tick.elapsed())
      //   .unwrap_or(Duration::from_secs(0));

      // if last_tick.elapsed() >= tick_rate {
      //   if self.timer_state.running {
      //     decrease_sec(self.timer_state);
      //   }
      //   last_tick = Instant::now();
      // }

      terminal.draw(|frame| self.draw(frame))?;
      // if event::poll(timeout)? {
      self.handle_events()?;
      // }
    }
    Ok(())
  }

  fn draw(&mut self, frame: &mut Frame) {
    let timer_state = get_timer_state().unwrap();
    let icon = if timer_state.running { "" } else { "" };
    let right_header_text = format!(
      "Current session: {} - {icon} {}",
      name_for_session(timer_state.session_type),
      render_time(&timer_state)
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
      Line::from(":Press q to quit: Space to toggle timer : and N to skip :").centered(),
    );
    frame.render_widget(&main_block, main_area);

    let inner_area = main_block.inner(main_area);
    match self.app_state.current_page {
      Pages::Timer => show_timer(&timer_state, inner_area, frame),
      Pages::History => show_history(&timer_state, inner_area, frame.buffer_mut()),
      Pages::Stats => show_stats(&timer_state, inner_area, frame.buffer_mut()),
      Pages::Settings => show_settings(&timer_state, inner_area, frame.buffer_mut()),
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
      KeyCode::Char(' ') => send_command(Message::ToggleSession).ignore(),
      KeyCode::Char('n') => send_command(Message::NextSession).ignore(),
      KeyCode::Char('[') => self.select_page(-1),
      KeyCode::Char(']') => self.select_page(1),
      _ => {}
    }
  }

  fn quit(&mut self) {
    // add_session_to_db();
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
