use crate::{
  config::settings::get_crr_preset,
  daemon::commands::{Message, get_timer_state, send_message},
  timer::{engine::render_time, state::TimerState, utils::name_for_session},
  tui::{
    merge_block::{MergeBlock, clear_bottom_tees},
    pages::{
      history::show_history,
      // settings::show_settings,
      stats::show_stats,
      timer::show_timer,
    },
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
  layout::{Constraint, Layout, Rect},
  text::Text,
  widgets::{Block, Paragraph},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pages {
  Timer,
  Stats,
  History,
  // TODO: Settings page
  // Settings,
}

pub struct AppState {
  exit: bool,
  current_page: Pages,
  pub max_history_index: usize,
  pub crr_history_index: usize,
  pub preset_start_index: usize,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      exit: false,
      current_page: Pages::Timer,
      max_history_index: 10,
      crr_history_index: 0,
      preset_start_index: 0,
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
    all_pages: vec![
      Pages::Timer,
      Pages::Stats,
      Pages::History,
      // Pages::Settings,
    ],
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
    clear_bottom_tees();
    let icon = if self.timer_state.running {
      ""
    } else {
      ""
    };
    let preset = get_crr_preset();
    let right_header_text = format!(
      "{} {}/{}  {icon} {}",
      name_for_session(self.timer_state.session_type),
      self.timer_state.session_number,
      preset.sessions_before_long_break,
      render_time(&self.timer_state),
    );
    let current_page = match self.app_state.current_page {
      Pages::Timer => "Timer",
      Pages::Stats => "Stats",
      Pages::History => "History",
    };

    let main_area = frame.area();
    let main_block = Block::bordered();
    frame.render_widget(&main_block, main_area);
    let main_inner = main_block.inner(main_area);

    let [header, inner_area, footer] = Layout::vertical([
      Constraint::Length(1), // Header
      Constraint::Fill(1),   // Content
      Constraint::Length(2), // Footer
    ])
    .spacing(0)
    .areas(main_inner);

    let top_layout = Layout::horizontal([
      Constraint::Fill(0),
      Constraint::Length(right_header_text.len() as u16),
    ])
    .spacing(2);
    let [left_space, right_space] = header.layout(&top_layout);
    frame.render_widget(Text::from(format!(" Focusd: {current_page} ")), left_space);
    frame.render_widget(Text::from(right_header_text), right_space);

    match self.app_state.current_page {
      Pages::Timer => show_timer(&self.timer_state, inner_area, frame, &mut self.app_state),
      Pages::History => show_history(inner_area, frame, &mut self.app_state),
      Pages::Stats => show_stats(inner_area, frame),
      // Pages::Settings => show_settings(inner_area, frame),
    }

    let footer_sep = Rect::new(footer.x, footer.y, footer.width, 1);
    let footer_text = Rect::new(footer.x, footer.y + 1, footer.width, 1);
    MergeBlock::new("")
      .top()
      .no_padding()
      .render(frame, footer_sep);
    frame.render_widget(
      Paragraph::new(" Press q to quit - Space to toggle timer - N to skip - R to reset ")
        .centered(),
      footer_text,
    );
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
      KeyCode::Char(' ') => send_message(Message::ToggleSession).ignore_type(),
      KeyCode::Char('n') => send_message(Message::NextSession).ignore_type(),
      KeyCode::Char('r') => send_message(Message::ResetSession).ignore_type(),
      KeyCode::Char('[') => self.select_page(-1),
      KeyCode::Char(']') => self.select_page(1),
      KeyCode::Char('j') | KeyCode::Down => match self.app_state.current_page {
        Pages::Timer => self.preset_down(),
        Pages::History => self.history_down(),
        _ => {}
      },
      KeyCode::Char('k') | KeyCode::Up => match self.app_state.current_page {
        Pages::Timer => self.preset_up(),
        Pages::History => self.history_up(),
        _ => {}
      },

      KeyCode::Char('c')
        if key_event
          .modifiers
          .contains(crossterm::event::KeyModifiers::CONTROL) =>
      {
        self.quit();
      }
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

  fn history_up(&mut self) {
    if self.app_state.crr_history_index > 0 {
      self.app_state.crr_history_index -= 1;
    }
  }
  fn history_down(&mut self) {
    if self.app_state.crr_history_index < self.app_state.max_history_index {
      self.app_state.crr_history_index += 1;
    }
  }

  fn preset_up(&mut self) {
    if self.app_state.preset_start_index > 0 {
      self.app_state.preset_start_index -= 1;
    }
  }
  fn preset_down(&mut self) {
    if self.app_state.preset_start_index < 19 {
      self.app_state.preset_start_index += 1;
    }
  }
}
