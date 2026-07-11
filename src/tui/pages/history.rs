use std::sync::Mutex;

use crate::{
  database::history::get_full_history_no_err, timer::utils::name_for_session, tui::app::AppState,
};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Paragraph, Row, Table},
};

static START_INDEX: Mutex<usize> = Mutex::new(0);

pub fn scroll_history_down() {
  let mut n = START_INDEX.lock().unwrap();
  *n += 1;
}

pub fn scroll_history_up() {
  let mut n = START_INDEX.lock().unwrap();
  *n -= 1;
}

pub fn get_history_index() -> usize {
  *START_INDEX.lock().unwrap()
}

pub fn show_history(area: Rect, frame: &mut Frame, app_state: &mut AppState) {
  let all_history = get_full_history_no_err();

  if all_history.is_empty() {
    frame.render_widget(Paragraph::new("No history yet").centered(), area);
  }

  let table_heading = Row::new(["Date", "Type", "Duration (mins)", "Status"]);

  let row_available = area.height as usize - 2;
  let mut n = row_available;
  if all_history.len() > row_available {
    app_state.max_history_index = all_history.len() - row_available
  } else {
    n = all_history.len();
    app_state.max_history_index = 0
  }

  let mut table_items: Vec<Row> = Vec::new();
  for i in get_history_index()..n {
    let history = &all_history[i];
    let status = if history.completed_duration == history.planned_duration {
      "Completed"
    } else {
      "Incomplete"
    };
    let r = Row::new([
      history.end_time.format("%Y-%m-%d").to_string(),
      name_for_session(history.session_type).to_string(),
      (history.completed_duration / 60).to_string(),
      status.to_string(),
    ]);
    table_items.push(r);
  }

  let widths = [
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
  ];
  let list = Table::new(table_items, widths).header(table_heading);

  frame.render_widget(list, area);
}
