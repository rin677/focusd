use crate::config::settings::get_config;
use ratatui::{
  style::{Color, Style},
  symbols::merge::MergeStrategy,
  widgets::Block,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThemeName {
  Catpuccin,
}

#[derive(Clone)]
pub struct Theme {
  pub normal: Style,
  pub selected: Style,
  pub title: Style,
  pub border: Style,
}

impl Theme {
  fn new(name: ThemeName) -> Self {
    match name {
      ThemeName::Catpuccin => Self {
        normal: Style::default().fg(Color::White).bg(Color::Black),
        selected: Style::default().fg(Color::Black).bg(Color::Cyan),
        title: Style::default().fg(Color::White).bold(),
        border: Style::default().fg(Color::Blue),
      },
    }
  }

  pub fn block(&self) -> Block<'static> {
    Block::bordered()
      .merge_borders(MergeStrategy::Exact)
      .style(self.normal)
      .border_style(self.border)
      .title_style(self.title)
  }
}

pub fn get_current_theme() -> Theme {
  Theme::new(get_config().theme)
}
