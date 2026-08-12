use crate::{
  config::{
    settings::{
      Fonts, create_preset, delete_active_preset, get_config, get_crr_preset, set_config_value,
      set_preset_value, toggle_config_value,
    },
    themes::{get_current_theme, themed_block},
  },
  utils::{
    cycle::{cycle_index, cycle_item},
    ignore::IgnoreType,
    times_ago::render_duration,
  },
};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, prelude::*, widgets::Paragraph};
use toml_edit::value;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsSubItems {
  TuiProgress,
  TuiAsciiArt,
  TuiStats,
  TuiFont,
  TuiTheme,

  PresetActive,
  PresetWork,
  PresetShortBreak,
  PresetLongBreak,
  PresetSessions,
  PresetNew,
  PresetDelete,

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
  pub input: Input,
  pub input_mode: InputMode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
  #[default]
  Normal,
  Editing,
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
      input: Input::default(),
      input_mode: InputMode::Normal,
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
    SettingsItem::Hooks => "Hooks".to_string(),
    SettingsItem::Sounds => "Sounds".to_string(),
    SettingsItem::Tui => "TUI".to_string(),
  }
}

fn value_for_settings_item(item: &SettingsItem) -> String {
  let config = get_config();
  match item {
    SettingsItem::Notification => enabled(config.show_notifications),
    SettingsItem::Tui | SettingsItem::Hooks => ">".to_string(),
    SettingsItem::Preset => config.active_preset,
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

    SettingsSubItems::PresetActive => "Active Preset".to_string(),
    SettingsSubItems::PresetWork => "Work".to_string(),
    SettingsSubItems::PresetShortBreak => "Short Break".to_string(),
    SettingsSubItems::PresetLongBreak => "Long Break".to_string(),
    SettingsSubItems::PresetSessions => "Sessions Before Long Break".to_string(),
    SettingsSubItems::PresetNew => "New Preset".to_string(),
    SettingsSubItems::PresetDelete => "Delete Preset".to_string(),

    SettingsSubItems::SoundWork => "Sound Work".to_string(),
    SettingsSubItems::SoundShortBreak => "Sound Short Break".to_string(),
    SettingsSubItems::SoundLongBreak => "Sound Long Break".to_string(),

    SettingsSubItems::HookPause => "Hook Pause".to_string(),
    SettingsSubItems::HookResume => "Hook Resume".to_string(),
    SettingsSubItems::HookPauseWork => "Hook Pause Work".to_string(),
    SettingsSubItems::HookResumeWork => "Hook Resume Work".to_string(),
    SettingsSubItems::HookPauseLongBreak => "Hook Pause Long Break".to_string(),
    SettingsSubItems::HookPauseShortBreak => "Hook Pause Short Break".to_string(),
    SettingsSubItems::HookResumeLongBreak => "Hook Resume Long Break".to_string(),
    SettingsSubItems::HookResumeShortBreak => "Hook Resume Short Break".to_string(),
    SettingsSubItems::HookStartShortBreak => "Hook Start Short Break".to_string(),
    SettingsSubItems::HookStartLongBreak => "Hook Start Long Break".to_string(),
    SettingsSubItems::HookStartWork => "Hook Start Work".to_string(),
  }
}

