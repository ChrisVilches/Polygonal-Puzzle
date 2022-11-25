use polygon_puzzle::shapes::polygon::Polygon;

pub mod desmos;
pub mod paths;
pub mod svg;

const RESULTS_DIR: &str = "results";

pub trait WriteResult {
  fn write_result(&mut self, boundary: f64, case_number: i32, p1: Polygon, p2: Polygon);
}
