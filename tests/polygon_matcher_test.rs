// TODO: Create an integration test.
//       Open the input file, process every case
//       compare it with the output.
//
//       When this is done, change this crate to a library crate instead of a binary crate.
//       Although a binary would also be great (for running the program and checking the results myself)

use polygon_puzzle::{
  polygon_matcher,
  shapes::{point::Point, polygon::Polygon},
};

struct TestCase {
  polygons: (Polygon, Polygon),
  correct_answer: f64,
}

static INPUT_DATA: &str = include_str!("./data/input");
static OUTPUT_DATA: &str = include_str!("./data/output");

fn read_all_test_cases_data() -> Vec<TestCase> {
  let answers = OUTPUT_DATA.lines().map(|line| line.parse().unwrap());

  let mut polygons: Vec<Polygon> = vec![];

  let lines_iterator = &mut INPUT_DATA.lines();

  while let Some(line) = lines_iterator.next() {
    let n = line.parse().unwrap();

    let mut vertices: Vec<Point> = lines_iterator
      .take(n)
      .map(|line| line.parse().unwrap())
      .collect();

    vertices.reverse();

    polygons.push(Polygon { vertices });
  }

  let odd = polygons.iter().step_by(2).cloned();
  let even = polygons.iter().skip(1).step_by(2).cloned();
  let polygon_pair = odd.zip(even);

  polygon_pair
    .zip(answers)
    .map(|(polygons, correct_answer)| TestCase {
      polygons,
      correct_answer,
    })
    .collect()
}

// TODO: Make sure this macro is properly made.
macro_rules! assert_similar {
  ($left:expr, $right:expr) => {
    match (&$left, &$right) {
      (left_val, right_val) => {
        // TODO: Maybe make the error parameterizable.
        assert!((*left_val - *right_val).abs() < 0.0000001)
      }
    }
  };
}

#[test]
fn test_best_match() {
  let all_cases = read_all_test_cases_data();
  for case in all_cases {
    let (p1, p2) = case.polygons;
    let match_result = polygon_matcher::best_match(&p1, &p2);
    println!("{:.12} == {:.12}", case.correct_answer, match_result);
    assert_similar!(case.correct_answer, match_result);
  }
}

#[test]
fn test_read_all_test_cases_data() {
  let all_cases = read_all_test_cases_data();
  assert_eq!(all_cases.len(), 59);

  for case in all_cases {
    let (p1, p2) = case.polygons;
    assert!(!p1.vertices.is_empty());
    assert!(!p2.vertices.is_empty());
  }
}
