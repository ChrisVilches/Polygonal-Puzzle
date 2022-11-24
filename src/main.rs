use std::{error::Error, io};

use polygon_puzzle::{polygon_matcher, shapes::polygon::Polygon};

fn main() -> Result<(), Box<dyn Error>> {
  let stdin = io::stdin();
  let stdin_lines = &mut stdin.lines();

  while let Some(line) = stdin_lines.next() {
    let n = line?.parse()?;
    let polygon1 = Polygon::from(n, &mut stdin_lines.map(Result::unwrap))?;

    let n = stdin_lines
      .next()
      .expect("should have 2 polygons per case")?
      .parse()?;
    let polygon2 = Polygon::from(n, &mut stdin_lines.map(Result::unwrap))?;

    println!("{:.12}", polygon_matcher::best_match(&polygon1, &polygon2));
  }

  Ok(())
}
