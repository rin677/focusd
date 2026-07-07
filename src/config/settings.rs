use serde::{Deserialize, Serialize};
use std::{
  env::{self},
  fs, io,
  path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  notification: bool,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      notification: false,
    }
  }
}

fn config_path() -> Option<PathBuf> {
  let home = env::var_os("HOME")?;
  Some(PathBuf::from(home).join(".config/focusd/config.toml"))
}

fn load_config() -> io::Result<Config> {
  let path =
    config_path().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME not found"))?;

  let contents = fs::read_to_string(&path)?;
  let config: Config =
    toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

  Ok(config)
}

fn get_config() -> Config {
  if let Some(path) = config_path() {
    if let Some(parent) = path.parent() {
      let _ = fs::create_dir_all(parent);
    }

    if !path.exists() {
      let config = Config::default();
      let _ = save_config(&path, &config);
      return config;
    }

    if let Ok(contents) = fs::read_to_string(&path)
      && let Ok(config) = toml::from_str::<Config>(&contents)
    {
      return config;
    }
  }

  Config::default()
}

fn save_config(path: &PathBuf, config: &Config) -> io::Result<()> {
  let toml_string = toml::to_string_pretty(config).expect("failed to serialize");
  fs::write(path, toml_string)
}
