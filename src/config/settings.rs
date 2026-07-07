use serde::{Deserialize, Serialize};
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub num_session: u64,
  pub short_break_duratoin: u64,
  pub long_break_duration: u64,
  pub work_duration: u64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      num_session: 4,
      short_break_duratoin: 5,
      long_break_duration: 15,
      work_duration: 25,
    }
  }
}

fn config_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;
  Some(PathBuf::from(home).join(".config/focusd/config.toml"))
}

fn load_config(path : &PathBuf) -> io::Result<Config> {
  let contents = fs::read_to_string(&path)?;
  let config: Config =
    toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  Ok(config)
}

pub fn get_config() -> Config {
  if let Some(path) = config_path() {
    if !path.exists() {
      create_config_file();
      return Config::default();
    }

      let Ok(config) = load_config(&path)
    {
      return config;
    }
  }

  Config::default()
}

pub fn create_config_file() {
  let Some(path) = config_path() else { return };
  if let Some(parent) = path.parent() {
    let _ = fs::create_dir_all(parent);
  }
  if !path.exists() {
    let config = Config::default();
    let _ = save_config(&path, &config);
  }
}

pub fn save_config(path: &PathBuf, config: &Config) -> io::Result<()> {
  let toml_string = toml::to_string_pretty(config).expect("failed to serialize");
  fs::write(path, toml_string)
}
