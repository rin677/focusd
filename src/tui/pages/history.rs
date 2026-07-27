use crate::{
  database::history::get_full_history_no_err,
  timer::utils::name_for_session,
  tui::merge_block::MergeBlock,
  utils::{figlet::big_text, times_ago::render_duration},
};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Paragraph, Row, Table},
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
    let area = MergeBlock::new("").top().render(frame, area);
    let all_history = get_full_history_no_err();

    if all_history.is_empty() {
      frame.render_widget(Paragraph::new(big_text("No history yet")).centered(), area);
      return;
    }

    let table_heading = Row::new(["Date", "Type", "Duration", "Status"]).bottom_margin(1);

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

  pub fn history_up(&mut self) {
    if self.crr_history_index > 0 {
      self.crr_history_index -= 1;
    }
  }
  pub fn history_down(&mut self) {
    if self.crr_history_index < self.max_history_index {
      self.crr_history_index += 1;
    }
  }
}
