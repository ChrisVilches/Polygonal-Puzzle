use crate::shapes::{polygon::Polygon, segment::Segment};

pub struct EdgeIterator<'a> {
  polygon: &'a Polygon,
  curr_idx: usize,
}

impl<'a> EdgeIterator<'a> {
  pub fn new(polygon: &'a Polygon) -> Self {
    Self {
      polygon,
      curr_idx: 0,
    }
  }
}

impl<'a> Iterator for EdgeIterator<'a> {
  type Item = Segment;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_idx >= self.polygon.len() {
      return None;
    }

    let item = Segment {
      p: self.polygon.vertices[self.curr_idx],
      q: self.polygon.vertex_at((self.curr_idx as i32) + 1),
    };

    self.curr_idx += 1;
    Some(item)
  }
}
