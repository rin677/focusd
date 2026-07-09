pub trait Ignore {
  fn ignore(self);
}

impl<T, E> Ignore for Result<T, E> {
  fn ignore(self) {
    let _ = self;
  }
}
