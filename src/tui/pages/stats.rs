use crate::{
  database::history::get_full_history_no_err,
  stats::calculate::{
    DurType, get_completed_sessions, get_completion_rate, get_current_streak,
    get_daily_work_durations_7_days, get_daily_work_durations_n_days,
    get_session_type_distribution, get_total_time,
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
use tui_piechart::{PieChart, PieSlice, symbols};

pub fn show_stats(area: Rect, frame: &mut Frame) {
  let layout = Layout::vertical([
    Constraint::Max(4),  // Total (block border + 2 content)
    Constraint::Max(5),  // Streak (block border + 3 content)
    Constraint::Fill(1), // Heatmap | Pie chart
    Constraint::Fill(1), // Bar chart
  ])
  .spacing(1);

  let [first, second, mid, chart_area] = area.layout(&layout);
  redner_total_row(first, frame);
  render_second_row(second, frame);

  let mid_split =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1);
  let [heatmap_area, pie_area] = mid.layout(&mid_split);
  render_heatmap(heatmap_area, frame);
  render_pie_chart(pie_area, frame);

  render_bar_chart(chart_area, frame);
}

fn render_second_row(area: Rect, frame: &mut Frame) {
  let block = Block::bordered().title("Streak & Completion");
  let inner = block.inner(area);
  frame.render_widget(block, area);

  let layout = Layout::vertical([Constraint::Max(1), Constraint::Max(1)]);
  let [streek, completion] = inner.layout(&layout);
  let streek_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
  let completion_layout =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

  let [streek_left, streek_right] = streek.layout(&streek_layout);
  let [completion_left, completion_right] = completion.layout(&completion_layout);

  let h = get_full_history_no_err();
  let current_streak = Paragraph::new(format!("Current Streak: {}", get_current_streak(h)));
  let completion_rate = Paragraph::new(format!("Completion rate: {:.0}%", get_completion_rate()));
  let completed_sessions =
    Paragraph::new(format!("Completed sessions: {}", get_completed_sessions()));
  let longest_streak = Paragraph::new(format!("Longest streak: {}", "TODO"));

  frame.render_widget(current_streak, streek_left);
  frame.render_widget(completion_rate, completion_left);
  frame.render_widget(completed_sessions, completion_right);
  frame.render_widget(longest_streak, streek_right);
}

fn render_bar_chart(area: Rect, frame: &mut Frame) {
  let bar_width: u16 = 3;
  let bar_gap: u16 = 1;
  let cols_per_bar = (bar_width + bar_gap) as usize;

  let block = Block::bordered().title("Daily Focus");
  let inner = block.inner(area);
  frame.render_widget(block, area);

  let days = ((inner.width as usize) / cols_per_bar).max(7).min(90);

  let (data, max_value) = if days == 7 {
    let d = get_daily_work_durations_7_days();
    let max = d.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);
    (d.into_iter().map(|(l, v)| (l, v)).collect::<Vec<_>>(), max)
  } else {
    let d = get_daily_work_durations_n_days(days);
    let max = d.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);
    (
      d.into_iter()
        .map(|(date_str, v)| {
          let label = date_str[8..10].trim_start_matches('0').to_string();
          (label, v)
        })
        .collect::<Vec<_>>(),
      max,
    )
  };

  let bars: Vec<Bar> = data
    .into_iter()
    .map(|(label, value)| Bar::with_label(label, value).text_value(fmt_duration_short(value)))
    .collect();
  let chart = BarChart::new(bars)
    .max(max_value)
    .bar_width(bar_width)
    .bar_gap(bar_gap);
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

  let mut indicator = vec![Span::raw("Less ")];
  for i in 0..=4 {
    indicator.push(Span::styled("  ", Style::new().bg(heat_color(i))));
    indicator.push(Span::raw(" "));
  }
  indicator.push(Span::raw(" More"));
  lines.push(Line::from(indicator));

  let p = Paragraph::new(Text::from(lines));
  frame.render_widget(p, inner);
}

fn render_pie_chart(area: Rect, frame: &mut Frame) {
  let dist = get_session_type_distribution();
  if dist.is_empty() {
    return;
  }

  let colors = [
    Color::Cyan,
    Color::Yellow,
    Color::Magenta,
    Color::Green,
    Color::Red,
  ];
  let slices: Vec<PieSlice> = dist
    .iter()
    .enumerate()
    .map(|(i, (name, val))| PieSlice::new(name, *val, colors[i % colors.len()]))
    .collect();

  let chart = PieChart::new(slices)
    .pie_char(symbols::PIE_CHAR_BLOCK)
    .block(Block::bordered().title("Session Types"))
    .show_legend(true)
    .show_percentages(false);
  frame.render_widget(chart, area);
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
  let block = Block::bordered().title("Time Summary");
  let inner = block.inner(area);
  frame.render_widget(block, area);

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
  ] = inner.layout(&total_layout);
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
