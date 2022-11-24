use std::cmp::Ordering;

use crate::{constants::EPS, shapes::point::Point};

#[inline]
fn zero(x: f64) -> bool {
  x.abs() < EPS
}

#[inline]
pub fn equal(a: f64, b: f64) -> bool {
  zero(a - b)
}

pub fn angle(a: Point, b: Point) -> f64 {
  let x = a.cross(b).atan2(a * b);
  if x < 0_f64 {
    2_f64.mul_add(std::f64::consts::PI, x)
  } else {
    x
  }
}

pub fn orientation(o: Point, a: Point, b: Point) -> i8 {
  let cross = (a - o).cross(b - o);
  if zero(cross) {
    0
  } else if cross > 0_f64 {
    1
  } else {
    -1
  }
}

// TODO: Is this really counter clock wise?
#[inline]
pub fn ccw(o: Point, a: Point, b: Point) -> bool {
  orientation(o, a, b) == 1
}

#[inline]
pub fn max(a: f64, b: f64) -> f64 {
  if a > b {
    a
  } else {
    b
  }
}

#[inline]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn cmp(a: &f64, b: &f64) -> Ordering {
  if a < b {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}
