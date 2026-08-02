use crate::{config::settings::get_config, tui::app::Pages};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Block, Clear, Paragraph},
};

#[derive(Default)]
pub struct Popup {
  pub showen: bool,
}

impl Popup {
  pub fn render(&self, current_page: Pages, area: Rect, frame: &mut Frame) {
    if self.showen {
      let popup_block = Block::bordered().title("Help");
      let keymaps = self.get_keymaps(current_page);
      let text: Vec<Line> = keymaps
        .iter()
        .map(|(key, desc)| {
          Line::from(vec![
            Span::styled(
              format!(" {:<10}", key),
              Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(desc),
          ])
        })
        .collect();
      let height = keymaps.len() as u16 + 2;
      let centered_area = area.centered(Constraint::Length(45), Constraint::Length(height));
      frame.render_widget(Clear, centered_area);
      let paragraph = Paragraph::new(text).block(popup_block);
      frame.render_widget(paragraph, centered_area);
    }
  }

  pub fn toggle(&mut self) {
    self.showen = !self.showen
  }
  pub fn hide(&mut self) {
    self.showen = false
  }

  pub fn get_keymaps(&self, current_page: Pages) -> Vec<(String, String)> {
    let mut keymaps: Vec<(String, String)> = vec![
      ("Space".to_string(), "Toggle Timer".to_string()),
      ("n".to_string(), "Next Session".to_string()),
      ("r".to_string(), "Reset Session".to_string()),
      ("q".to_string(), "Quit".to_string()),
      ("]".to_string(), "Next page".to_string()),
      ("[".to_string(), "Previous page".to_string()),
      ("?".to_string(), "Toggle help menu".to_string()),
    ];

    match current_page {
      Pages::Timer => {
        if get_config().tui_show_stats {
          keymaps.push(("Enter".to_string(), "Select the preset".to_string()));
        }
        keymaps.push(("k/↑".to_string(), "Prevoius preset".to_string()));
        keymaps.push(("j/↓".to_string(), "Next preset".to_string()));
      }
      Pages::History => {
        keymaps.push(("k/↑".to_string(), "Scroll up".to_string()));
        keymaps.push(("j/↓".to_string(), "Scroll down".to_string()));
      }
      Pages::Settings => {
        keymaps.push(("k/↑".to_string(), "Down".to_string()));
        keymaps.push(("j/↓".to_string(), "Up".to_string()));
        keymaps.push((
          "l/→".to_string(),
          "Open Sub menu or scroll through values".to_string(),
        ));
        keymaps.push(("h/←".to_string(), "Back".to_string()));
      }
      _ => {}
    }
    keymaps
  }
}
