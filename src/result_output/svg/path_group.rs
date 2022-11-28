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

    // TODO: Test that point order is shuffled correctly (again)
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

  #[test]
  fn test_paths_put_merge_1() {
    for _ in 0..10_000 {
      let mut group = PathGroup { paths: vec![] };
      assert_eq!(group.paths.len(), 0);

      group.put(&seg(0, 0, 5, 0));
      assert_eq!(group.paths.len(), 1);

      group.put(&seg(15, 0, 10, 0));
      assert_eq!(group.paths.len(), 2);

      group.put(&seg(5, 0, 10, 0));
      assert_eq!(group.paths.len(), 1);
    }
  }

  #[test]
  fn test_paths_put_merge_2() {
    for _ in 0..10_000 {
      let mut group = PathGroup { paths: vec![] };
      assert_eq!(group.paths.len(), 0);

      group.put(&seg(15, 0, 10, 0));
      assert_eq!(group.paths.len(), 1);

      group.put(&seg(0, 0, 5, 0));
      assert_eq!(group.paths.len(), 2);

      group.put(&seg(5, 0, 10, 0));
      assert_eq!(group.paths.len(), 1);
    }
  }

  #[test]
  fn test_paths_put_merge_3() {
    for _ in 0..10_000 {
      let mut group = PathGroup { paths: vec![] };
      assert_eq!(group.paths.len(), 0);

      group.put(&seg(5, 0, 10, 0));
      assert_eq!(group.paths.len(), 1);

      group.put(&seg(1, 0, 3, 0));
      assert_eq!(group.paths.len(), 2);

      group.put(&seg(3, 0, 5, 0));
      assert_eq!(group.paths.len(), 1);
    }
  }

  #[test]
  fn test_paths_put_merge_4() {
    for _ in 0..10_000 {
      let mut group = PathGroup { paths: vec![] };
      let a = seg(490, 410, 490, 390);
      let b = seg(490, 390, 470, 390);
      let c = seg(490, 410, 510, 410);

      group.put(&a);
      assert_eq!(group.paths.len(), 1);

      group.put(&b);
      assert_eq!(group.paths.len(), 1);

      group.put(&c);
      assert_eq!(group.paths.len(), 1);
    }
  }

  #[test]
  fn test_paths_put_merge_shuffle() {
    let mut r = thread_rng();

    for _ in 0..10_000 {
      let mut segments = vec![];
      for x in 0..100 {
        segments.push(seg(x, 10, x + 1, 10));
      }

      segments.shuffle(&mut r);
      let group = PathGroup::from_segments(&segments);
      assert_eq!(group.paths.len(), 1);
    }
  }

  #[test]
  fn test_paths_put_merge_shuffle_remove_one() {
    let mut r = thread_rng();

    for _ in 0..10_000 {
      let mut segments = vec![];
      for x in 0..100 {
        if x == 50 {
          continue;
        }
        segments.push(seg(x, 10, x + 1, 10));
      }
      segments.shuffle(&mut r);
      let group = PathGroup::from_segments(&segments);
      assert_eq!(group.paths.len(), 2);
    }
  }

  #[test]
  fn test_try_merge() {
    let src = VecDeque::from_iter([1, 2, 3]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(!try_merge(&mut dest, &src));

    let src = VecDeque::from_iter([6, 8, 9]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(try_merge(&mut dest, &src));
    assert_eq!(dest, [4, 5, 6, 8, 9]);

    let src = VecDeque::from_iter([8, 9, 10]);
    assert!(!try_merge(&mut dest, &src));

    let src = VecDeque::from_iter([9, 12, 14]);
    assert!(try_merge(&mut dest, &src));
    assert_eq!(dest, [4, 5, 6, 8, 9, 12, 14]);
  }

  #[test]
  #[should_panic]
  fn test_try_merge_empty_src() {
    let src = VecDeque::from_iter([]);
    let mut dest = VecDeque::from_iter([4, 5, 6]);
    assert!(!try_merge(&mut dest, &src));
  }

  #[test]
  #[should_panic]
  fn test_try_merge_empty_dest() {
    let src = VecDeque::from_iter([1, 2, 3]);
    let mut dest = VecDeque::from_iter([]);
    assert!(!try_merge(&mut dest, &src));
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
}
