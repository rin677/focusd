use crate::{
  config::settings::{get_config, toggle_config_value},
  utils::ignore::IgnoreType,
};
use ratatui::{Frame, layout::Rect, prelude::*};
use tui_widget_list::{ListBuilder, ListState, ListView};

pub enum SettingsItem {
  Theme,
  Notification,
  TuiProgress,
  TuiStats,
  TuiAsciiArt,
  Font,
  DailyGoal,
  Preset,
  Hooks,
  Sounds,
}

fn enabled(val: bool) -> String {
  if val {
    "Enabled".to_string()
  } else {
    "Not enabled".to_string()
  }
}

fn name_for_settings_item(item: &SettingsItem) -> String {
  match item {
    SettingsItem::Theme => "Theme (not implemented)".to_string(),
    SettingsItem::Notification => {
      let config = get_config();
      format!("Notification: {}", enabled(config.show_notifications))
    }
    SettingsItem::Font => "Font".to_string(),
    SettingsItem::DailyGoal => "Daily Goal".to_string(),
    SettingsItem::Preset => "Preset".to_string(),
    SettingsItem::Hooks => "Hooks (Not implemented)".to_string(),
    SettingsItem::Sounds => "Sounds (Not Implemented)".to_string(),
    SettingsItem::TuiStats => {
      let config = get_config();
      format!("TUI show stats: {}", enabled(config.tui_show_stats))
    }
    SettingsItem::TuiProgress => {
      let config = get_config();
      format!("TUI Progress bar: {}", enabled(config.tui_show_progress))
    }
    SettingsItem::TuiAsciiArt => {
      let config = get_config();
      format!("TUI show ascii art: {}", enabled(config.tui_show_ascii_art))
    }
  }
}

pub struct SettingsPage {
  pub selected_setting_index: usize,
  pub settings_items: Vec<SettingsItem>,
}

impl Default for SettingsPage {
  fn default() -> Self {
    Self {
      selected_setting_index: 0,
      settings_items: vec![
        SettingsItem::Theme,
        SettingsItem::Notification,
        SettingsItem::Font,
        SettingsItem::DailyGoal,
        SettingsItem::Preset,
        SettingsItem::Hooks,
        SettingsItem::Sounds,
        SettingsItem::TuiProgress,
        SettingsItem::TuiAsciiArt,
        SettingsItem::TuiStats,
      ],
    }
  }
}

impl SettingsPage {
  pub fn render(&self, area: Rect, frame: &mut Frame) {
    let builder = ListBuilder::new(|context| {
      let text = match self.settings_items.get(context.index) {
        Some(item) => name_for_settings_item(item),
        None => String::new(),
      };
      let mut item = Line::from(text);
      if context.is_selected {
        item = item.style(Style::default().bg(Color::DarkGray));
      }
      (item, 1)
    });

    let mut state = ListState::default();
    state.select(Some(self.selected_setting_index));
    let list = ListView::new(builder, self.settings_items.len());
    list.render(area, frame.buffer_mut(), &mut state);
  }

  pub fn handle_right(&self) {
    let crr_item = self.get_selected_item();
    match crr_item {
      SettingsItem::Notification => toggle_config_value("show_notifications").ignore_type(),
      SettingsItem::TuiProgress => toggle_config_value("tui_show_progress").ignore_type(),
      SettingsItem::TuiAsciiArt => toggle_config_value("tui_show_ascii_art").ignore_type(),
      SettingsItem::TuiStats => toggle_config_value("tui_show_stats").ignore_type(),

      _ => {}
    }
  }
  fn get_selected_item(&self) -> &SettingsItem {
    self
      .settings_items
      .get(self.selected_setting_index)
      .unwrap()
  }
  pub fn up(&mut self) {
    if self.selected_setting_index > 0 {
      self.selected_setting_index -= 1;
    }
  }
  pub fn down(&mut self) {
    let count = self.settings_items.len() - 1;
    if self.selected_setting_index < count {
      self.selected_setting_index += 1;
    }
  }
}
