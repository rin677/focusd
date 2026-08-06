use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThemeName {
  Catpuccin,
}

#[derive(Clone)]
pub struct Theme {
  normal: Style,
  selected: Style,
  title: Style,
  border: Style,
}

impl Theme {
  fn new(name: ThemeName) -> Self {
    match name {
      ThemeName::Catpuccin => Self {
        normal: Style::default().fg(Color::White).bg(Color::Black),
        selected: Style::default().fg(Color::Black).bg(Color::Cyan),
        title: Style::default().fg(Color::Yellow).bold(),
        border: Style::default().fg(Color::Blue),
      },
    }
  }
}
