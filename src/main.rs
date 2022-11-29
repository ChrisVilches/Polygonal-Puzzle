#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

use std::{error::Error, io};

mod result_output;
use crossbeam::channel::{Receiver, Sender};
use polygonal_puzzle::{polygon_matcher, shapes::polygon::Polygon};
use result_output::WriteResult;

fn write_results_thread(r: &Receiver<(Polygon, Polygon, f64)>) {
  let mut case_number = 1;

  let mut writes: [Box<dyn WriteResult>; 2] = [
    Box::new(result_output::desmos::OutputWriter::new()),
    Box::new(result_output::svg::OutputWriter {}),
  ];

  while let Ok((p1, p2, boundary)) = r.recv() {
    writes
      .iter_mut()
      .for_each(|w| w.write_result(boundary, case_number, p1.clone(), p2.clone()));

    case_number += 1;
  }
}

fn main_thread(s: Sender<(Polygon, Polygon, f64)>) -> Result<(), Box<dyn Error>> {
  let stdin_lines = &mut io::stdin().lines();

  while let Some(line) = stdin_lines.next() {
    let n = line?.parse()?;
    let polygon1 = Polygon::from(n, &mut stdin_lines.map(Result::unwrap))?;

    let n = stdin_lines
      .next()
      .expect("should have 2 polygons per case")?
      .parse()?;
    let polygon2 = Polygon::from(n, &mut stdin_lines.map(Result::unwrap))?;

    let (p1, p2, boundary) = polygon_matcher::best_match(&polygon1, &polygon2);

    s.send((p1.clone(), p2.clone(), boundary))?;
    println!("{:.12}", boundary);
  }

  std::mem::drop(s);

  Ok(())
}

fn main() {
  let (s, r) = crossbeam::channel::unbounded();

  crossbeam::scope(|scope| {
    scope.spawn(|_| main_thread(s).unwrap());
    scope.spawn(|_| write_results_thread(&r));
  })
  .expect("both threads should exit without errors");
}
