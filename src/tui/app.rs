use crate::{
  config::{
    settings::get_crr_preset,
    themes::{get_current_theme, themed_block},
  },
  daemon::commands::{Message, get_timer_state, send_message},
  timer::{engine::render_time, state::TimerState, utils::name_for_session},
  tui::{
    pages::{history::HistoryPage, settings::SettingsPage, stats::show_stats, timer::TimerPage},
    popup::Popup,
  },
  utils::{cycle::cycle_item, ignore::IgnoreType},
};
use std::{
  io,
  time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  layout::{Constraint, Layout, Spacing},
  text::Text,
  widgets::Paragraph,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Pages {
  #[default]
  Timer,
  Stats,
  History,
  Settings,
}

#[derive(Default)]
pub struct AppState {
  exit: bool,
  current_page: Pages,
  timer_page: TimerPage,
  history_page: HistoryPage,
  settings_page: SettingsPage,
}

struct App {
  app_state: AppState,
  timer_state: TimerState,
  all_pages: Vec<Pages>,
  popup: Popup,
}

pub fn main(page: Pages) -> io::Result<()> {
  let state = get_timer_state()?;
  let app_state = AppState {
    current_page: page,
    ..Default::default()
  };
  let mut app = App {
    app_state,
    timer_state: state,
    popup: Popup::default(),
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
    let theme = get_current_theme();
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
      Pages::Settings => "Settings",
    };

    let area = frame.area();
    let [header, inner_area, footer] = Layout::vertical([
      Constraint::Length(3), // Header
      Constraint::Fill(1),   // Content
      Constraint::Length(3), // Footer
    ])
    .spacing(Spacing::Overlap(1))
    .areas(area);

    let content_block = themed_block();
    frame.render_widget(&content_block, inner_area);

    let header_block = themed_block();
    frame.render_widget(&header_block, header);
    let header_inner = header_block.inner(header);

    let top_layout = Layout::horizontal([
      Constraint::Fill(0),
      Constraint::Length(right_header_text.len() as u16),
    ])
    .spacing(2);
    let [left_space, right_space] = header_inner.layout(&top_layout);
    frame.render_widget(Text::from(format!(" Focusd: {current_page} ")), left_space);
    frame.render_widget(Text::styled(right_header_text, theme.accent), right_space);
    let page_area = content_block.inner(inner_area);

    match self.app_state.current_page {
      Pages::Timer => self
        .app_state
        .timer_page
        .render(&self.timer_state, inner_area, frame),
      Pages::Stats => show_stats(inner_area, frame),
      Pages::History => self.app_state.history_page.render(page_area, frame),
      Pages::Settings => self.app_state.settings_page.render(page_area, frame),
    }

    let footer_block = themed_block();
    frame.render_widget(&footer_block, footer);
    let footer_text = footer_block.inner(footer);
    let hint = if self.app_state.current_page == Pages::Settings {
      format!(" · {}", self.app_state.settings_page.footer_hint())
    } else {
      String::new()
    };
    frame.render_widget(
      Paragraph::new(format!("Space Toggle timer · ? Keymaps {hint} ")).centered(),
      footer_text,
    );

    self.popup.render(self.app_state.current_page, area, frame);
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
    if self.app_state.current_page == Pages::Settings && self.app_state.settings_page.is_editing() {
      self.app_state.settings_page.handle_editing_key(key_event);
      return;
    }
    match key_event.code {
      KeyCode::Char('q') => {
        if self.popup.showen {
          self.popup.hide()
        } else {
          self.quit()
        }
      }
      KeyCode::Char(' ') => send_message(Message::ToggleSession).ignore_type(),
      KeyCode::Char('n') => send_message(Message::NextSession).ignore_type(),
      KeyCode::Char('r') => send_message(Message::ResetSession).ignore_type(),
      KeyCode::Char('[') => self.select_page(-1),
      KeyCode::Char(']') => self.select_page(1),
      KeyCode::Char('?') => self.popup.toggle(),
      KeyCode::Esc => {
        if self.app_state.current_page == Pages::Settings {
          self.app_state.settings_page.exit_sub_menu();
        } else {
          self.popup.hide()
        }
      }
      KeyCode::Char('j') | KeyCode::Down => match self.app_state.current_page {
        Pages::Timer => self.app_state.timer_page.down(),
        Pages::History => self.app_state.history_page.down(),
        Pages::Settings => self.app_state.settings_page.down(),
        _ => {}
      },
      KeyCode::Char('k') | KeyCode::Up => match self.app_state.current_page {
        Pages::Timer => self.app_state.timer_page.up(),
        Pages::History => self.app_state.history_page.up(),
        Pages::Settings => self.app_state.settings_page.up(),
        _ => {}
      },
      KeyCode::Enter => match self.app_state.current_page {
        Pages::Timer => self.app_state.timer_page.select_preset(),
        Pages::Settings => self.app_state.settings_page.handle_right(),
        _ => {}
      },
      KeyCode::Char('l') | KeyCode::Right => {
        if self.app_state.current_page == Pages::Settings {
          self.app_state.settings_page.handle_right()
        }
      }
      KeyCode::Char('h') | KeyCode::Left => {
        if self.app_state.current_page == Pages::Settings {
          self.app_state.settings_page.handle_left()
        }
      }

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
    self.app_state.current_page = cycle_item(self.app_state.current_page, &self.all_pages, offset);
  }
}
