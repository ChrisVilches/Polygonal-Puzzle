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
    x + 2_f64 * std::f64::consts::PI
  } else {
    x
  }
}

pub fn orientation(o: Point, a: Point, b: Point) -> i8 {
  let cross = (a - o).cross(b - o);
  if zero(cross) {
    return 0;
  }
  if cross > 0_f64 {
    1
  } else {
    -1
  }
}

#[inline]
pub fn max(a: f64, b: f64) -> f64 {
  if a > b {
    a
  } else {
    b
  }
}
