use crate::shapes::{point::Point, polygon::Polygon};

pub struct VertexIterator<'a> {
  polygon: &'a Polygon,
  curr_idx: usize,
}

impl<'a> VertexIterator<'a> {
  pub fn new(polygon: &'a Polygon) -> Self {
    Self {
      polygon,
      curr_idx: 0,
    }
  }
}

impl<'a> Iterator for VertexIterator<'a> {
  type Item = (Point, Point, Point);

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_idx >= self.polygon.len() {
      return None;
    }

    let item = self.polygon.vertices_at(self.curr_idx as i32);
    self.curr_idx += 1;
    Some(item)
  }
}
