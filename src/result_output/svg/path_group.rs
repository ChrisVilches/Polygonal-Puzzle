use polygon_puzzle::shapes::{point::Point, segment::Segment};
use std::collections::VecDeque;

fn try_merge_back<T: PartialEq + Copy>(dest: &mut VecDeque<T>, src: &VecDeque<T>) -> bool {
  let src_front = *src.front().unwrap();
  let src_back = *src.back().unwrap();
  let back = *dest.back().unwrap();

  let prev_len = dest.len();

  if back == src_front {
    dest.extend(src.iter().skip(1));
  } else if back == src_back {
    dest.extend(src.iter().rev().skip(1));
  }

  prev_len != dest.len()
}

pub struct PathGroup {
  pub paths: Vec<VecDeque<Point>>,
}

impl PathGroup {
  fn put(&mut self, s: &Segment) {
    let new_path = VecDeque::<Point>::from_iter([s.p, s.q]);
    self.paths.push(new_path);
    self.merge_once();
    self.merge_once();
  }

  fn merge_find_index(&mut self) -> Option<usize> {
    for i in 0..self.paths.len() {
      for j in 0..self.paths.len() {
        if i == j {
          continue;
        }

        let src = self.paths[j].clone();
        if try_merge_back(&mut self.paths[i], &src) {
          return Some(j);
        }
      }
    }
    None
  }

  fn merge_once(&mut self) {
    let removed_idx = self.merge_find_index();

    self.paths = (0..self.paths.len())
      .filter(|idx| Some(*idx) != removed_idx)
      .map(|idx| self.paths[idx].clone())
      .collect();
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
    let p = Point {
      x: f64::from(x0),
      y: f64::from(y0),
    };

    let q = Point {
      x: f64::from(x1),
      y: f64::from(y1),
    };

    assert_ne!(p, q);
    Segment { p, q }
  }

  #[test]
  fn test_paths_put() {
    let mut group = PathGroup { paths: vec![] };
    assert_eq!(group.paths.len(), 0);

    group.put(&seg(0, 0, 0, 1));
    assert_eq!(group.paths.len(), 1);

    group.put(&seg(5, 5, 7, 8));
    assert_eq!(group.paths.len(), 2);

    group.put(&seg(100, 54, 7, 8));
    assert_eq!(group.paths.len(), 2);

    assert_eq!(group.paths[0].len(), 2);
    assert_eq!(group.paths[1].len(), 3);
  }

  #[test]
  fn test_paths_put_merge() {
    let mut group = PathGroup { paths: vec![] };
    assert_eq!(group.paths.len(), 0);

    group.put(&seg(0, 0, 5, 0));
    assert_eq!(group.paths.len(), 1);

    group.put(&seg(15, 0, 10, 0));
    assert_eq!(group.paths.len(), 2);

    group.put(&seg(5, 0, 10, 0));
    assert_eq!(group.paths.len(), 1);

    let x_values = group.paths[0].iter().map(|p| p.x).collect::<Vec<f64>>();

    assert_eq!(x_values, vec![0_f64, 5_f64, 10_f64, 15_f64]);
  }

  #[test]
  fn test_try_merge_back() {
    let src = VecDeque::from_iter([1, 2, 3]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(!try_merge_back(&mut dest, &src));

    let src = VecDeque::from_iter([6, 8, 9]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(try_merge_back(&mut dest, &src));
    assert_eq!(dest, [4, 5, 6, 8, 9]);

    let src = VecDeque::from_iter([8, 9, 10]);
    assert!(!try_merge_back(&mut dest, &src));

    let src = VecDeque::from_iter([9, 12, 14]);
    assert!(try_merge_back(&mut dest, &src));
    assert_eq!(dest, [4, 5, 6, 8, 9, 12, 14]);
  }

  #[test]
  #[should_panic]
  fn test_try_merge_back_empty_src() {
    let src = VecDeque::from_iter([]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(!try_merge_back(&mut dest, &src));
  }

  #[test]
  #[should_panic]
  fn test_try_merge_back_empty_dest() {
    let src = VecDeque::from_iter([1, 2, 3]);
    let mut dest = VecDeque::from_iter([]);
    assert!(!try_merge_back(&mut dest, &src));
  }
}
