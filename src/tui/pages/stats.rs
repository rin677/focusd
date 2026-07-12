use crate::{
  stats::calculate::{DurType, get_total_time},
  utils::times_ago::render_duration,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  widgets::Paragraph,
};

pub fn show_stats(area: Rect, frame: &mut Frame) {
  let layout = Layout::vertical([
    Constraint::Max(2), // Total ...
    Constraint::Max(3), // streek and completion rate
  ]);

  let [first, second] = area.layout(&layout);
  redner_total_row(first, frame);
}

fn redner_total_row(area: Rect, frame: &mut Frame) {
  let total_layout = Layout::horizontal([
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
    Constraint::Percentage(25),
  ])
  .spacing(2);
  let [
    total_today_area,
    total_this_week_area,
    total_this_month_area,
    total_all_time_area,
  ] = area.layout(&total_layout);
  let total_today = Paragraph::new(format!(
    "Today\n{}",
    render_duration(get_total_time(DurType::Today))
  ))
  .centered();
  let total_this_week = Paragraph::new(format!(
    "This week\n{}",
    render_duration(get_total_time(DurType::Week))
  ))
  .centered();
  let total_this_month = Paragraph::new(format!(
    "This month\n{}",
    render_duration(get_total_time(DurType::Month))
  ))
  .centered();
  let total_all_time = Paragraph::new(format!(
    "All time\n{}",
    render_duration(get_total_time(DurType::All))
  ))
  .centered();

  frame.render_widget(total_today, total_today_area);
  frame.render_widget(total_this_week, total_this_week_area);
  frame.render_widget(total_this_month, total_this_month_area);
  frame.render_widget(total_all_time, total_all_time_area);
}
