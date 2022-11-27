use crate::{
  constants::EPS,
  traits::{common_boundary::CommonBoundary, intersection::Intersects},
  util::{equal, orientation},
};

use super::point::Point;

#[derive(Clone, Copy)]
pub struct Segment {
  pub p: Point,
  pub q: Point,
}

impl Intersects for Segment {
  fn intersects(&self, other: &Self) -> bool {
    let o1 = orientation(self.p, self.q, other.p);
    let o2 = orientation(self.p, self.q, other.q);

    if o1 * o2 >= 0 {
      return false;
    }

    let o3 = orientation(other.p, other.q, self.p);
    let o4 = orientation(other.p, other.q, self.q);
    o3 * o4 < 0
  }
}

impl CommonBoundary<Option<Self>> for Segment {
  fn common_boundary(&self, other: &Self) -> Option<Self> {
    if self.contains(other.p) && self.contains(other.q) {
      Some(*other)
    } else if other.contains(self.p) && other.contains(self.q) {
      Some(*self)
    } else {
      self.common_boundary_aux(other)
    }
  }
}

impl Segment {
  fn common_boundary_aux(&self, s: &Self) -> Option<Self> {
    if self.contains(s.p) && s.contains(self.p) {
      Some(self.p.seg(s.p))
    } else if self.contains(s.p) && s.contains(self.q) {
      Some(self.q.seg(s.p))
    } else if self.contains(s.q) && s.contains(self.p) {
      Some(self.p.seg(s.q))
    } else if self.contains(s.q) && s.contains(self.q) {
      Some(self.q.seg(s.q))
    } else {
      None
    }
  }

  #[must_use]
  pub fn is_horizontal(&self) -> bool {
    equal(self.p.y, self.q.y)
  }

  #[must_use]
  pub fn face_right(&self) -> bool {
    !self.is_horizontal() && self.p.y < self.q.y
  }

  #[must_use]
  pub fn face_left(&self) -> bool {
    !self.is_horizontal() && self.p.y > self.q.y
  }

  #[must_use]
  pub fn horizontal_distance(&self, v: Point) -> f64 {
    if equal(self.p.x, self.q.x) {
      v.x - self.p.x
    } else {
      let slope = (self.q.y - self.p.y) / (self.q.x - self.p.x);
      let b = self.p.y - self.p.x * slope;
      v.x - (v.y - b) / slope
    }
  }

  #[must_use]
  pub fn contains_except_endpoints(&self, r: Point) -> bool {
    if orientation(self.p, self.q, r) == 0 {
      (self.q - self.p) * (r - self.p) > EPS && (self.p - self.q) * (r - self.q) > EPS
    } else {
      false
    }
  }

  #[must_use]
  pub fn length(&self) -> f64 {
    self.p.dist(self.q)
  }

  fn contains(&self, r: Point) -> bool {
    self.p == r || self.q == r || self.contains_except_endpoints(r)
  }
}
