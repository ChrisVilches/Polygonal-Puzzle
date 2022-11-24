use crate::{
  constants::EPS,
  iterators::{edge_iterator::EdgeIterator, vertex_iterator::VertexIterator},
  traits::{
    common_boundary::CommonBoundary,
    intersection::{Intersects, IntersectsHeuristic},
  },
  util::{angle, ccw},
};
use std::{borrow::Borrow, str::FromStr};

use super::{point::Point, segment::Segment};

#[derive(Clone, Debug)]
pub struct Polygon {
  pub vertices: Vec<Point>,
}

impl IntersectsHeuristic for Polygon {
  #[allow(clippy::integer_division)]
  fn intersects(&self, other: &Self, prev: &mut (i32, i32)) -> bool {
    let iters_i = ((self.len() / 2) + 1) as i32;
    let iters_j = ((other.len() / 2) + 1) as i32;

    for i in 0..iters_i {
      for j in 0..iters_j {
        if self.intersection_aux(other, prev.0 + i, prev.1 + j) {
          prev.0 += i;
          prev.1 += j;
          return true;
        }
        if j > 0 && self.intersection_aux(other, prev.0 + i, prev.1 - j) {
          prev.0 += i;
          prev.1 -= j;
          return true;
        }
      }

      if i == 0 {
        continue;
      }

      for j in 0..iters_j {
        if self.intersection_aux(other, prev.0 - i, prev.1 + j) {
          prev.0 -= i;
          prev.1 += j;
          return true;
        }
        if j > 0 && self.intersection_aux(other, prev.0 - i, prev.1 - j) {
          prev.0 -= i;
          prev.1 -= j;
          return true;
        }
      }
    }

    false
  }
}

impl CommonBoundary for Polygon {
  fn common_boundary(&self, other: &Self) -> f64 {
    self
      .edges()
      .flat_map(|e1: Segment| {
        other
          .edges()
          .map(move |e2: Segment| e1.common_boundary(&e2))
      })
      .sum()
  }
}

impl Polygon {
  /// # Errors
  /// Parsing errors may occur.
  pub fn from<I>(vertices_count: usize, lines: &mut I) -> Result<Self, String>
  where
    I: Iterator,
    I::Item: Borrow<str>,
  {
    let mut vertices: Vec<Point> = lines
      .take(vertices_count)
      .map(|line| Point::from_str(line.borrow()))
      .collect::<Result<Vec<Point>, String>>()?;

    vertices.reverse();

    Ok(Self { vertices })
  }

  #[must_use]
  pub fn new(vertices: Vec<Point>) -> Self {
    Self { vertices }
  }

  #[must_use]
  pub const fn edges(&self) -> EdgeIterator {
    EdgeIterator::new(self)
  }

  #[must_use]
  pub const fn vertices(&self) -> VertexIterator {
    VertexIterator::new(self)
  }

  #[must_use]
  pub fn len(&self) -> usize {
    self.vertices.len()
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.vertices.is_empty()
  }

  #[must_use]
  pub fn vertex_at(&self, i: i32) -> Point {
    let n = self.len() as i32;
    let i = (i + (n << 10)) % n;
    self.vertices[i as usize]
  }

  #[must_use]
  pub fn vertices_at(&self, i: i32) -> (Point, Point, Point) {
    let n = self.len() as i32;
    let i = (i + (n << 10)) % n;
    (
      self.vertex_at(i - 1),
      self.vertex_at(i),
      self.vertex_at(i + 1),
    )
  }

  #[must_use]
  pub fn negate(&self) -> Self {
    Self {
      vertices: self.vertices.iter().map(Point::negate).collect(),
    }
  }

  #[must_use]
  pub fn rotations(&self) -> Vec<Self> {
    let polygon = &mut self.clone();
    let mut polygons = vec![];

    for i in 0..polygon.len() {
      let p = polygon.vertices[i];

      polygon.vertices.iter_mut().for_each(|point| *point -= p);

      let q = polygon.vertex_at((i + 1) as i32);
      let ang = q.y.atan2(-q.x);

      polygon
        .vertices
        .iter_mut()
        .for_each(|p| *p = p.rot_ccw(ang));

      let new_q = polygon.vertex_at((i + 1) as i32);

      polygon.vertices.iter_mut().for_each(|p| *p -= new_q);

      polygons.push(polygon.clone());
    }

    polygons
  }

  fn intersection_aux(&self, other: &Self, i: i32, j: i32) -> bool {
    let (a0, a1, a2) = self.vertices_at(i);
    let (b0, b1, b2) = other.vertices_at(j);

    if a1.seg(a2).intersects(&b1.seg(b2)) {
      return true;
    }

    if b1.seg(b2).contains_except_endpoints(a1) && (ccw(b1, b2, a2) || ccw(b1, b2, a0)) {
      return true;
    }
    if a1.seg(a2).contains_except_endpoints(b1) && (ccw(a1, a2, b2) || ccw(a1, a2, b0)) {
      return true;
    }

    if a1.equal(b1) {
      let th = angle(b2 - b1, b0 - b1);
      let th2 = angle(b2 - b1, a0 - b1);
      if th2 > EPS && th2 < th - EPS {
        return true;
      }
      let th2 = angle(b2 - b1, a2 - b1);
      if th2 > EPS && th2 < th - EPS {
        return true;
      }
    }

    false
  }
}
