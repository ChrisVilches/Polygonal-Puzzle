use polygon_puzzle::shapes::polygon::Polygon;
use std::str::FromStr;

pub struct TestCase {
  pub polygons: (Polygon, Polygon),
  pub correct_answer: f64,
  pub boundary_count: usize,
}

fn lines_to_nums<T>(text: &str) -> Vec<T>
where
  T: FromStr + std::fmt::Debug,
  <T as FromStr>::Err: std::fmt::Debug,
{
  text
    .lines()
    .map(|line| line.parse::<T>().expect("should parse"))
    .collect::<Vec<T>>()
}

fn zip_all(
  polygons: Vec<Polygon>,
  answers: Vec<f64>,
  boundary_counts: Vec<usize>,
) -> Vec<TestCase> {
  let odd = polygons.iter().step_by(2).cloned();
  let even = polygons.iter().skip(1).step_by(2).cloned();
  let polygon_pair = odd.zip(even);
  let result = answers.into_iter().zip(boundary_counts.into_iter());
  polygon_pair
    .zip(result)
    .map(|(polygons, (correct_answer, boundary_count))| TestCase {
      polygons,
      correct_answer,
      boundary_count,
    })
    .collect()
}

impl TestCase {
  pub fn vec_from(input: &str, output: &str, boundary: &str) -> Vec<Self> {
    let answers = lines_to_nums::<f64>(output);
    let boundary_counts = lines_to_nums::<usize>(boundary);

    let mut polygons: Vec<Polygon> = vec![];

    let lines_iterator = &mut input.lines();

    while let Some(line) = lines_iterator.next() {
      let n = line.parse().expect("should parse an integer");
      polygons.push(Polygon::from(n, lines_iterator).expect("should parse polygon"));
    }

    zip_all(polygons, answers, boundary_counts)
  }
}
