use super::paths::PathGroup;
use super::{WriteResult, RESULTS_DIR};
use polygon_puzzle::shapes::point::Point;
use polygon_puzzle::shapes::polygon::Polygon;
use polygon_puzzle::shapes::segment::Segment;
use polygon_puzzle::traits::common_boundary::CommonBoundary;
use polygon_puzzle::util::{cmp, equal, max, min};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

const FACTOR: f64 = 20_f64;
const MARGIN: f64 = 10_f64;
const COMMON_BOUNDARY_STROKE_WIDTH: f64 = 3_f64;
const COMMON_BOUNDARY_COLOR: &str = "#00FF00";
const COLOR_POLYGON_1: &str = "#5b65b3";
const COLOR_POLYGON_2: &str = "#a64459";

fn polyline_to_svg<'a, T>(mut points: T) -> Data
where
  T: Iterator<Item = &'a Point>,
{
  let data = Data::new();

  match points.next() {
    Some(init) => points.fold(data.move_to((init.x, init.y)), |d, p| d.line_to((p.x, p.y))),
    None => data,
  }
}

// TODO: Some of the functions in this module are a bit weird. Arguments and return values
//       are not 100% clear. A bunch of effectful "functions", etc.
pub struct OutputWriter {}

impl OutputWriter {
  fn polygon_to_svg(polygon: &Polygon, color: &str) -> Path {
    Path::new()
      .set("fill", color)
      .set("d", polyline_to_svg(polygon.vertices.iter()).close())
  }

  fn boundary_to_svg(p1: &Polygon, p2: &Polygon) -> Vec<Path> {
    let segments = <Polygon as CommonBoundary<Vec<Segment>>>::common_boundary(p1, p2);

    PathGroup::from_segments(&segments)
      .paths
      .into_iter()
      .map(|p| polyline_to_svg(p.points.iter()))
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

  fn polygons_to_document(p1: &Polygon, p2: &Polygon, x_size: f64, y_size: f64) -> Document {
    let document_init = Document::new()
      .set("viewBox", (0, 0, x_size, y_size))
      .add(Self::polygon_to_svg(p1, COLOR_POLYGON_1))
      .add(Self::polygon_to_svg(p2, COLOR_POLYGON_2));

    Self::boundary_to_svg(p1, p2)
      .into_iter()
      .fold(document_init, svg::node::element::SVG::add)
  }

  fn render_polygons_image(mut p1: Polygon, mut p2: Polygon, path: &str) {
    let mut max_x = -1e10;
    let mut max_y = -1e10;

    p1.vertices
      .iter_mut()
      .chain(p2.vertices.iter_mut())
      .for_each(|p| {
        p.x += MARGIN;
        p.y += MARGIN;
        max_x = max(max_x, p.x);
        max_y = max(max_y, p.y);
      });

    let document = Self::polygons_to_document(&p1, &p2, max_x + MARGIN, max_y + MARGIN);

    svg::save(path, &document).unwrap();
  }

  fn move_polygons_corner(polygons: &mut [&mut Polygon]) {
    let mut min_point = Point { x: 1e10, y: 1e10 };

    let iter = polygons.iter_mut().flat_map(|p| p.vertices.iter_mut());

    iter.for_each(|p| {
      min_point.x = min(min_point.x, p.x);
      min_point.y = min(min_point.y, p.y);
    });

    let iter = polygons.iter_mut().flat_map(|p| p.vertices.iter_mut());

    iter.for_each(|p| *p -= min_point);
  }

  fn create_svg_no_match(mut p1: &mut Polygon, mut p2: &mut Polygon) {
    Self::move_polygons_corner(&mut [&mut p1]);
    Self::move_polygons_corner(&mut [&mut p2]);

    let shift_x = p1
      .vertices
      .iter()
      .map(|p| p.x)
      .max_by(cmp)
      .unwrap_or_default();

    p2.vertices.iter_mut().for_each(|p| p.x += MARGIN + shift_x);
  }
}

impl WriteResult for OutputWriter {
  fn write_result(&mut self, boundary: f64, case_number: i32, mut p1: Polygon, mut p2: Polygon) {
    p1.vertices
      .iter_mut()
      .chain(p2.vertices.iter_mut())
      .for_each(|p| {
        p.x *= FACTOR;
        p.y *= -FACTOR;
      });

    if equal(boundary, 0_f64) {
      Self::create_svg_no_match(&mut p1, &mut p2);
    } else {
      Self::move_polygons_corner(&mut [&mut p1, &mut p2]);
    }

    Self::render_polygons_image(p1, p2, &format!("{}/{:0>2}.svg", RESULTS_DIR, case_number));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_polyline_to_svg() {
    let points = vec![
      Point { x: 0_f64, y: 0_f64 },
      Point { x: 1_f64, y: 1_f64 },
      Point { x: 4_f64, y: 5.2 },
    ];

    let svg = polyline_to_svg(points.iter());
    assert_eq!(svg.len(), 3);

    let path = Path::new().set("d", svg);
    assert_eq!(path.to_string(), "<path d=\"M0,0 L1,1 L4,5.2\"/>");
  }
}
