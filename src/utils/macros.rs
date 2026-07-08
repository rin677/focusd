#[macro_export]
macro_rules! throw {
  ($msg:expr) => {
    return Err(std::io::Error::new(std::io::ErrorKind::Other, $msg))
  };
}
