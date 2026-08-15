use std::io::Result;

pub trait Print {
  fn print(self) -> Result<()>;
}

impl Print for Result<String> {
  fn print(self) -> Result<()> {
    match self {
      Ok(value) => {
        println!("{value}");
        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}
