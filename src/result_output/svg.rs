use super::{WriteResult, RESULTS_DIR};
use polygon_puzzle::shapes::polygon::Polygon;
use polygon_puzzle::util::{cmp, equal, max, min};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

const FACTOR: f64 = 60_f64;
const MARGIN: f64 = 10_f64;

pub struct SvgOutputWriter {}

impl SvgOutputWriter {
  fn polygon_to_svg(polygon: &Polygon, color: &str) -> Path {
    let mut data = Data::new();

    for i in 0..polygon.vertices.len() {
      let v = polygon.vertices[i];

      if i == 0 {
        data = data.move_to((v.x, v.y));
      } else {
        data = data.line_to((v.x, v.y));
      }
    }

    Path::new()
      .set("fill", color)
      .set("stroke", "black")
      .set("stroke-width", 1)
      .set("d", data.close())
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

    let document = Document::new()
      .set("viewBox", (0, 0, max_x + MARGIN, max_y + MARGIN))
      .add(Self::polygon_to_svg(&p1, "#a64459"))
      .add(Self::polygon_to_svg(&p2, "#2837ad"));

    svg::save(path, &document).unwrap();
  }

  fn move_polygons_corner(polygons: &mut [&mut Polygon]) {
    let mut min_x = 1e10;
    let mut min_y = 1e10;

    let iter = polygons.iter_mut().flat_map(|p| p.vertices.iter_mut());

    iter.for_each(|p| {
      min_x = min(min_x, p.x);
      min_y = min(min_y, p.y);
    });

    let iter = polygons.iter_mut().flat_map(|p| p.vertices.iter_mut());

    iter.for_each(|p| {
      p.x -= min_x;
      p.y -= min_y;
    });
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

// TODO: Nice, but SVG file opens very slowly (maybe the data is inefficient? but that's
//       not supposed to happen with SVG)
impl WriteResult for SvgOutputWriter {
  fn write_result(&mut self, boundary: f64, case_number: i32, mut p1: Polygon, mut p2: Polygon) {
    p1.vertices
      .iter_mut()
      .chain(p2.vertices.iter_mut())
      .for_each(|p| {
        p.x *= FACTOR;
        p.y *= FACTOR;
      });

    if equal(boundary, 0_f64) {
      Self::create_svg_no_match(&mut p1, &mut p2);
    } else {
      Self::move_polygons_corner(&mut [&mut p1, &mut p2]);
    }

    Self::render_polygons_image(p1, p2, &format!("{}/{:0>2}.svg", RESULTS_DIR, case_number));
  }
}
