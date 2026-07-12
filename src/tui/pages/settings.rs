use crate::utils::figlet::big_text;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub fn show_settings(area: Rect, frame: &mut Frame) {
  // TODO: Implement this
  let text = big_text("Settings will be here");
  let widget = Paragraph::new(text).centered();
  frame.render_widget(widget, area);
}
