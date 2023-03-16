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
	pub fn new(x: T, y: T) -> Self {
		Vector2 { x, y }
	}

	pub fn get_magnitude(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();

		x.hypot(y)
	}

	pub fn get_rad_dir(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();

		(y/x).atan()
	}

	pub fn get_deg_dir(&self) -> f64 {
		self.get_rad_dir() * 57.2958f64
	}
}

impl<T: Vectory + Display> Display for Vector2<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

// Vector2 operator definitions
macro_rules! impl_vector2 {
  (Add) => {
    impl_vector2!(@inner Add, Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Add, Vector2<T>, &Vector2<T>);
    impl_vector2!(@inner Add, &Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Add, &Vector2<T>, &Vector2<T>);
  };
  (Sub) => {
    impl_vector2!(@inner Sub, Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Sub, Vector2<T>, &Vector2<T>);
    impl_vector2!(@inner Sub, &Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Sub, &Vector2<T>, &Vector2<T>);
  };
  (Mul) => {
    impl_vector2!(@inner Mul, Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Mul, Vector2<T>, &Vector2<T>);
    impl_vector2!(@inner Mul, &Vector2<T>, Vector2<T>);
    impl_vector2!(@inner Mul, &Vector2<T>, &Vector2<T>);
  };

  (@inner Add, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Add<$rhs> for $lhs {
      type Output = Vector2<T>;

      fn add(self, rhs: $rhs) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
      }
    }
  };
  (@inner Sub, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Sub<$rhs> for $lhs {
      type Output = Vector2<T>;

      fn sub(self, rhs: $rhs) -> Vector2<T> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
      }
    }
  };
  (@inner Mul, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Mul<$rhs> for $lhs {
      type Output = Vector2<T>;

      fn mul(self, rhs: $rhs) -> Vector2<T> {
        Vector2 { x: self.x * rhs.x, y: self.y * rhs.y }
      }
    }
  };
}

impl_vector2!(Add);
impl_vector2!(Sub);
impl_vector2!(Mul);

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
	pub fn new(x: T, y: T, z: T) -> Self {
		Vector3 { x, y, z }
	}

	pub fn get_magnitude(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();
		let z = self.z.as_();

		(x*x + y*y + z*z).sqrt()
	}

	pub fn get_rad_dir(&self) -> f64 {
		let x = self.x.as_();
		let y = self.y.as_();
		let z = self.z.as_();

		(z.hypot(y)/x).atan()
	}

	pub fn get_deg_dir(&self) -> f64 {
		self.get_rad_dir() * 57.2958f64
	}
}

impl<T: Vectory + Display> Display for Vector3<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

// Vector3 operator definitions
macro_rules! impl_vector3 {
  (Add) => {
    impl_vector3!(@inner Add, Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Add, Vector3<T>, &Vector3<T>);
    impl_vector3!(@inner Add, &Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Add, &Vector3<T>, &Vector3<T>);
  };
  (Sub) => {
    impl_vector3!(@inner Sub, Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Sub, Vector3<T>, &Vector3<T>);
    impl_vector3!(@inner Sub, &Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Sub, &Vector3<T>, &Vector3<T>);
  };
  (Mul) => {
    impl_vector3!(@inner Mul, Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Mul, Vector3<T>, &Vector3<T>);
    impl_vector3!(@inner Mul, &Vector3<T>, Vector3<T>);
    impl_vector3!(@inner Mul, &Vector3<T>, &Vector3<T>);
  };

  (@inner Add, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Add<$rhs> for $lhs {
      type Output = Vector3<T>;

      fn add(self, rhs: $rhs) -> Vector3<T> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
      }
    }
  };
  (@inner Sub, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Sub<$rhs> for $lhs {
      type Output = Vector3<T>;

      fn sub(self, rhs: $rhs) -> Vector3<T> {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
      }
    }
  };
  (@inner Mul, $lhs:ty, $rhs:ty) => {
    impl<T: Vectory> Mul<$rhs> for $lhs {
      type Output = Vector3<T>;

      fn mul(self, rhs: $rhs) -> Vector3<T> {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
      }
    }
  };
}

impl_vector3!(Add);
impl_vector3!(Sub);
impl_vector3!(Mul);

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
