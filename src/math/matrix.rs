use std::fmt::{Display, Formatter, Result};
use std::cmp::min;

// Matrix definition
#[derive(Clone, Copy)]
pub struct Matrix<const X: usize, const Y: usize = X> {
  pub data: [[f64; X]; Y]
}

impl<const X: usize, const Y: usize> Matrix<X, Y> {
  #[allow(dead_code)]
  pub fn new(v: f64) -> Self {
    let data = [[v; X]; Y];
    Matrix { data }
  }

  #[allow(dead_code)]
  pub fn identity() -> Self {
    let mut data = [[0f64; X]; Y];
    for i in 0..min(X, Y) {
      data[i][i] = 1f64;
    }
    Matrix { data }
  }
}

impl<const X: usize, const Y: usize> Display for Matrix<X, Y> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut output = String::new();
    for y in 0..Y {
      output += "[";
      for x in 0..X {
        output += &self.data[y][x].to_string();
        if x != X-1 {
          output += ", ";
        }
      }
      output += "]\n";
    }
    write!(f, "{}", output)
  }
}
