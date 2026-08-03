use crate::{
  config::settings::{Fonts, get_config, set_config_value, toggle_config_value},
  utils::{ignore::IgnoreType, times_ago::render_duration},
};
use ratatui::{Frame, layout::Rect, prelude::*};
use toml_edit::value;
use tui_widget_list::{ListBuilder, ListState, ListView};

#[derive(Clone, Copy)]
pub enum SettingsItem {
  Preset,
  Notification,
  Tui,
  DailyGoal,
  Hooks,
  Sounds,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SettingsSubItems {
  TuiProgress,
  TuiAsciiArt,
  TuiStats,
  TuiFont,
  TuiTheme,

  SoundShortBreak,
  SoundLongBreak,
  SoundWork,

  HookPause,
  HookResume,
  HookPauseWork,
  HookResumeWork,
  HookResumeShortBreak,
  HookPauseShortBreak,
  HookResumeLongBreak,
  HookPauseLongBreak,
  HookStartShortBreak,
  HookStartLongBreak,
  HookStartWork,
}

pub struct SettingsPage {
  pub selected_setting_index: usize,
  pub settings_items: Vec<SettingsItem>,
  pub in_sub_menu: bool,
  pub sub_menu_index: usize,
}

pub enum AnySettingItem {
  Main(SettingsItem),
  Sub(SettingsSubItems),
}

impl Default for SettingsPage {
  fn default() -> Self {
    Self {
      selected_setting_index: 0,
      sub_menu_index: 0,
      in_sub_menu: false,
      settings_items: vec![
        SettingsItem::Preset,
        SettingsItem::Notification,
        SettingsItem::Tui,
        SettingsItem::DailyGoal,
        SettingsItem::Hooks,
        SettingsItem::Sounds,
      ],
    }
  }
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
    SettingsItem::Notification => "Notification".to_string(),
    SettingsItem::DailyGoal => "Daily Goal".to_string(),
    SettingsItem::Preset => "Preset".to_string(),
    SettingsItem::Hooks => "Hooks (Not implemented)".to_string(),
    SettingsItem::Sounds => "Sounds (Not Implemented)".to_string(),
    SettingsItem::Tui => "TUI".to_string(),
  }
}

fn value_for_settings_item(item: &SettingsItem) -> String {
  let config = get_config();
  match item {
    SettingsItem::Notification => enabled(config.show_notifications),
    SettingsItem::Tui => ">".to_string(),
    SettingsItem::DailyGoal => render_duration(config.daily_goal_minutes * 60),
    _ => "Not implemented".to_string(),
  }
}

fn name_for_settings_sub_item(item: &SettingsSubItems) -> String {
  match item {
    SettingsSubItems::TuiFont => "Font".to_string(),
    SettingsSubItems::TuiStats => "TUI show Stats".to_string(),
    SettingsSubItems::TuiProgress => "TUI show progress bar".to_string(),
    SettingsSubItems::TuiAsciiArt => "TUI show ASCII Art".to_string(),
    SettingsSubItems::TuiTheme => "TUI Theme".to_string(),

    SettingsSubItems::SoundWork => "Sound Work".to_string(),
    SettingsSubItems::SoundShortBreak => "Sound Short Break".to_string(),
    SettingsSubItems::SoundLongBreak => "Sound Long Break".to_string(),

    SettingsSubItems::HookPause => "Hook Pause".to_string(),
    SettingsSubItems::HookResume => "Hook Resume".to_string(),
    SettingsSubItems::HookPauseWork => "Hook pause work".to_string(),
    SettingsSubItems::HookResumeWork => "Hook resume work".to_string(),
    SettingsSubItems::HookPauseLongBreak => "Hook resume short break".to_string(),
    SettingsSubItems::HookPauseShortBreak => "Hook pause short break".to_string(),
    SettingsSubItems::HookResumeLongBreak => "Hook resume long break".to_string(),
    SettingsSubItems::HookResumeShortBreak => "Hook resume short break".to_string(),
    SettingsSubItems::HookStartShortBreak => "Hook start short break".to_string(),
    SettingsSubItems::HookStartLongBreak => "Hook start long break".to_string(),
    SettingsSubItems::HookStartWork => "Hook start work".to_string(),
  }
}

fn value_for_settings_sub_item(item: &SettingsSubItems) -> String {
  let config = get_config();
  match item {
    SettingsSubItems::TuiStats => enabled(config.tui_show_stats),
    SettingsSubItems::TuiProgress => enabled(config.tui_show_progress),
    SettingsSubItems::TuiAsciiArt => enabled(config.tui_show_ascii_art),
    SettingsSubItems::TuiFont => format!("{:?}", config.font),
    _ => "Not Implemented".to_string(),
  }
}

fn get_sub_items(item: &SettingsItem) -> Vec<SettingsSubItems> {
  match item {
    SettingsItem::Tui => vec![
      SettingsSubItems::TuiProgress,
      SettingsSubItems::TuiAsciiArt,
      SettingsSubItems::TuiStats,
      SettingsSubItems::TuiFont,
      SettingsSubItems::TuiTheme,
    ],
    SettingsItem::Sounds => vec![
      SettingsSubItems::SoundWork,
      SettingsSubItems::SoundShortBreak,
      SettingsSubItems::SoundLongBreak,
    ],
    SettingsItem::Hooks => vec![
      SettingsSubItems::HookPause,
      SettingsSubItems::HookResume,
      SettingsSubItems::HookPauseWork,
      SettingsSubItems::HookResumeWork,
      SettingsSubItems::HookPauseLongBreak,
      SettingsSubItems::HookPauseShortBreak,
      SettingsSubItems::HookResumeLongBreak,
      SettingsSubItems::HookResumeShortBreak,
      SettingsSubItems::HookStartShortBreak,
      SettingsSubItems::HookStartLongBreak,
      SettingsSubItems::HookStartWork,
    ],
    _ => vec![],
  }
}

