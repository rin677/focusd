use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  env::{self},
  fs, io,
  path::PathBuf,
};

use crate::{throw, utils::profile::Profile};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Preset {
  pub work_minutes: u64,
  pub short_break_minutes: u64,
  pub long_break_minutes: u64,
  pub sessions_before_long_break: u64,
}

pub const POMODORO_PRESET: Preset = Preset {
  work_minutes: 25,
  short_break_minutes: 5,
  long_break_minutes: 15,
  sessions_before_long_break: 4,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub active_preset: String,
  pub presets: HashMap<String, Preset>,
  pub show_notifications: bool,
}

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
    }
  }
}

fn config_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;

  let profile = Profile::current();
  let path = profile.config_path();
  Some(PathBuf::from(home).join(path))
}

fn load_config(path: &PathBuf) -> io::Result<Config> {
  let contents = fs::read_to_string(path)?;
  toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn get_config() -> Config {
  if let Some(path) = config_path() {
    if !path.exists() {
      create_config_file();
      return Config::default();
    }

    // FIX: Gives default config if something is wrong in config file Ignoring all the configs
    if let Ok(config) = load_config(&path) {
      return config;
    }
  }

  Config::default()
}

pub fn get_crr_preset() -> Preset {
  let config = get_config();
  config
    .presets
    .get(&config.active_preset)
    .copied()
    .unwrap_or(POMODORO_PRESET)
}

pub fn create_config_file() {
  let Some(path) = config_path() else { return };
  // TODO: create better config with commnet explaining what each key does
  if let Some(parent) = path.parent() {
    let _ = fs::create_dir_all(parent);
  }
  if !path.exists() {
    let config = Config::default();
    let _ = save_config(&config);
  }
}

pub fn save_config(config: &Config) -> io::Result<()> {
  let Some(path) = config_path() else {
    throw!("file not found");
  };
  let toml_string = toml::to_string_pretty(config).expect("failed to serialize");
  fs::write(path, toml_string)
}
