use crate::{
  constants::EPS,
  traits::{
    common_boundary::CommonBoundary,
    intersection::{Intersection, IntersectionHeuristic},
  },
  util::{angle, orientation},
};

use super::{point::Point, segment::Segment};

// TODO: Add indexing by passing the index to the vertices field.
#[derive(Clone)]
pub struct Polygon {
  // TODO: Make this private, and just use the "vertex_at" or implement the index operator.
  pub vertices: Vec<Point>,
}

impl IntersectionHeuristic for Polygon {
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
    return false;
  }
}

impl CommonBoundary for Polygon {
  fn common_boundary(&self, other: &Self) -> f64 {
    let mut total = 0_f64;

    // TODO: Make this functional
    for i in 0..self.len() {
      // TODO: Convert edge creation to a polygon method.
      //       Maybe use in other places as well.
      let edge1 = Segment::new(self.vertices[i], self.vertex_at((i as i32) + 1));
      for j in 0..other.len() {
        let edge2 = Segment::new(other.vertices[j], other.vertex_at((j as i32) + 1));
        total += edge1.common_boundary(&edge2);
      }
    }

    total
  }
}

impl Polygon {
  pub fn new(vertices: Vec<Point>) -> Self {
    Self { vertices }
  }

  pub fn len(&self) -> usize {
    self.vertices.len()
  }

  pub fn vertex_at(&self, i: i32) -> Point {
    let n = self.len() as i32;
    let i = (i + (n << 10)) % n;
    self.vertices[i as usize]
  }

  pub fn negate(&self) -> Self {
    Self {
      vertices: self.vertices.iter().map(Point::negate).collect(),
    }
  }

  // TODO: Transform this into an iterator and yield all rotations?
  pub fn rotations(&self) -> Vec<Polygon> {
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
    let a0 = self.vertex_at(i - 1);
    let a1 = self.vertex_at(i);
    let a2 = self.vertex_at(i + 1);

    let b0 = other.vertex_at(j - 1);
    let b1 = other.vertex_at(j);
    let b2 = other.vertex_at(j + 1);

    if Segment::new(a1, a2).intersects(&Segment::new(b1, b2)) {
      return true;
    }

    if Segment::new(b1, b2).contains_except_endpoints(a1) {
      if orientation(b1, b2, a2) == 1 {
        return true;
      }
      if orientation(b1, b2, a0) == 1 {
        return true;
      }
    }
    if Segment::new(a1, a2).contains_except_endpoints(b1) {
      if orientation(a1, a2, b2) == 1 {
        return true;
      }
      if orientation(a1, a2, b0) == 1 {
        return true;
      }
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
