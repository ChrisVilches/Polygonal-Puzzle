use polygon_puzzle::shapes::polygon::Polygon;

pub struct TestCase {
  pub polygons: (Polygon, Polygon),
  pub correct_answer: f64,
}

impl TestCase {
  pub fn vec_from(input: &str, output: &str) -> Vec<Self> {
    let answers = output.lines().map(|line| line.parse().expect("should parse a float"));

    let mut polygons: Vec<Polygon> = vec![];

    let lines_iterator = &mut input.lines();

    while let Some(line) = lines_iterator.next() {
      let n = line.parse().expect("should parse an integer");
      polygons.push(Polygon::from(n, lines_iterator).expect("should parse polygon"));
    }

    let odd = polygons.iter().step_by(2).cloned();
    let even = polygons.iter().skip(1).step_by(2).cloned();
    let polygon_pair = odd.zip(even);

    polygon_pair
      .zip(answers)
      .map(|(polygons, correct_answer)| Self {
        polygons,
        correct_answer,
      })
      .collect()
  }
}
