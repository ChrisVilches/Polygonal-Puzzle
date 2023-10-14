use std::{
  fmt::Display,
  ops::{Mul, Sub, SubAssign},
  str::FromStr,
};

use crate::util::equal;

use super::segment::Segment;

#[derive(Clone, Copy, Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl std::hash::Hash for Point {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let x = (self.x * 1000_f64).round() as i64;
    let y = (self.y * 1000_f64).round() as i64;
    x.hash(state);
    y.hash(state);
  }
}

impl Eq for Point {}

impl SubAssign for Point {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({:.6}, {:.6})", self.x, self.y)
  }
}

impl Mul for Point {
  type Output = f64;

  fn mul(self, rhs: Self) -> Self::Output {
    self.x.mul_add(rhs.x, self.y * rhs.y)
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    equal(self.x, other.x) && equal(self.y, other.y)
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl FromStr for Point {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coordinates: Vec<f64> = s
      .split(' ')
      .map(str::parse)
      .collect::<Result<Vec<f64>, std::num::ParseFloatError>>()
      .map_err(|e| e.to_string())?;

    let x = *coordinates.first().unwrap();
    let y = *coordinates
      .get(1)
      .ok_or("point string should have Y value")?;

    Ok(Self { x, y })
  }
}

impl Point {
  #[must_use]
  pub fn dist(&self, other: Self) -> f64 {
    (self.x - other.x).hypot(self.y - other.y)
  }

  #[must_use]
  pub fn cross(&self, other: Self) -> f64 {
    self.x.mul_add(other.y, -self.y * other.x)
  }

  #[must_use]
  pub fn negate(&self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }

  #[must_use]
  pub const fn seg(&self, other: Self) -> Segment {
    Segment { p: *self, q: other }
  }

  #[must_use]
  pub fn rot_ccw(&self, t: f64) -> Self {
    Self {
      x: self.x.mul_add(t.cos(), -self.y * t.sin()),
      y: self.x.mul_add(t.sin(), self.y * t.cos()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("4 5", Ok(Point { x: 4_f64, y: 5_f64 }))]
  #[test_case("64.45 15.222", Ok(Point { x: 64.45, y: 15.222 }))]
  #[test_case("-200.1 -100", Ok(Point { x: -200.1, y: -100_f64 }))]
  #[test_case("64.45", Err("point string should have Y value".to_owned()))]
  #[test_case("  ", Err("cannot parse float from empty string".to_owned()))]
  #[test_case("44 x 66", Err("invalid float literal".to_owned()))]
  #[test_case("xxx 543", Err("invalid float literal".to_owned()))]
  #[test_case("543 xxx", Err("invalid float literal".to_owned()))]
  fn test_from_str(s: &str, res: Result<Point, String>) {
    assert_eq!(s.parse::<Point>(), res);
  }

  #[test_case(Point { x: 5_f64, y: 6_f64 }, "(5.000000, 6.000000)")]
  #[test_case(Point { x: 1.23456, y: 6.54 }, "(1.234560, 6.540000)")]
  fn test_display(p: Point, res: &str) {
    assert_eq!(format!("{}", p), res);
  }

  #[test_case(Point{ x: 10_f64, y: 0_f64 }, std::f64::consts::PI / 2_f64, Point{ x: 0_f64, y: 10_f64 })]
  fn test_rot_ccw(p: Point, ang: f64, res: Point) {
    assert_eq!(p.rot_ccw(ang), res);
  }
}
