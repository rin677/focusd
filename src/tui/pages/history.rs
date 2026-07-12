use crate::{
  database::history::get_full_history_no_err, timer::utils::name_for_session, tui::app::AppState,
  utils::times_ago::render_duration,
};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Paragraph, Row, Table},
};
use std::cmp::min;
use std::sync::Mutex;

static START_INDEX: Mutex<usize> = Mutex::new(0);

pub fn scroll_history_down() {
  let mut n = START_INDEX.lock().unwrap();
  *n = n.saturating_add(1);
}

pub fn scroll_history_up() {
  let mut n = START_INDEX.lock().unwrap();
  *n = n.saturating_sub(1);
}

pub fn get_history_index() -> usize {
  *START_INDEX.lock().unwrap()
}

pub fn show_history(area: Rect, frame: &mut Frame, app_state: &mut AppState) {
  let all_history = get_full_history_no_err();

  if all_history.is_empty() {
    frame.render_widget(Paragraph::new("No history yet").centered(), area);
  }

  let table_heading = Row::new(["Date", "Type", "Duration", "Status"]).bottom_margin(1);

  let row_available = area.height as usize - 2;
  let start = min(get_history_index(), app_state.max_history_index);
  let n = min(start + row_available, all_history.len());
  if all_history.len() > row_available {
    app_state.max_history_index = all_history.len() - row_available
  } else {
    app_state.max_history_index = 0
  }

  let mut table_items: Vec<Row> = Vec::new();
  for i in start..n {
    let history = &all_history[i];
    let status = if history.completed_duration == history.planned_duration {
      "Completed"
    } else {
      "Incomplete"
    };
    let r = Row::new([
      history.end_time.format("%Y-%m-%d").to_string(),
      name_for_session(history.session_type).to_string(),
      render_duration(history.completed_duration),
      status.to_string(),
    ]);
    table_items.push(r);
  }

  let widths = [
    Constraint::Percentage(20),
    Constraint::Percentage(20),
    Constraint::Percentage(35),
    Constraint::Percentage(25),
  ];
  let list = Table::new(table_items, widths).header(table_heading);

  frame.render_widget(list, area);
}
