use std::{
  fs::File,
  io::{BufWriter, Write},
};

use polygonal_puzzle::{shapes::polygon::Polygon, traits::desmos::Desmos, util::equal};

use super::{WriteResult, RESULTS_DIR};

pub struct OutputWriter {
  file_handler: BufWriter<File>,
}

impl OutputWriter {
  pub fn new() -> Self {
    let f = File::create(format!("{RESULTS_DIR}/desmos.txt")).unwrap();

    Self {
      file_handler: BufWriter::new(f),
    }
  }
}

impl Drop for OutputWriter {
  fn drop(&mut self) {
    self.file_handler.flush().expect("should be able to flush");
  }
}

impl WriteResult for OutputWriter {
  fn write_result(&mut self, boundary: f64, case_number: i32, p1: Polygon, p2: Polygon) {
    let s = if equal(boundary, 0_f64) {
      format!("(case #{case_number}) No solution found\n")
    } else {
      format!(
        "(case #{}) Solution found ({:.12})\n{}\n{}\n",
        case_number,
        boundary,
        p1.fmt_desmos(),
        p2.fmt_desmos()
      )
    };

    self
      .file_handler
      .write_all((s + "\n").as_bytes())
      .expect("should be able to write to file");
  }
}