fn value_for_settings_sub_item(item: &SettingsSubItems) -> String {
  let config = get_config();
  let preset = get_crr_preset();
  match item {
    SettingsSubItems::PresetActive => config.active_preset,
    SettingsSubItems::PresetWork => render_duration(preset.work_minutes * 60),
    SettingsSubItems::PresetShortBreak => render_duration(preset.short_break_minutes * 60),
    SettingsSubItems::PresetLongBreak => render_duration(preset.long_break_minutes * 60),
    SettingsSubItems::PresetSessions => preset.sessions_before_long_break.to_string(),
    SettingsSubItems::PresetNew => ">".to_string(),
    SettingsSubItems::PresetDelete => config.active_preset,

    SettingsSubItems::TuiStats => enabled(config.tui_show_stats),
    SettingsSubItems::TuiProgress => enabled(config.tui_show_progress),
    SettingsSubItems::TuiAsciiArt => enabled(config.tui_show_ascii_art),
    SettingsSubItems::TuiFont => format!("{:?}", config.font),
    SettingsSubItems::TuiTheme => config.theme.slug().to_string(),

    SettingsSubItems::HookPause => config.hook_pause,
    SettingsSubItems::HookResume => config.hook_resume,
    SettingsSubItems::HookPauseWork => config.hook_pause_work,
    SettingsSubItems::HookResumeWork => config.hook_resume_work,
    SettingsSubItems::HookPauseLongBreak => config.hook_pause_long_break,
    SettingsSubItems::HookResumeLongBreak => config.hook_resume_long_break,
    SettingsSubItems::HookPauseShortBreak => config.hook_pause_short_break,
    SettingsSubItems::HookResumeShortBreak => config.hook_resume_short_break,
    SettingsSubItems::HookStartShortBreak => config.hook_start_short_break,
    SettingsSubItems::HookStartLongBreak => config.hook_start_long_break,
    SettingsSubItems::HookStartWork => config.hook_start_work,
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
    SettingsItem::Preset => vec![
      SettingsSubItems::PresetActive,
      SettingsSubItems::PresetWork,
      SettingsSubItems::PresetShortBreak,
      SettingsSubItems::PresetLongBreak,
      SettingsSubItems::PresetSessions,
      SettingsSubItems::PresetNew,
      SettingsSubItems::PresetDelete,
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

fn hook_config_key(item: &SettingsSubItems) -> Option<&'static str> {
  match item {
    SettingsSubItems::HookPause => Some("hook_pause"),
    SettingsSubItems::HookResume => Some("hook_resume"),
    SettingsSubItems::HookPauseWork => Some("hook_pause_work"),
    SettingsSubItems::HookResumeWork => Some("hook_resume_work"),
    SettingsSubItems::HookPauseLongBreak => Some("hook_pause_long_break"),
    SettingsSubItems::HookPauseShortBreak => Some("hook_pause_short_break"),
    SettingsSubItems::HookResumeLongBreak => Some("hook_resume_long_break"),
    SettingsSubItems::HookResumeShortBreak => Some("hook_resume_short_break"),
    SettingsSubItems::HookStartShortBreak => Some("hook_start_short_break"),
    SettingsSubItems::HookStartLongBreak => Some("hook_start_long_break"),
    SettingsSubItems::HookStartWork => Some("hook_start_work"),
    _ => None,
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

fn cycle_active_preset(offset: isize) {
  let config = get_config();
  let names: Vec<&str> = config.presets.keys().map(|s| s.as_str()).collect();
  let next_item = cycle_item(config.active_preset.as_str(), &names, offset);
  set_config_value("active_preset", value(next_item)).ignore_type();
}

fn change_preset_value(field: &str, step: i64) {
  let config = get_config();
  let preset_name = config.active_preset.clone();
  let Some(preset) = config.presets.get(&preset_name).copied() else {
    return;
  };
  let current = match field {
    "work_minutes" => preset.work_minutes as i64,
    "short_break_minutes" => preset.short_break_minutes as i64,
    "long_break_minutes" => preset.long_break_minutes as i64,
    "sessions_before_long_break" => preset.sessions_before_long_break as i64,
    _ => return,
  };
  set_preset_value(&preset_name, field, (current + step).max(0)).ignore_type();
}

impl SettingsPage {
  pub fn render(&self, area: Rect, frame: &mut Frame) {
    let items = self.get_items_to_render();
    let theme = get_current_theme();

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
        item = item.style(Style::default().bg(theme.selection));
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

    if self.is_editing() {
      let [list_area, input_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(area);
      list.render(list_area, frame.buffer_mut(), &mut state);
      self.render_input(input_area, frame);
    } else {
      list.render(area, frame.buffer_mut(), &mut state);
    }
  }

  fn render_input(&self, area: Rect, frame: &mut Frame) {
    let width = area.width.max(3) - 3;
    let scroll = self.input.visual_scroll(width as usize);
    let title = if matches!(self.get_selected_item_sub(), SettingsSubItems::PresetNew) {
      " Preset name (Enter: save, Esc: cancel) "
    } else {
      " Shell command (Enter: save, Esc: cancel) "
    };
    let input = Paragraph::new(self.input.value())
      .scroll((0, scroll as u16))
      .block(themed_block().title(title));
    frame.render_widget(input, area);
    let x = self.input.visual_cursor().max(scroll) - scroll + 1;
    frame.set_cursor_position((area.x + x as u16, area.y + 1));
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
        SettingsSubItems::TuiTheme => {
          let next_theme = get_config().theme.next();
          set_config_value("theme", value(next_theme.slug())).ignore_type();
        }
        SettingsSubItems::PresetActive => cycle_active_preset(1),
        SettingsSubItems::PresetWork => change_preset_value("work_minutes", 5),
        SettingsSubItems::PresetShortBreak => change_preset_value("short_break_minutes", 5),
        SettingsSubItems::PresetLongBreak => change_preset_value("long_break_minutes", 5),
        SettingsSubItems::PresetSessions => change_preset_value("sessions_before_long_break", 1),
        SettingsSubItems::PresetNew => self.start_editing(),
        SettingsSubItems::PresetDelete => delete_active_preset(),
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
          let next_font = cycle_item(selected_font, &all_fonts, 1);
          let f = format!("{:?}", next_font);
          set_config_value("font", value(f)).ignore_type();
        }
        _ => {
          if hook_config_key(&crr_item).is_some() {
            self.start_editing();
          }
        }
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
      let crr_item = self.get_selected_item_sub();
      match crr_item {
        SettingsSubItems::PresetActive => cycle_active_preset(-1),
        SettingsSubItems::PresetWork => change_preset_value("work_minutes", -5),
        SettingsSubItems::PresetShortBreak => change_preset_value("short_break_minutes", -5),
        SettingsSubItems::PresetLongBreak => change_preset_value("long_break_minutes", -5),
        SettingsSubItems::PresetSessions => change_preset_value("sessions_before_long_break", -1),
        _ => self.in_sub_menu = false,
      }
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

  pub fn is_editing(&self) -> bool {
    self.input_mode == InputMode::Editing
  }
  pub fn start_editing(&mut self) {
    if self.in_sub_menu {
      let item = self.get_selected_item_sub();
      if hook_config_key(&item).is_some() {
        self.input = Input::new(value_for_settings_sub_item(&item));
      } else if matches!(item, SettingsSubItems::PresetNew) {
        self.input = Input::default();
      }
    }
    self.input_mode = InputMode::Editing
  }
  pub fn stop_editing(&mut self) {
    self.input_mode = InputMode::Normal
  }

  pub fn exit_sub_menu(&mut self) {
    self.in_sub_menu = false;
  }

  pub fn footer_hint(&self) -> String {
    if self.is_editing() {
      return "Enter save · Esc cancel".to_string();
    }
    if !self.in_sub_menu {
      match self.get_selected_item_main() {
        SettingsItem::Notification => "←/→ toggle".to_string(),
        SettingsItem::DailyGoal => "←/→ adjust".to_string(),
        SettingsItem::Preset | SettingsItem::Tui | SettingsItem::Hooks | SettingsItem::Sounds => {
          "→ open".to_string()
        }
      }
    } else {
      match self.get_selected_item_sub() {
        SettingsSubItems::TuiProgress
        | SettingsSubItems::TuiAsciiArt
        | SettingsSubItems::TuiStats => "→ toggle · ← back".to_string(),
        SettingsSubItems::TuiFont | SettingsSubItems::TuiTheme => "→ cycle · ← back".to_string(),
        SettingsSubItems::PresetActive => "←/→ switch · Esc back".to_string(),
        SettingsSubItems::PresetWork
        | SettingsSubItems::PresetShortBreak
        | SettingsSubItems::PresetLongBreak
        | SettingsSubItems::PresetSessions => "←/→ adjust · Esc back".to_string(),
        SettingsSubItems::PresetNew => "→ create · ← back".to_string(),
        SettingsSubItems::PresetDelete => "→ delete · ← back".to_string(),
        SettingsSubItems::HookPause
        | SettingsSubItems::HookResume
        | SettingsSubItems::HookPauseWork
        | SettingsSubItems::HookResumeWork
        | SettingsSubItems::HookResumeShortBreak
        | SettingsSubItems::HookPauseShortBreak
        | SettingsSubItems::HookResumeLongBreak
        | SettingsSubItems::HookPauseLongBreak
        | SettingsSubItems::HookStartShortBreak
        | SettingsSubItems::HookStartLongBreak
        | SettingsSubItems::HookStartWork => "→ edit · ← back".to_string(),
        _ => "Not implemented".to_string(),
      }
    }
  }

  pub fn handle_editing_key(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Enter => {
        if self.in_sub_menu {
          let item = self.get_selected_item_sub();
          if SettingsSubItems::PresetNew == item {
            let name = self.input.value().trim().to_string();
            if !name.is_empty() {
              create_preset(&name).ignore_type();
            }
          } else {
            if let Some(key) = hook_config_key(&item) {
              set_config_value(key, value(self.input.value().to_string())).ignore_type();
            }
          }
        }
        self.stop_editing();
      }
      KeyCode::Esc => self.stop_editing(),
      _ => {
        self.input.handle_event(&Event::Key(key_event));
      }
    }
  }

  pub fn up(&mut self) {
    self.up_or_down(-1)
  }
  pub fn down(&mut self) {
    self.up_or_down(1)
  }

  pub fn up_or_down(&mut self, offset: isize) {
    let count = self.get_items_to_render().len();
    let index = if self.in_sub_menu {
      self.sub_menu_index
    } else {
      self.selected_setting_index
    };
    let next_index = cycle_index(index, count, offset);
    if self.in_sub_menu {
      self.sub_menu_index = next_index;
    } else {
      self.selected_setting_index = next_index
    }
  }
}
