use crate::shapes::polygon::Polygon;
use shapes::point::Point;
use std::io;
mod constants;
mod polygon_matcher;
mod shapes;
mod traits;
mod util;

fn read_line() -> String {
  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  line
}

fn read_int() -> i32 {
  read_line().trim().parse().unwrap()
}

fn read_point() -> Point {
  let line = read_line();
  let nums: Vec<&str> = line.split(' ').collect();

  Point {
    x: nums[0].trim().parse().unwrap(),
    y: nums[1].trim().parse().unwrap(),
  }
}

fn main() {
  loop {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse();

    let n = match n {
      Ok(val) => val,
      Err(_) => {
        break;
      }
    };

    let mut vertices1: Vec<Point> = (0..n).map(|_| read_point()).collect();
    let mut vertices2: Vec<Point> = (0..read_int()).map(|_| read_point()).collect();

    vertices1.reverse();
    vertices2.reverse();

    let polygon1 = Polygon::new(vertices1);
    let polygon2 = Polygon::new(vertices2);

    println!("{:.12}", polygon_matcher::best_match(&polygon1, &polygon2));
  }
}
