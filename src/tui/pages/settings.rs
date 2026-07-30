use ratatui::{Frame, layout::Rect, prelude::*};
use tui_widget_list::{ListBuilder, ListState, ListView};

pub enum SettingsItem {
  Theme,
  Notification,
  Font,
  DailyGoal,
  Preset,
  Hooks,
  Sounds,
}

fn name_for_settings_item(item: &SettingsItem) -> String {
  match item {
    SettingsItem::Theme => "Theme".to_string(),
    SettingsItem::Notification => "Notification".to_string(),
    SettingsItem::Font => "Font".to_string(),
    SettingsItem::DailyGoal => "DailyGoal".to_string(),
    SettingsItem::Preset => "Preset".to_string(),
    SettingsItem::Hooks => "Hooks".to_string(),
    SettingsItem::Sounds => "Sounds".to_string(),
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
