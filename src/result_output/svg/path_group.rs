use polygon_puzzle::shapes::{point::Point, segment::Segment};
use std::collections::VecDeque;

fn try_merge<T: PartialEq + Copy>(dest: &mut VecDeque<T>, src: &VecDeque<T>) -> bool {
  let src_front = *src.front().unwrap();
  let src_back = *src.back().unwrap();
  let dest_front = *dest.front().unwrap();
  let dest_back = *dest.back().unwrap();

  let prev_len = dest.len();

  if dest_back == src_front {
    dest.extend(src.iter().skip(1));
  } else if dest_back == src_back {
    dest.extend(src.iter().rev().skip(1));
  } else if dest_front == src_front {
    src.iter().skip(1).for_each(|t| dest.push_front(*t));
  } else if dest_front == src_back {
    src.iter().rev().skip(1).for_each(|t| dest.push_front(*t));
  }

  prev_len != dest.len()
}

fn borrow_mut_two<T>(v: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
  assert!(i < j);
  let (left, right) = v.split_at_mut(j);
  (&mut left[i], &mut right[0])
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
      for j in i + 1..self.paths.len() {
        let (a, b) = borrow_mut_two(&mut self.paths, i, j);

        if try_merge(a, b) {
          return Some(j);
        }
      }
    }
    None
  }

  fn merge_once(&mut self) {
    if let Some(idx) = self.merge_find_index() {
      self.paths.remove(idx);
    }
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
  use std::collections::HashSet;

  use super::*;
  use rand::{seq::SliceRandom, thread_rng};
  use test_case::test_case;

  fn seg(x0: i32, y0: i32, x1: i32, y1: i32) -> Segment {
    let point1 = Point {
      x: f64::from(x0),
      y: f64::from(y0),
    };

    let point2 = Point {
      x: f64::from(x1),
      y: f64::from(y1),
    };

    let (p, q) = if rand::random::<f64>() > 0.5 {
      (point1, point2)
    } else {
      (point2, point1)
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

  #[test_case(vec![], 1, 1)]
  #[test_case(vec![0], 1, 1)]
  #[test_case(vec![50], 2, 1)]
  #[test_case(vec![50, 70], 3, 1)]
  #[test_case(vec![50, 69, 70], 3, 1)]
  #[test_case(vec![50, 68, 70], 4, 1)]
  #[test_case(vec![0, 149], 1, 1)]
  #[test_case(vec![0, 148], 2, 1)]
  #[test_case(vec![], 75, 2)]
  fn test_paths_put_merge_shuffle(skip: Vec<i32>, paths_result: usize, step_by: usize) {
    let skip_set: HashSet<i32> = skip.into_iter().collect();

    for _ in 0..10_000 {
      let mut segments: Vec<Segment> = (0..150)
        .step_by(step_by)
        .filter(|i| !skip_set.contains(i))
        .map(|x| seg(x, 10, x + 1, 10))
        .collect();
      segments.shuffle(&mut thread_rng());

      let group = PathGroup::from_segments(&segments);
      assert_eq!(group.paths.len(), paths_result);
    }
  }

  #[test_case(&[1, 2, 3], &[4, 5, 6], false, &[4, 5, 6])]
  #[test_case(&[6, 8, 9], &[4, 5, 6], true, &[4, 5, 6, 8, 9])]
  #[test_case(&[8, 9, 10], &[4, 5, 6], false, &[4, 5, 6])]
  #[test_case(&[9, 12, 14], &[4, 8, 9], true, &[4, 8, 9, 12, 14])]
  #[test_case(&[1, 2], &[2, 3, 4], true, &[1, 2, 3, 4])]
  fn test_try_merge(s: &[i32], d: &[i32], result: bool, d2: &[i32]) {
    let src = VecDeque::from_iter(s.iter().cloned());
    let mut dest = VecDeque::from_iter(d.iter().cloned());
    assert_eq!(try_merge(&mut dest, &src), result);
    assert_eq!(dest, d2);
  }

  #[test_case(&[1, 2, 3], &[])]
  #[test_case(&[], &[4, 5, 6])]
  #[test_case(&[], &[])]
  #[should_panic]
  fn test_try_merge_panic(s: &[i32], d: &[i32]) {
    let src = VecDeque::from_iter(s.iter().cloned());
    let mut dest = VecDeque::from_iter(d.iter().cloned());
    try_merge(&mut dest, &src);
  }

  #[test_case(2, 3)]
  #[test_case(0, 7)]
  #[test_case(1, 6)]
  #[test_case(3, 8)]
  #[test_case(5, 6)]
  fn test_borrow_mut_two(i: usize, j: usize) {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let (ref_a, ref_b) = borrow_mut_two(&mut v, i, j);

    *ref_a += 1;
    *ref_b += 5;

    assert_eq!(*ref_a, i + 1);
    assert_eq!(*ref_b, j + 5);
  }

  #[test_case(2, 2)]
  #[test_case(3, 2)]
  #[should_panic]
  fn test_borrow_mut_two_panic(i: usize, j: usize) {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let (_ref_a, _ref_b) = borrow_mut_two(&mut v, i, j);
  }
}
