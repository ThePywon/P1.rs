use num_traits::cast::AsPrimitive;
use num_traits::Num;
use std::ops::{Add, Sub, Mul};
use std::fmt::{Display, Formatter, Result};

pub trait Vectory: Num + AsPrimitive<f64> {}
impl<T: Num + AsPrimitive<f64>> Vectory for T {}

// Vector2 definition
#[derive(Clone, Copy)]
pub struct Vector2<T: Vectory> {
	pub x: T,
	pub y: T
}

// Vector2 methods definition
impl<T: Vectory> Vector2<T> {
  #[allow(dead_code)]
	pub fn new(x: T, y: T) -> Self {
		Vector2 { x, y }
	}

  #[allow(dead_code)]
	pub fn get_magnitude(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();

		x.hypot(y)
	}

  #[allow(dead_code)]
	pub fn get_rad_dir(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();

		(y/x).atan()
	}

  #[allow(dead_code)]
	pub fn get_deg_dir(&self) -> f64 {
		self.get_rad_dir() * 57.2958f64
	}
}

impl<T: Vectory + Display> Display for Vector2<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

// Vector operator definitions
macro_rules! impl_vector {
  ($op:ident, $id:ident) => {
    impl_vector!(@inner $op, $id, [][]);
    impl_vector!(@inner $op, $id, [][&]);
    impl_vector!(@inner $op, $id, [&][]);
    impl_vector!(@inner $op, $id, [&][&]);
  };

  (@inner Add, Vector2, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Add<$($r_prefix)?Vector2<T>> for $($l_prefix)?Vector2<T> {
      type Output = Vector2<T>;

      fn add(self, rhs: $($r_prefix)?Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
      }
    }
  };
  (@inner Add, Vector3, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Add<$($r_prefix)?Vector3<T>> for $($l_prefix)?Vector3<T> {
      type Output = Vector3<T>;

      fn add(self, rhs: $($r_prefix)?Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
      }
    }
  };
  (@inner Sub, Vector2, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Sub<$($r_prefix)?Vector2<T>> for $($l_prefix)?Vector2<T> {
      type Output = Vector2<T>;

      fn sub(self, rhs: $($r_prefix)?Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
      }
    }
  };
  (@inner Sub, Vector3, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Sub<$($r_prefix)?Vector3<T>> for $($l_prefix)?Vector3<T> {
      type Output = Vector3<T>;

      fn sub(self, rhs: $($r_prefix)?Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
      }
    }
  };
  (@inner Mul, Vector2, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Mul<$($r_prefix)?Vector2<T>> for $($l_prefix)?Vector2<T> {
      type Output = Vector2<T>;

      fn mul(self, rhs: $($r_prefix)?Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x * rhs.x, y: self.y * rhs.y }
      }
    }
  };
  (@inner Mul, Vector3, [$($l_prefix:tt)?] [$($r_prefix:tt)?]) => {
    impl<T: Vectory> Mul<$($r_prefix)?Vector3<T>> for $($l_prefix)?Vector3<T> {
      type Output = Vector3<T>;

      fn mul(self, rhs: $($r_prefix)?Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
      }
    }
  };
}

impl_vector!(Add, Vector2);
impl_vector!(Sub, Vector2);
impl_vector!(Mul, Vector2);

// Vector2 with scalar mul operator definition
impl<T: Vectory> Mul<T> for Vector2<T> {
	type Output = Vector2<T>;

	fn mul(self, rhs: T) -> Vector2<T> {
		Vector2 { x: self.x * rhs, y: self.y * rhs }
	}
}
impl<T: Vectory> Mul<T> for &Vector2<T> {
	type Output = Vector2<T>;

	fn mul(self, rhs: T) -> Vector2<T> {
		Vector2 { x: self.x * rhs, y: self.y * rhs }
	}
}

#[derive(Clone, Copy)]
pub struct Vector3<T: Vectory> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T: Vectory> Vector3<T> {
  #[allow(dead_code)]
	pub fn new(x: T, y: T, z: T) -> Self {
		Vector3 { x, y, z }
	}

  #[allow(dead_code)]
	pub fn get_magnitude(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();
		let z = self.z.as_();

		(x*x + y*y + z*z).sqrt()
	}

  #[allow(dead_code)]
	pub fn get_rad_dir(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();
		let z = self.z.as_();

		(z.hypot(y)/x).atan()
	}

  #[allow(dead_code)]
	pub fn get_deg_dir(&self) -> f64 {
		self.get_rad_dir() * 57.2958f64
	}
}

impl<T: Vectory + Display> Display for Vector3<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl_vector!(Add, Vector3);
impl_vector!(Sub, Vector3);
impl_vector!(Mul, Vector3);

// Vector3 with scalar mul operator definition
impl<T: Vectory> Mul<T> for Vector3<T> {
	type Output = Vector3<T>;

	fn mul(self, rhs: T) -> Vector3<T> {
		Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}
impl<T: Vectory> Mul<T> for &Vector3<T> {
	type Output = Vector3<T>;

	fn mul(self, rhs: T) -> Vector3<T> {
		Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}
