use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Block, Clear, Paragraph},
};

use crate::{config::settings::get_config, tui::app::Pages};

#[derive(Default)]
pub struct Popup {
  pub showen: bool,
}

impl Popup {
  pub fn render(&self, current_page: Pages, area: Rect, frame: &mut Frame) {
    if self.showen {
      let popup_block = Block::bordered().title("Help");
      let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
      // clears out any background in the area before rendering the popup
      frame.render_widget(Clear, centered_area);
      let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
      frame.render_widget(paragraph, centered_area);
    }
  }

  pub fn toggle(&mut self) {
    self.showen = !self.showen
  }
  pub fn hide(&mut self) {
    self.showen = false
  }

}
