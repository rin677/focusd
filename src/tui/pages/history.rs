use crate::{
  config::themes::get_current_theme,
  database::history::get_full_history_no_err,
  utils::{figlet::big_text, timer::render_duration},
};
use ratatui::prelude::Stylize;
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  style::Modifier,
  widgets::{Cell, Paragraph, Row, Table},
};
use std::cmp::min;

pub struct HistoryPage {
  pub max_history_index: usize,
  pub crr_history_index: usize,
}

impl Default for HistoryPage {
  fn default() -> Self {
    Self {
      max_history_index: 10,
      crr_history_index: 0,
    }
  }
}

impl HistoryPage {
  pub fn render(&mut self, area: Rect, frame: &mut Frame) {
    let all_history = get_full_history_no_err();

    if all_history.is_empty() {
      frame.render_widget(Paragraph::new(big_text("No history yet")).centered(), area);
      return;
    }

    let theme = get_current_theme();
    let table_heading = Row::new(["Date", "Type", "Duration", "Status"])
      .bottom_margin(1)
      .style(Modifier::BOLD)
      .fg(theme.accent);

    let row_available = area.height as usize - 2;
    let start = min(self.crr_history_index, self.max_history_index);
    let n = min(start + row_available, all_history.len());
    if all_history.len() > row_available {
      self.max_history_index = all_history.len() - row_available
    } else {
      self.max_history_index = 0
    }

    let mut table_items: Vec<Row> = Vec::new();
    for history in &all_history[start..n] {
      let status = if history.is_completed {
        Cell::from("Completed").style(theme.success)
      } else {
        Cell::from("Incomplete").style(theme.error)
      };
      let r = Row::new(vec![
        Cell::from(history.end_time.format("%Y-%m-%d").to_string()),
        Cell::from(history.session_type.name().to_string()),
        Cell::from(render_duration(history.completed_duration)),
        status,
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

  pub fn up(&mut self) {
    if self.crr_history_index > 0 {
      self.crr_history_index -= 1;
    }
  }
  pub fn down(&mut self) {
    if self.crr_history_index < self.max_history_index {
      self.crr_history_index += 1;
    }
  }
}
