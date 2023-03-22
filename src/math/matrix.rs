use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul};
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

  pub fn is_square() -> bool { X == Y }

  #[allow(dead_code)]
  pub fn identity() -> Self {
    if X != Y {
      panic!("identity matrix MUST be square");
    }
    let mut data = [[0f64; X]; Y];
    for i in 0..X {
      data[i][i] = 1f64;
    }
    Matrix { data }
  }

  #[allow(dead_code)]
  pub fn scale(values: [f64; X]) -> Self {
    if X != Y {
      panic!("scaling matrix MUST be square");
    }
    let mut data = [[0f64; X]; Y];
    for i in 0..values.len() {
      data[i][i] = values[i];
    }
    Matrix { data }
  }

  #[allow(dead_code)]
  pub fn translate(values: [f64; X]) -> Self {
    if X != Y {
      panic!("translation matrix MUST be square");
    }
    let mut data = [[0f64; X]; Y];
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
    impl_mul_matrix!(@inner, Matrix<A, B>, Matrix<C, D>);
    impl_mul_matrix!(@inner, Matrix<A, B>, &Matrix<C, D>);
    impl_mul_matrix!(@inner, &Matrix<A, B>, Matrix<C, D>);
    impl_mul_matrix!(@inner, &Matrix<A, B>, &Matrix<C, D>);
  };

  (@inner, $lhs:ty, $rhs:ty) => {
    impl<const A: usize, const B: usize, const C: usize, const D: usize> Mul<$rhs> for $lhs {
      type Output = Matrix<C, B>;

      fn mul(self, rhs: $rhs) -> Matrix<C, B> {
        if A != D {
          panic!("lhs MUST have a width equal to the height of rhs");
        }
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

impl_matrix!(Add);
impl_matrix!(Sub);
impl_mul_matrix!();

pub trait AsVector2 {
  fn as_vector2(&self) -> Vector2<f64>;
}
pub trait AsVector3 {
  fn as_vector3(&self) -> Vector3<f64>;
}

impl AsVector2 for Matrix<1, 3> {
  fn as_vector2(&self) -> Vector2<f64> {
    Vector2 { x: self.data[0][0], y: self.data[1][0] }
  }
}

impl AsVector3 for Matrix<1, 4> {
  fn as_vector3(&self) -> Vector3<f64> {
    Vector3 { x: self.data[0][0], y: self.data[1][0], z: self.data[2][0] }
  }
}
