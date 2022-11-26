pub struct AlternateIterator {
  curr: i32,
  inc: i32,
  sign: i32,
}
impl AlternateIterator {
  pub const fn new() -> Self {
    Self {
      curr: 0,
      inc: 0,
      sign: 1,
    }
  }
}
impl Iterator for AlternateIterator {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    self.sign *= -1;
    self.curr += self.sign * self.inc;
    self.inc += 1;

    Some(self.curr)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_alternate_iterator() {
    let mut it = AlternateIterator::new();
    assert_eq!(it.next().unwrap(), 0);
    assert_eq!(it.next().unwrap(), 1);
    assert_eq!(it.next().unwrap(), -1);
    assert_eq!(it.next().unwrap(), 2);
    assert_eq!(it.next().unwrap(), -2);
    assert_eq!(it.next().unwrap(), 3);
    assert_eq!(it.next().unwrap(), -3);
  }

  #[test]
  fn test_alternate_iterator_collect() {
    let vec: Vec<i32> = AlternateIterator::new().take(10).collect();
    assert_eq!(vec, vec![0, 1, -1, 2, -2, 3, -3, 4, -4, 5]);
  }
}
