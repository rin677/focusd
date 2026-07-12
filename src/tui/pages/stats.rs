use crate::{
  database::history::get_full_history_no_err,
  stats::calculate::{
    DurType, get_completed_sessions, get_completion_rate, get_current_streak, get_total_time,
  },
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
  ])
  .spacing(2);

  let [first, second] = area.layout(&layout);
  redner_total_row(first, frame);
  render_second_row(second, frame);
}

fn render_second_row(area: Rect, frame: &mut Frame) {
  let layout = Layout::vertical([Constraint::Max(1), Constraint::Max(1)]);
  let [streek, completion] = area.layout(&layout);
  let streek_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
  let completion_layout =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

  let [streek_left, streek_right] = streek.layout(&streek_layout);
  let [completion_left, completion_right] = completion.layout(&completion_layout);

  // let curretn_streak = get_current_streak(all_history)
  let h = get_full_history_no_err();
  let current_streak = Paragraph::new(format!("Streak: {}", get_current_streak(h)));
  let completion_rate = Paragraph::new(format!("Completion rate: {}", get_completion_rate()));
  let completed_sessions =
    Paragraph::new(format!("Completed sessions: {}", get_completed_sessions()));
  let longest_streak = Paragraph::new(format!("Longest streak: {}", "TODO"));

  frame.render_widget(current_streak, streek_left);
  frame.render_widget(completion_rate, completion_left);
  frame.render_widget(completed_sessions, completion_right);
  frame.render_widget(longest_streak, streek_right);
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
