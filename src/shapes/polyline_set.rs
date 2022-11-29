use std::collections::HashMap;

use crate::shapes::{point::Point, segment::Segment};

struct Node {
  value: Point,
  neighbors: [Option<usize>; 2],
}

impl Node {
  const fn new(p: Point) -> Self {
    Self {
      value: p,
      neighbors: [None, None],
    }
  }

  fn is_full(&self) -> bool {
    self.neighbors.iter().all(Option::is_some)
  }

  fn add_link(&mut self, node_idx: usize) {
    assert!(!self.is_full(), "cannot add neighbors");

    if self.neighbors[0].is_none() {
      self.neighbors[0] = Some(node_idx);
    } else {
      self.neighbors[1] = Some(node_idx);
    }
  }
}

pub struct PolylineSet {
  nodes: Vec<Node>,
  point_to_node_idx: HashMap<Point, usize>,
}

impl PolylineSet {
  fn find_or_create_index(&mut self, p: Point) -> usize {
    if let Some(idx) = self.point_to_node_idx.get(&p) {
      *idx
    } else {
      self.nodes.push(Node::new(p));
      let new_idx = self.nodes.len() - 1;
      self.point_to_node_idx.insert(p, new_idx);
      new_idx
    }
  }

  fn put(&mut self, p: Point, q: Point) {
    let idx_p = self.find_or_create_index(p);
    let idx_q = self.find_or_create_index(q);
    self.nodes[idx_p].add_link(idx_q);
    self.nodes[idx_q].add_link(idx_p);
  }

  fn dfs(&self, u: usize, visited: &mut Vec<bool>, polylines: &mut Vec<Vec<Point>>) {
    if visited[u] {
      return;
    }

    visited[u] = true;

    polylines.last_mut().unwrap().push(self.nodes[u].value);

    for v in self.nodes[u].neighbors.iter().flatten() {
      self.dfs(*v, visited, polylines);
    }
  }

  #[must_use]
  pub fn get_polylines(&self) -> Vec<Vec<Point>> {
    let mut visited = vec![false; self.nodes.len()];

    let mut polylines: Vec<Vec<Point>> = vec![];

    for node_idx in 0..self.nodes.len() {
      if self.nodes[node_idx].is_full() {
        continue;
      }

      if visited[node_idx] {
        continue;
      }

      polylines.push(vec![]);

      self.dfs(node_idx, &mut visited, &mut polylines);
    }

    polylines
  }

  #[must_use]
  pub fn from_segments(segments: &[Segment]) -> Self {
    let mut polylines = Self {
      nodes: vec![],
      point_to_node_idx: HashMap::new(),
    };

    for s in segments {
      polylines.put(s.p, s.q);
    }

    polylines
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
  fn test_polyline_set_put() {
    let mut set = PolylineSet::from_segments(&[]);
    assert_eq!(set.get_polylines().len(), 0);

    let Segment { p, q } = seg(0, 0, 0, 1);
    set.put(p, q);
    assert_eq!(set.get_polylines().len(), 1);

    let Segment { p, q } = seg(5, 5, 7, 8);
    set.put(p, q);
    assert_eq!(set.get_polylines().len(), 2);

    let Segment { p, q } = seg(100, 54, 7, 8);
    set.put(p, q);
    assert_eq!(set.get_polylines().len(), 2);

    assert_eq!(set.get_polylines()[0].len(), 2);
    assert_eq!(set.get_polylines()[1].len(), 3);
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
  fn test_polyline_set_from_segments(skip: Vec<i32>, set_size: usize, step_by: usize) {
    let skip_set: HashSet<i32> = skip.into_iter().collect();

    for _ in 0..100_000 {
      let mut segments: Vec<Segment> = (0..150)
        .step_by(step_by)
        .filter(|i| !skip_set.contains(i))
        .map(|x| seg(x, 10, x + 1, 10))
        .collect();
      segments.shuffle(&mut thread_rng());

      let set = PolylineSet::from_segments(&segments);
      assert_eq!(set.get_polylines().len(), set_size);
    }
  }

  #[test]
  #[should_panic]
  fn test_polyline_set_from_segments_panic() {
    let a = seg(0, 0, 1, 0);
    let b = seg(2, 0, 3, 0);
    let c = seg(1, 0, 2, 0);

    let mut polyline_set = PolylineSet::from_segments(&[a, b, c]);

    let Segment { p, q } = seg(1, 0, 1, 2);
    polyline_set.put(p, q);
  }
}
