use std::{
  fs::File,
  io::{BufWriter, Write},
};

use polygon_puzzle::{shapes::polygon::Polygon, traits::desmos::Desmos, util::equal};

use super::{WriteResult, RESULTS_DIR};

pub struct DesmosOutputWriter {
  file_handler: BufWriter<File>,
}

const EXPECT_WRITE_DATA: &str = "should be able to write to file";

impl DesmosOutputWriter {
  pub fn new() -> Self {
    let f = File::create(format!("{}/desmos.txt", RESULTS_DIR)).unwrap();

    Self {
      file_handler: BufWriter::new(f),
    }
  }
}

impl Drop for DesmosOutputWriter {
  fn drop(&mut self) {
    self.file_handler.flush().expect("should be able to flush");
  }
}

impl WriteResult for DesmosOutputWriter {
  fn write_result(&mut self, boundary: f64, case_number: i32, p1: Polygon, p2: Polygon) {
    let s = if equal(boundary, 0_f64) {
      format!("(case #{}) No solution found\n", case_number)
    } else {
      format!(
        "(case #{}) Solution found ({:.12})\n{}\n\n{}\n",
        case_number,
        boundary,
        p1.fmt_desmos(),
        p2.fmt_desmos()
      )
    };

    self
      .file_handler
      .write_all(s.as_bytes())
      .expect(EXPECT_WRITE_DATA);
    self.file_handler.write_all(b"\n").expect(EXPECT_WRITE_DATA);
  }
}
