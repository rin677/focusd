pub mod commands;
pub mod run;

pub const SOCKET_PATH: &str = if cfg!(feature = "dev-build") {
  "/tmp/focusd-dev.sock"
} else {
  "/tmp/focusd.sock"
};
