pub trait Intersection {
  fn intersects(&self, other: &Self) -> bool;
}

pub trait IntersectionHeuristic {
  fn intersects(&self, other: &Self, prev: &mut (i32, i32)) -> bool;
}
