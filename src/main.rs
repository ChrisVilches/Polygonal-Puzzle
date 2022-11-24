use std::io;

use polygon_puzzle::{
  polygon_matcher,
  shapes::{point::Point, polygon::Polygon},
};

fn read_line() -> Option<String> {
  let mut line = String::new();

  match io::stdin().read_line(&mut line) {
    Ok(_) => Some(line.trim().to_owned()),
    Err(_) => None,
  }
}

fn read_next_case() -> Option<(Polygon, Polygon)> {
  let line = read_line()?;

  let n = line.parse::<usize>().ok()?;
  let mut vertices1: Vec<Point> = (0..n)
    .map(|_| read_line().unwrap().parse().unwrap())
    .collect();
  let n = read_line()?.parse::<usize>().ok()?;

  let mut vertices2: Vec<Point> = (0..n)
    .map(|_| read_line().unwrap().parse().unwrap())
    .collect();

  vertices1.reverse();
  vertices2.reverse();

  Some((Polygon::new(vertices1), Polygon::new(vertices2)))
}

fn main() {
  while let Some((p1, p2)) = read_next_case() {
    println!("{:.12}", polygon_matcher::best_match(&p1, &p2));
  }
}
