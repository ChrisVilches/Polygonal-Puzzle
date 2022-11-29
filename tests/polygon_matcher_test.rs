mod test_case;

// TODO: The project (crate) should be called "polygonal puzzle", not "polygon puzzle"

#[macro_use]
mod macros;
use crate::test_case::TestCase;
use polygon_puzzle::{
  polygon_matcher,
  shapes::{polygon::Polygon, polyline_set::PolylineSet, segment::Segment},
  traits::common_boundary::CommonBoundary,
};

static INPUT_DATA: &str = include_str!("./data/input");
static OUTPUT_DATA: &str = include_str!("./data/output");
static OUTPUT_BOUNDARY_COUNT_DATA: &str = include_str!("./data/boundary_count");

#[test]
fn test_official_data() {
  for TestCase {
    polygons: (p1, p2),
    correct_answer,
    boundary_count,
  } in TestCase::vec_from(INPUT_DATA, OUTPUT_DATA, OUTPUT_BOUNDARY_COUNT_DATA)
  {
    let (p1, p2, common_boundary_length) = polygon_matcher::best_match(&p1, &p2);

    let polyline_set = PolylineSet::from_segments(
      &<Polygon as CommonBoundary<Vec<Segment>>>::common_boundary(&p1, &p2),
    );

    let common_boundaries_result = polyline_set.get_polylines().len();

    println!(
      "{:.12} == {:.12} (common boundaries: {})",
      correct_answer, common_boundary_length, common_boundaries_result
    );
    assert_similar!(correct_answer, common_boundary_length);
    assert_eq!(boundary_count, common_boundaries_result);
  }
}

#[test]
fn test_official_data_read_io() {
  let all_cases = TestCase::vec_from(INPUT_DATA, OUTPUT_DATA, OUTPUT_BOUNDARY_COUNT_DATA);
  assert_eq!(all_cases.len(), 61);

  for case in all_cases {
    let (p1, p2) = case.polygons;
    assert!(!p1.vertices.is_empty());
    assert!(!p2.vertices.is_empty());
  }
}
