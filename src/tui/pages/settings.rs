use crate::utils::figlet::big_text;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

// TODO: Implement this
#[allow(dead_code)]
#[derive(Default)]
pub struct SettingsPage {
  pub selected_setting_index: usize,
}

impl SettingsPage {
  pub fn render(&self, area: Rect, frame: &mut Frame) {
    let text = big_text("Settings will be here");
    let widget = Paragraph::new(text).centered();
    frame.render_widget(widget, area);
  }
}
