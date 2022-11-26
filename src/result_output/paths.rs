use std::collections::VecDeque;

use polygon_puzzle::shapes::{point::Point, segment::Segment};

pub struct Path {
  pub points: VecDeque<Point>,
}

impl Path {
  fn put(&mut self, s: &Segment) -> bool {
    self.put_empty(s) || self.put_back(s) || self.put_front(s)
  }

  fn put_back(&mut self, Segment { p, q }: &Segment) -> bool {
    let back = self.points.back().unwrap();

    if back == p {
      self.points.push_back(*q);
    } else if back == q {
      self.points.push_back(*p);
    } else {
      return false;
    }
    true
  }

  fn put_front(&mut self, Segment { p, q }: &Segment) -> bool {
    let front = self.points.front().unwrap();

    if front == p {
      self.points.push_front(*q);
    } else if front == q {
      self.points.push_front(*p);
    } else {
      return false;
    }

    true
  }

  fn put_empty(&mut self, Segment { p, q }: &Segment) -> bool {
    if self.points.is_empty() {
      self.points.push_back(*p);
      self.points.push_back(*q);
      true
    } else {
      false
    }
  }

  fn new() -> Self {
    Self {
      points: VecDeque::new(),
    }
  }
}

pub struct PathGroup {
  pub paths: Vec<Path>,
}

impl PathGroup {
  fn put(&mut self, s: &Segment) {
    for path in &mut self.paths {
      if path.put(s) {
        return;
      }
    }

    let mut new_path = Path::new();
    assert!(new_path.put(s));

    self.paths.push(new_path);
  }

  pub fn from_segments(segments: &[Segment]) -> Self {
    let mut paths = Self { paths: vec![] };

    for s in segments {
      paths.put(s);
    }

    paths
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn seg(x0: i32, y0: i32, x1: i32, y1: i32) -> Segment {
    Segment {
      p: Point {
        x: f64::from(x0),
        y: f64::from(y0),
      },
      q: Point {
        x: f64::from(x1),
        y: f64::from(y1),
      },
    }
  }

  #[test]
  fn test_path_put() {
    let mut path = Path::new();
    assert!(path.put(&seg(0, 0, 0, 1)));
    assert!(path.put(&seg(5, 5, 0, 1)));
    assert!(!path.put(&seg(7, 7, 8, 8)));
    assert!(path.put(&seg(0, 0, -2, -6)));
  }

  #[test]
  fn test_paths_put() {
    let mut group = PathGroup { paths: vec![] };
    assert_eq!(group.paths.len(), 0);

    group.put(&seg(0, 0, 0, 0));
    assert_eq!(group.paths.len(), 1);

    group.put(&seg(5, 5, 7, 8));
    assert_eq!(group.paths.len(), 2);

    group.put(&seg(100, 54, 7, 8));
    assert_eq!(group.paths.len(), 2);
  }
}
