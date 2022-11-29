use self::shape_to_svg::ShapeToSvg;

use super::{WriteResult, RESULTS_DIR};
use polygonal_puzzle::shapes::point::Point;
use polygonal_puzzle::shapes::polygon::Polygon;
use polygonal_puzzle::util::{cmp, equal, max, min};
use svg::Document;

const FACTOR: f64 = 20_f64;
const MARGIN: f64 = 10_f64;
const COLOR_POLYGON_1: &str = "#5b65b3";
const COLOR_POLYGON_2: &str = "#a64459";

mod shape_to_svg;

pub struct OutputWriter {}

impl OutputWriter {
  fn image_size_with_margin(p1: &Polygon, p2: &Polygon) -> (f64, f64) {
    let mut max_x = -1e10;
    let mut max_y = -1e10;

    p1.vertices.iter().chain(p2.vertices.iter()).for_each(|p| {
      max_x = max(max_x, p.x);
      max_y = max(max_y, p.y);
    });

    (MARGIN.mul_add(2_f64, max_x), MARGIN.mul_add(2_f64, max_y))
  }

  fn add_margin(polygon: &mut Polygon) {
    polygon.vertices.iter_mut().for_each(|p| {
      p.x += MARGIN;
      p.y += MARGIN;
    });
  }

  fn render_polygons_image(mut p1: Polygon, mut p2: Polygon, path: &str) {
    let (width, height) = Self::image_size_with_margin(&p1, &p2);

    Self::add_margin(&mut p1);
    Self::add_margin(&mut p2);

    let document_init = Document::new()
      .set("viewBox", (0, 0, width, height))
      .add(ShapeToSvg::polygon_to_svg_path(&p1, COLOR_POLYGON_1))
      .add(ShapeToSvg::polygon_to_svg_path(&p2, COLOR_POLYGON_2));

    let svg_image = ShapeToSvg::boundary_to_svg_paths(&p1, &p2)
      .into_iter()
      .fold(document_init, svg::node::element::SVG::add);

    svg::save(path, &svg_image).unwrap();
  }

  fn scale_polygon(polygon: &mut Polygon) {
    polygon.vertices.iter_mut().for_each(|p| {
      p.x *= FACTOR;
      p.y *= -FACTOR;
    });
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

  fn separate_polygons(p1: &Polygon, p2: &mut Polygon) {
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
    Self::scale_polygon(&mut p1);
    Self::scale_polygon(&mut p2);

    if equal(boundary, 0_f64) {
      Self::move_polygons_corner(&mut [&mut p1]);
      Self::move_polygons_corner(&mut [&mut p2]);
      Self::separate_polygons(&p1, &mut p2);
    } else {
      Self::move_polygons_corner(&mut [&mut p1, &mut p2]);
    }

    Self::render_polygons_image(p1, p2, &format!("{}/{:0>2}.svg", RESULTS_DIR, case_number));
  }
}
