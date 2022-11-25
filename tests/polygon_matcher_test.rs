mod test_case;

#[macro_use]
mod macros;
use crate::test_case::TestCase;
use polygon_puzzle::polygon_matcher;

static INPUT_DATA: &str = include_str!("./data/input");
static OUTPUT_DATA: &str = include_str!("./data/output");

#[test]
fn test_best_match() {
  for TestCase {
    polygons: (p1, p2),
    correct_answer,
  } in TestCase::vec_from(INPUT_DATA, OUTPUT_DATA)
  {
    let (_, _, boundary) = polygon_matcher::best_match(&p1, &p2);

    println!("{:.12} == {:.12}", correct_answer, boundary);
  }
}

#[test]
fn test_read_all_test_cases_data() {
  let all_cases = TestCase::vec_from(INPUT_DATA, OUTPUT_DATA);
  assert_eq!(all_cases.len(), 59);

  for case in all_cases {
    let (p1, p2) = case.polygons;
    assert!(!p1.vertices.is_empty());
    assert!(!p2.vertices.is_empty());
  }
}