fn increase_goal() {
  let goal = get_config().daily_goal_minutes;
  let _ = set_config_value("daily_goal_minutes", value((goal + 5) as i64));
}
fn decrease_goal() {
  let goal = get_config().daily_goal_minutes;
  let _ = set_config_value("daily_goal_minutes", value(goal.saturating_sub(5) as i64));
}

impl SettingsPage {
  pub fn render(&self, area: Rect, frame: &mut Frame) {
    let items = self.get_items_to_render();

    let builder = ListBuilder::new(|context| {
      let text = match items.get(context.index) {
        Some(item) => match item {
          AnySettingItem::Main(item) => format!(
            "{:<30}{}",
            name_for_settings_item(item),
            value_for_settings_item(item)
          ),
          AnySettingItem::Sub(item) => format!(
            "{:<30}{}",
            name_for_settings_sub_item(item),
            value_for_settings_sub_item(item)
          ),
        },
        None => String::new(),
      };
      let mut item = Line::from(text);
      if context.is_selected {
        item = item.style(Style::default().bg(Color::DarkGray));
      }
      (item, 1)
    });

    let mut state = ListState::default();
    let selected_index = if self.in_sub_menu {
      self.sub_menu_index
    } else {
      self.selected_setting_index
    };
    state.select(Some(selected_index));
    let list = ListView::new(builder, items.len());
    list.render(area, frame.buffer_mut(), &mut state);
  }

  pub fn get_items_to_render(&self) -> Vec<AnySettingItem> {
    if self.in_sub_menu {
      let crr = self.get_selected_item_main();
      let items = get_sub_items(&crr);
      items.iter().map(|i| AnySettingItem::Sub(*i)).collect()
    } else {
      self
        .settings_items
        .iter()
        .map(|i| AnySettingItem::Main(*i))
        .collect()
    }
  }

  pub fn handle_right(&mut self) {
    if self.in_sub_menu {
      let crr_item = self.get_selected_item_sub();
      match crr_item {
        SettingsSubItems::TuiProgress => toggle_config_value("tui_show_progress").ignore_type(),
        SettingsSubItems::TuiAsciiArt => toggle_config_value("tui_show_ascii_art").ignore_type(),
        SettingsSubItems::TuiStats => toggle_config_value("tui_show_stats").ignore_type(),
        SettingsSubItems::TuiFont => {
          let selected_font = get_config().font;
          let all_fonts: Vec<Fonts> = vec![
            Fonts::AnsiRegular,
            Fonts::AnsiShadow,
            Fonts::DosRebel,
            Fonts::Future,
            Fonts::Mono12,
            Fonts::Mono9,
            Fonts::SmBlock,
            Fonts::Terminus,
            Fonts::TubesRegular,
          ];

          if let Some(index) = all_fonts.iter().position(|f| *f == selected_font) {
            let next_font = if index == all_fonts.len() - 1 {
              &all_fonts[0]
            } else {
              &all_fonts[index + 1]
            };
            let f = format!("{:?}", next_font);
            set_config_value("font", value(f)).ignore_type();
          }
        }
        _ => {}
      }
      return;
    }
    let crr_item = self.get_selected_item_main();
    let sub_items = get_sub_items(&crr_item);
    if !sub_items.is_empty() {
      self.in_sub_menu = true;
      self.sub_menu_index = 0;
      return;
    }
    match crr_item {
      SettingsItem::Notification => toggle_config_value("show_notifications").ignore_type(),
      SettingsItem::DailyGoal => increase_goal(),
      _ => {}
    }
  }

  pub fn handle_left(&mut self) {
    if self.in_sub_menu {
      self.in_sub_menu = false
    } else {
      let crr_item = self.get_selected_item_main();
      match crr_item {
        SettingsItem::Notification => toggle_config_value("show_notifications").ignore_type(),
        SettingsItem::DailyGoal => decrease_goal(),
        _ => {}
      }
    }
  }

  fn get_selected_item_main(&self) -> SettingsItem {
    self.settings_items[self.selected_setting_index]
  }
  fn get_selected_item_sub(&self) -> SettingsSubItems {
    let main = self.get_selected_item_main();
    let items = get_sub_items(&main);
    items[self.sub_menu_index]
  }

  pub fn up(&mut self) {
    if self.in_sub_menu {
      if self.sub_menu_index > 0 {
        self.sub_menu_index -= 1;
      }
    } else {
      if self.selected_setting_index > 0 {
        self.selected_setting_index -= 1;
      }
    }
  }
  pub fn down(&mut self) {
    let count = self.get_items_to_render().len() - 1;
    if self.in_sub_menu {
      if self.sub_menu_index < count {
        self.sub_menu_index += 1;
      }
    } else {
      if self.selected_setting_index < count {
        self.selected_setting_index += 1;
      }
    }
  }
}
