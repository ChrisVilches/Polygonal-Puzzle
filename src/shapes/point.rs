use std::{
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

impl SubAssign for Point {
  fn sub_assign(&mut self, other: Self) {
    *self = Self {
      x: self.x - other.x,
      y: self.y - other.y,
    };
  }
}

impl Mul for Point {
  type Output = f64;

  fn mul(self, rhs: Self) -> Self::Output {
    self.x.mul_add(rhs.x, self.y * rhs.y)
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

    let x = *coordinates
      .first()
      .ok_or("point string should have X value")?;
    let y = *coordinates
      .get(1)
      .ok_or("point string should have Y value")?;

    Ok(Self { x, y })
  }
}

impl Point {
  pub fn dist(&self, other: Self) -> f64 {
    (self.x - other.x).hypot(self.y - other.y)
  }

  pub fn cross(&self, other: Self) -> f64 {
    self.x * other.y - self.y * other.x
  }

  pub fn equal(&self, other: Self) -> bool {
    equal(self.x, other.x) && equal(self.y, other.y)
  }

  pub fn negate(&self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }

  pub const fn seg(&self, other: Self) -> Segment {
    Segment { p: *self, q: other }
  }

  pub fn rot_ccw(&self, t: f64) -> Self {
    Self {
      x: self.x * t.cos() - self.y * t.sin(),
      y: self.x.mul_add(t.sin(), self.y * t.cos()),
    }
  }
}
