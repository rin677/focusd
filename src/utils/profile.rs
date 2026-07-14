use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
  Dev,
  Prod,
}

impl Profile {
  pub fn current() -> Self {
    let exe = env::current_exe().unwrap_or_default();

    if exe.ends_with(std::path::Path::new("target/debug/focusd")) {
      Self::Dev
    } else {
      Self::Prod
    }
  }

  pub fn socket_path(self) -> &'static str {
    match self {
      Self::Dev => "/tmp/focusd-dev.sock",
      Self::Prod => "/tmp/focusd.sock",
    }
  }

  pub fn db_path(self) -> PathBuf {
    let home = env::var_os("HOME").unwrap();

    match self {
      Self::Dev => PathBuf::from(home).join(".local/share/focusd/db/history-test.db"),
      Self::Prod => PathBuf::from(home).join(".local/share/focusd/db/history.db"),
    }
  }

  pub fn config_path(self) -> PathBuf {
    let home = env::var_os("HOME").unwrap();

    match self {
      Self::Dev => PathBuf::from(home).join(".config/focusd/config-dev.toml"),
      Self::Prod => PathBuf::from(home).join(".config/focusd/config.toml"),
    }
  }
}
