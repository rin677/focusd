pub trait IgnoreType {
  fn ignore_type(self);
}

impl<T, E> IgnoreType for Result<T, E> {
  fn ignore_type(self) {
    let _ = self;
  }
}
