use polygonal_puzzle::{
  shapes::{point::Point, polygon::Polygon, polyline_set::PolylineSet, segment::Segment},
  traits::common_boundary::CommonBoundary,
};
use svg::node::element::{path::Data, Path};

const COMMON_BOUNDARY_COLOR: &str = "#00FF00";
const COMMON_BOUNDARY_STROKE_WIDTH: f64 = 3_f64;

pub struct ShapeToSvg {}

impl ShapeToSvg {
  fn polyline_to_svg_data<'a, T>(mut points: T) -> Data
  where
    T: Iterator<Item = &'a Point>,
  {
    let data = Data::new();

    match points.next() {
      Some(init) => points.fold(data.move_to((init.x, init.y)), |d, p| d.line_to((p.x, p.y))),
      None => data,
    }
  }

  pub fn polygon_to_svg_path(polygon: &Polygon, color: &str) -> Path {
    Path::new().set("fill", color).set(
      "d",
      Self::polyline_to_svg_data(polygon.vertices.iter()).close(),
    )
  }

  pub fn boundary_to_svg_paths(p1: &Polygon, p2: &Polygon) -> Vec<Path> {
    let segments = <Polygon as CommonBoundary<Vec<Segment>>>::common_boundary(p1, p2);

    PolylineSet::from_segments(&segments)
      .get_polylines()
      .into_iter()
      .map(|p| Self::polyline_to_svg_data(p.iter()))
      .map(|d| {
        Path::new()
          .set("stroke", COMMON_BOUNDARY_COLOR)
          .set("fill", "none")
          .set("stroke-linecap", "round")
          .set("stroke-width", COMMON_BOUNDARY_STROKE_WIDTH)
          .set("d", d)
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_polyline_to_svg_data() {
    let points = vec![
      Point { x: 0_f64, y: 0_f64 },
      Point { x: 1_f64, y: 1_f64 },
      Point { x: 4_f64, y: 5.2 },
    ];

    let svg = ShapeToSvg::polyline_to_svg_data(points.iter());
    assert_eq!(svg.len(), 3);

    let path = Path::new().set("d", svg);
    assert_eq!(path.to_string(), "<path d=\"M0,0 L1,1 L4,5.2\"/>");
  }
}
