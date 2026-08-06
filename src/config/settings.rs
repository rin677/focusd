use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  env::{self},
  fs, io,
  path::PathBuf,
};
use toml::Value;
use toml_edit::{DocumentMut, Item, Table, value};

use crate::{
  config::themes::ThemeName,
  throw,
  utils::{ignore::IgnoreType, profile::Profile},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Fonts {
  AnsiRegular,
  AnsiShadow,
  DosRebel,
  Future,
  Mono12,
  Mono9,
  SmBlock,
  Terminus,
  TubesRegular,
}

/// Data type of config (which will be stored in `~/.config/focusd/config.toml`)
// TODO: Allow customizing fonts, hooks, gool and more
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  pub active_preset: String,
  pub presets: HashMap<String, Preset>,
  pub show_notifications: bool,
  pub daily_goal_minutes: u64,

  // TUI settings
  pub tui_show_progress: bool,
  pub tui_show_ascii_art: bool,
  pub tui_show_stats: bool,

  pub theme: ThemeName,
  pub font: Fonts,

  // Hooks
  pub hook_pause: String,
  pub hook_resume: String,
  pub hook_resume_short_break: String,
  pub hook_pause_short_break: String,
  pub hook_resume_work: String,
  pub hook_pause_work: String,
  pub hook_pause_long_break: String,
  pub hook_resume_long_break: String,
  pub hook_start_short_break: String,
  pub hook_start_long_break: String,
  pub hook_start_work: String,
}

/// Preset type for session
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Preset {
  pub work_minutes: u64,
  pub short_break_minutes: u64,
  pub long_break_minutes: u64,
  pub sessions_before_long_break: u64,
}

/// Main pomodoro preset
pub const POMODORO_PRESET: Preset = Preset {
  work_minutes: 25,
  short_break_minutes: 5,
  long_break_minutes: 15,
  sessions_before_long_break: 4,
};

impl Default for Config {
  fn default() -> Self {
    let mut presets: HashMap<String, Preset> = HashMap::new();
    presets.insert(String::from("pomodoro"), POMODORO_PRESET);
    presets.insert(
      String::from("deep_work"),
      Preset {
        work_minutes: 50,
        short_break_minutes: 10,
        long_break_minutes: 30,
        sessions_before_long_break: 4,
      },
    );
    Self {
      active_preset: "pomodoro".to_string(),
      presets,
      show_notifications: true,
      daily_goal_minutes: 0,
      tui_show_progress: true,
      tui_show_ascii_art: true,
      tui_show_stats: true,
      font: Fonts::Terminus,
      theme: ThemeName::Catpuccin,
      hook_pause_short_break: "".to_string(),
      hook_resume_short_break: "".to_string(),
      hook_pause_work: "".to_string(),
      hook_resume_work: "".to_string(),
      hook_pause_long_break: "".to_string(),
      hook_resume_long_break: "".to_string(),
      hook_start_short_break: "".to_string(),
      hook_start_long_break: "".to_string(),
      hook_start_work: "".to_string(),
      hook_pause: "".to_string(),
      hook_resume: "".to_string(),
    }
  }
}

fn config_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;

  let profile = Profile::current();
  let path = profile.config_path();
  Some(PathBuf::from(home).join(path))
}

fn get<T>(table: &toml::value::Table, key: &str, default: T) -> T
where
  T: serde::de::DeserializeOwned,
{
  table
    .get(key)
    .and_then(|v| T::deserialize(v.clone()).ok())
    .unwrap_or(default)
}

fn load_config(path: &PathBuf) -> io::Result<Config> {
  let contents = fs::read_to_string(path)?;
  let mut cfg = Config::default();
  let raw: Value =
    toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  let table = raw.as_table();
  if let Some(val) = table {
    cfg.active_preset = get(val, "active_preset", cfg.active_preset);
    cfg.presets = get(val, "presets", cfg.presets);
    cfg.tui_show_stats = get(val, "tui_show_stats", cfg.tui_show_stats);
    cfg.font = get(val, "font", cfg.font);
    cfg.theme = get(val, "theme", cfg.theme);
    cfg.tui_show_progress = get(val, "tui_show_progress", cfg.tui_show_progress);
    cfg.tui_show_ascii_art = get(val, "tui_show_ascii_art", cfg.tui_show_ascii_art);
    cfg.show_notifications = get(val, "show_notifications", cfg.show_notifications);
    cfg.daily_goal_minutes = get(val, "daily_goal_minutes", cfg.daily_goal_minutes);
    cfg.hook_pause_short_break = get(val, "hook_pause_short_break", cfg.hook_pause_short_break);
    cfg.hook_resume_short_break = get(val, "hook_resume_short_break", cfg.hook_resume_short_break);
    cfg.hook_pause_work = get(val, "hook_pause_work", cfg.hook_pause_work);
    cfg.hook_resume_work = get(val, "hook_resume_work", cfg.hook_resume_work);
    cfg.hook_pause_long_break = get(val, "hook_pause_long_break", cfg.hook_pause_long_break);
    cfg.hook_resume_long_break = get(val, "hook_resume_long_break", cfg.hook_resume_long_break);
    cfg.hook_start_short_break = get(val, "hook_start_short_break", cfg.hook_start_short_break);
    cfg.hook_start_long_break = get(val, "hook_start_long_break", cfg.hook_start_long_break);
    cfg.hook_start_work = get(val, "hook_start_work", cfg.hook_start_work);
    cfg.hook_pause = get(val, "hook_pause", cfg.hook_pause);
    cfg.hook_resume = get(val, "hook_resume", cfg.hook_resume);
  }

  Ok(cfg)
}

