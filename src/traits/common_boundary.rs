pub trait CommonBoundary<T> {
  fn common_boundary(&self, other: &Self) -> T;
}
