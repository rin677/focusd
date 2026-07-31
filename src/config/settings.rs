use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  env::{self},
  fs, io,
  path::PathBuf,
};
use toml::Value;
use toml_edit::{DocumentMut, Item, value};

use crate::{throw, utils::profile::Profile};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

  pub font: Fonts,
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
    cfg.tui_show_progress = get(val, "tui_show_progress", cfg.tui_show_progress);
    cfg.tui_show_ascii_art = get(val, "tui_show_ascii_art", cfg.tui_show_ascii_art);
    cfg.show_notifications = get(val, "show_notifications", cfg.show_notifications);
    cfg.daily_goal_minutes = get(val, "daily_goal_minutes", cfg.daily_goal_minutes);
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
