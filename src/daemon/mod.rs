pub mod commands;
pub mod run;

use crate::utils::profile::Profile;

pub fn socket_path() -> &'static str {
  Profile::current().socket_path()
}
