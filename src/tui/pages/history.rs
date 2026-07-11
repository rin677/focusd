use crate::{database::history::get_full_history_no_err, timer::utils::name_for_session};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Row, Table},
};

pub fn show_history(area: Rect, frame: &mut Frame) {
  let all_history = get_full_history_no_err();
  let table_heading = Row::new(["Date", "Type", "Duration (mins)", "Status"]);
  let table_items: Vec<Row> = all_history
    .into_iter()
    .map(|history| {
      let status = if history.completed_duration == history.planned_duration {
        "Completed"
      } else {
        "Incomplete"
      };
      Row::new([
        history.end_time.format("%Y-%m-%d").to_string(),
        name_for_session(history.session_type).to_string(),
        (history.completed_duration / 60).to_string(),
        status.to_string(),
      ])
    })
    .collect();
  let widths = [
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
  ];
  let list = Table::new(table_items, widths).header(table_heading);

  frame.render_widget(list, area);
}