/// Returns the configuration
///
/// Create config files if it don't exists.
/// returns default config if config file is not found or in any case of errors.
pub fn get_config() -> Config {
  if let Some(path) = config_path() {
    if !path.exists() {
      create_config_file();
      return Config::default();
    }

    if let Ok(config) = load_config(&path) {
      return config;
    }
  }

  Config::default()
}

/// Returns active preset which is in use
/// Fallbacks to default in case of errors
pub fn get_crr_preset() -> Preset {
  let config = get_config();
  config
    .presets
    .get(&config.active_preset)
    .copied()
    .unwrap_or(POMODORO_PRESET)
}

/// Generates config files with default configuration
pub fn create_config_file() {
  let Some(path) = config_path() else { return };
  if let Some(parent) = path.parent() {
    let _ = fs::create_dir_all(parent);
  }
  if !path.exists() {
    let config = include_str!("../../examples/config.toml");
    fs::write(path, config).ok();
  }
}

/// Saves config file given the configuration
pub fn save_config(config: &Config) -> io::Result<()> {
  let Some(path) = config_path() else {
    throw!("file not found");
  };
  let toml_string = toml::to_string_pretty(config).expect("failed to serialize");
  fs::write(path, toml_string)
}

pub fn set_config_value<T>(key: &str, value: T) -> io::Result<()>
where
  T: Into<Item>,
{
  let Some(path) = config_path() else {
    throw!("file not found");
  };

  let contents = fs::read_to_string(&path)?;
  let mut doc = contents
    .parse::<DocumentMut>()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  doc[key] = value.into();
  fs::write(path, doc.to_string())
}

pub fn toggle_config_value(key: &str) -> io::Result<()> {
  let config = get_config();
  let raw: toml::Value = toml::Value::try_from(config).expect("failed to serialize config");
  let current = raw[key].as_bool().expect("Value is not boolean");
  set_config_value(key, value(!current))
}

/// Updates a single field of a preset in the config file.
pub fn set_preset_value(preset_name: &str, field: &str, new_value: i64) -> io::Result<()> {
  let Some(path) = config_path() else {
    throw!("file not found");
  };
  let contents = fs::read_to_string(&path)?;
  let mut doc = contents
    .parse::<DocumentMut>()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  let Some(presets) = doc["presets"].as_table_mut() else {
    throw!("presets not found");
  };
  let Some(preset) = presets.get_mut(preset_name) else {
    throw!("preset not found");
  };
  let Some(table) = preset.as_table_mut() else {
    throw!("preset is not a table");
  };
  table[field] = value(new_value);
  fs::write(path, doc.to_string())
}

/// Creates a new preset copying values from the active preset and activates it.
pub fn create_preset(name: &str) -> io::Result<()> {
  let config = get_config();
  if config.presets.contains_key(name) {
    return Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      "preset already exists",
    ));
  }
  let preset = get_crr_preset();
  let Some(path) = config_path() else {
    throw!("file not found");
  };
  let contents = fs::read_to_string(&path)?;
  let mut doc = contents
    .parse::<DocumentMut>()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  let Some(presets) = doc["presets"].as_table_mut() else {
    throw!("presets not found");
  };
  if presets.get(name).is_some() {
    return Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      "preset already exists",
    ));
  }
  let mut table = Table::new();
  table["work_minutes"] = value(preset.work_minutes as i64);
  table["short_break_minutes"] = value(preset.short_break_minutes as i64);
  table["long_break_minutes"] = value(preset.long_break_minutes as i64);
  table["sessions_before_long_break"] = value(preset.sessions_before_long_break as i64);
  presets.insert(name, Item::Table(table));
  fs::write(path, doc.to_string())?;
  set_config_value("active_preset", value(name))
}

/// Deletes a preset and switches active preset to another one if needed.
pub fn delete_preset(name: &str) -> io::Result<()> {
  let config = get_config();
  if config.presets.len() <= 1 {
    throw!("cannot delete the last preset");
  }
  let Some(path) = config_path() else {
    throw!("file not found");
  };
  let contents = fs::read_to_string(&path)?;
  let mut doc = contents
    .parse::<DocumentMut>()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  let Some(presets) = doc["presets"].as_table_mut() else {
    throw!("presets not found");
  };
  presets.remove(name);
  fs::write(path, doc.to_string())?;

  if config.active_preset == name {
    let mut names: Vec<&str> = config
      .presets
      .keys()
      .map(|s| s.as_str())
      .filter(|k| *k != name)
      .collect();
    names.sort();
    if let Some(next) = names.first() {
      set_config_value("active_preset", value(*next))?;
    }
  }
  Ok(())
}

pub fn delete_active_preset() {
  let name = get_config().active_preset.clone();
  delete_preset(&name).ignore_type();
}
