use crate::utils::profile::Profile;

pub mod commands;
pub mod run;

pub fn socket_path() -> &'static str {
  Profile::current().socket_path()
}
