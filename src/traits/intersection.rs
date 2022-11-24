pub trait Intersects {
  fn intersects(&self, other: &Self) -> bool;
}

pub trait IntersectsHeuristic {
  fn intersects(&self, other: &Self, prev: &mut (i32, i32)) -> bool;
}
