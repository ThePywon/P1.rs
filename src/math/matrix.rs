use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use super::vectors::{Vector2, Vector3};

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
}

pub trait SquareMatrix<const X: usize> {
  fn identity() -> Self;
  fn scale(values: [f64; X]) -> Self;
  fn translate(values: [f64; X]) -> Self;
}

impl<const X: usize> SquareMatrix<X> for Matrix<X> {
  #[allow(dead_code)]
  fn identity() -> Self {
    let mut data = [[0f64; X]; X];
    for i in 0..X {
      data[i][i] = 1f64;
    }
    Matrix { data }
  }

  #[allow(dead_code)]
  fn scale(values: [f64; X]) -> Self {
    let mut data = [[0f64; X]; X];
    for i in 0..values.len() {
      data[i][i] = values[i];
    }
    Matrix { data }
  }

  #[allow(dead_code)]
  fn translate(values: [f64; X]) -> Self {
    let mut data = [[0f64; X]; X];
    for i in 0..X {
      data[i][i] = 1f64;
    }
    for i in 0..values.len() {
      data[i][X-1] = values[i];
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

macro_rules! impl_matrix {
  ($op:ident) => {
    impl_matrix!(@inner, $op, Matrix<X, Y>, Matrix<X, Y>);
    impl_matrix!(@inner, $op, Matrix<X, Y>, &Matrix<X, Y>);
    impl_matrix!(@inner, $op, &Matrix<X, Y>, Matrix<X, Y>);
    impl_matrix!(@inner, $op, &Matrix<X, Y>, &Matrix<X, Y>);  
  };

  (@inner, Add, $lhs:ty, $rhs:ty) => {
    impl<const X: usize, const Y: usize> Add<$rhs> for $lhs {
      type Output = Matrix<X, Y>;

      fn add(self, rhs: $rhs) -> Matrix<X, Y> {
        let mut data = self.data;
        for y in 0..Y {
          for x in 0..X {
            data[y][x] += rhs.data[y][x];
          }
        }
        Matrix { data }
      }
    }
  };
  (@inner, Sub, $lhs:ty, $rhs:ty) => {
    impl<const X: usize, const Y: usize> Sub<$rhs> for $lhs {
      type Output = Matrix<X, Y>;

      fn sub(self, rhs: $rhs) -> Matrix<X, Y> {
        let mut data = self.data;
        for y in 0..Y {
          for x in 0..X {
            data[y][x] -= rhs.data[y][x];
          }
        }
        Matrix { data }
      }
    }
  };
}

macro_rules! impl_mul_matrix {
  () => {
    impl_mul_matrix!(@inner, Matrix<A, B>, Matrix<C, A>);
    impl_mul_matrix!(@inner, Matrix<A, B>, &Matrix<C, A>);
    impl_mul_matrix!(@inner, &Matrix<A, B>, Matrix<C, A>);
    impl_mul_matrix!(@inner, &Matrix<A, B>, &Matrix<C, A>);
  };

  (@inner, $lhs:ty, $rhs:ty) => {
    impl<const A: usize, const B: usize, const C: usize> Mul<$rhs> for $lhs {
      type Output = Matrix<C, B>;

      fn mul(self, rhs: $rhs) -> Matrix<C, B> {
        let mut data = [[0f64; C]; B];
        let mut result: f64;
        for y in 0..B {
          for x in 0..C {
            result = 0f64;
            for i in 0..A {
              result += self.data[y][i] * rhs.data[i][x];
            }
            data[y][x] = result;
          }
        }
        Matrix { data }
      }
    }
  };
}

macro_rules! impl_assign_matrix {
  ($op:ident) => {
    impl_assign_matrix!(@inner, $op, Self);
    impl_assign_matrix!(@inner, $op, &Self);
  };

  (@inner, AddAssign, $rhs:ty) => {
    impl<const X: usize, const Y: usize> AddAssign<$rhs> for Matrix<X, Y> {
      fn add_assign(&mut self, rhs: $rhs) {
        for y in 0..Y {
          for x in 0..X {
            self.data[y][x] += rhs.data[y][x];
          }
        }
      }
    }
  };
  (@inner, SubAssign, $rhs:ty) => {
    impl<const X: usize, const Y: usize> SubAssign<$rhs> for Matrix<X, Y> {
      fn sub_assign(&mut self, rhs: $rhs) {
        for y in 0..Y {
          for x in 0..X {
            self.data[y][x] -= rhs.data[y][x];
          }
        }
      }
    }
  };
  (@inner, MulAssign, $rhs:ty) => {
    impl<const X: usize> MulAssign<$rhs> for Matrix<X> {
      fn mul_assign(&mut self, rhs: $rhs) {
        let mut data = [[0f64; X]; X];
        let mut result: f64;
        for y in 0..X {
          for x in 0..X {
            result = 0f64;
            for i in 0..X {
              result += self.data[y][i] * rhs.data[i][x];
            }
            data[y][x] = result;
          }
        }
        self.data = data;
      }
    }
  }
}

impl_matrix!(Add);
impl_matrix!(Sub);
impl_mul_matrix!();
impl_assign_matrix!(AddAssign);
impl_assign_matrix!(SubAssign);
impl_assign_matrix!(MulAssign);

impl Into<Vector2<f64>> for Matrix<1, 3> {
  fn into(self) -> Vector2<f64> {
    Vector2 { x: self.data[0][0], y: self.data[1][0] }
  }
}
