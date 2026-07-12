use crate::{
  database::history::get_full_history_no_err,
  stats::calculate::{
    DurType, get_completed_sessions, get_completion_rate, get_current_streak,
    get_daily_work_durations_7_days, get_daily_work_durations_n_days, get_total_time,
  },
  utils::times_ago::render_duration,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::{Bar, BarChart, Block, Paragraph},
};

pub fn show_stats(area: Rect, frame: &mut Frame) {
  let layout = Layout::vertical([
    Constraint::Max(2), // Total ...
    Constraint::Max(3), // streek and completion rate
    Constraint::Fill(1), // Heatmap
    Constraint::Fill(1), // Bar chart
  ])
  .spacing(1);

  let [first, second, heatmap_area, chart_area] = area.layout(&layout);
  redner_total_row(first, frame);
  render_second_row(second, frame);
  render_heatmap(heatmap_area, frame);
  render_bar_chart(chart_area, frame);
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

fn render_bar_chart(area: Rect, frame: &mut Frame) {
  let block = Block::bordered().title("Daily Focus (7 days)");
  let inner = block.inner(area);
  frame.render_widget(block, area);

  let data = get_daily_work_durations_7_days();
  let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);
  let bars: Vec<Bar> = data
    .into_iter()
    .map(|(label, value)| {
      Bar::with_label(label, value).text_value(fmt_duration_short(value))
    })
    .collect();
  let chart = BarChart::new(bars).max(max_value).bar_width(5).bar_gap(1);
  frame.render_widget(chart, inner);
}

fn render_heatmap(area: Rect, frame: &mut Frame) {
  let block = Block::bordered().title("Daily Focus (4 weeks)");
  let inner = block.inner(area);
  frame.render_widget(block, area);

  let data = get_daily_work_durations_n_days(28);
  if data.is_empty() {
    return;
  }

  let max_val = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);
  let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

  let header: Line = Line::from(
    std::iter::once("     ".to_string())
      .chain(days.iter().map(|d| format!(" {:>3}", d)))
      .collect::<Vec<_>>()
      .join(""),
  );

  let mut lines = vec![header];

  for week in (0..4).rev() {
    let week_label = if week == 0 {
      "This".to_string()
    } else {
      format!("W-{}", week)
    };
    let mut spans = vec![Span::raw(format!("{:>5} ", week_label))];
    for (di, _day_name) in days.iter().enumerate() {
      let idx = (3 - week) * 7 + di;
      if idx < data.len() {
        let (_, val) = &data[idx];
        let intensity = if max_val > 0 {
          (*val as f64 / max_val as f64 * 4.0).round() as usize
        } else {
          0
        };
        let bg = heat_color(intensity);
        spans.push(Span::styled("  ", Style::new().bg(bg)));
        spans.push(Span::raw(" "));
      }
    }
    lines.push(Line::from(spans));
  }

  let p = Paragraph::new(Text::from(lines));
  frame.render_widget(p, inner);
}

fn heat_color(intensity: usize) -> Color {
  match intensity {
    0 => Color::Reset,
    1 => Color::Rgb(0x1a, 0x1a, 0x2e),
    2 => Color::Rgb(0x1e, 0x3a, 0x5f),
    3 => Color::Rgb(0x2d, 0x6a, 0x9f),
    _ => Color::Rgb(0x4f, 0xad, 0xd7),
  }
}

fn fmt_duration_short(seconds: u64) -> String {
  if seconds >= 3600 {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    if m > 0 {
      format!("{}h {:02}m", h, m)
    } else {
      format!("{}h", h)
    }
  } else if seconds >= 60 {
    format!("{}m", seconds / 60)
  } else {
    format!("{}s", seconds)
  }
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
