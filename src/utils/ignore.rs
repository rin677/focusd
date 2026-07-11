pub trait IgnoreType {
  fn ignore_type(self);
}

impl<T, E> IgnoreType for Result<T, E> {
  fn ignore_type(self) {
    let _ = self;
  }
}

pub trait Ignore {
  type Ok;
  fn ignore(self) -> Self::Ok;
}

impl<T, E> Ignore for Result<T, E> {
  type Ok = T;

  fn ignore(self) -> T {
    match self {
      Ok(value) => value,
      Err(_err) => panic!("ignored error"),
    }
  }
}
