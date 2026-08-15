use crate::config::settings::get_config;
use ratatui::{style::Style, symbols::merge::MergeStrategy, widgets::Block};
use ratatui_themes::{Theme, ThemePalette};

pub fn get_current_theme() -> ThemePalette {
  Theme::new(get_config().theme).palette()
}

pub fn themed_block() -> Block<'static> {
  let theme = get_current_theme();
  Block::bordered()
    .merge_borders(MergeStrategy::Exact)
    .style(Style::default().fg(theme.fg).bg(theme.bg))
    .border_style(theme.muted)
    .title_style(Style::default().bold().fg(theme.fg))
}
