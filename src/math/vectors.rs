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

// Vector2 add operator definition
impl<T: Vectory> Add<Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Vectory> Add<&Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Vectory> Add<Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Vectory> Add<&Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

// Vector2 sub operator definition
impl<T: Vectory> Sub<Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Vectory> Sub<&Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Vectory> Sub<Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Vectory> Sub<&Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

// Vector2 with Vector2 mul operator definition
impl<T: Vectory> Mul<Vector2<T>> for Vector2<T> {
	type Output = T;

	fn mul(self, rhs: Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Vectory> Mul<&Vector2<T>> for Vector2<T> {
	type Output = T;

	fn mul(self, rhs: &Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Vectory> Mul<Vector2<T>> for &Vector2<T> {
	type Output = T;

	fn mul(self, rhs: Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Vectory> Mul<&Vector2<T>> for &Vector2<T> {
	type Output = T;

	fn mul(self, rhs: &Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}

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

impl<T: Vectory> Display for Vector3<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

// Vector3 add operator definition
impl<T: Vectory> Add<Vector3<T>> for Vector3<T> {
	type Output = Vector3<T>;

	fn add(self, rhs: Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}
impl<T: Vectory> Add<&Vector3<T>> for Vector3<T> {
	type Output = Vector3<T>;

	fn add(self, rhs: &Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}
impl<T: Vectory> Add<Vector3<T>> for &Vector3<T> {
	type Output = Vector3<T>;

	fn add(self, rhs: Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}
impl<T: Vectory> Add<&Vector3<T>> for &Vector3<T> {
	type Output = Vector3<T>;

	fn add(self, rhs: &Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

// Vector3 sub operator definition
impl<T: Vectory> Sub<Vector3<T>> for Vector3<T> {
	type Output = Vector3<T>;

	fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}
impl<T: Vectory> Sub<&Vector3<T>> for Vector3<T> {
	type Output = Vector3<T>;

	fn sub(self, rhs: &Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}
impl<T: Vectory> Sub<Vector3<T>> for &Vector3<T> {
	type Output = Vector3<T>;

	fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}
impl<T: Vectory> Sub<&Vector3<T>> for &Vector3<T> {
	type Output = Vector3<T>;

	fn sub(self, rhs: &Vector3<T>) -> Vector3<T> {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

// Vector3 with Vector3 mul operator definition
impl<T: Vectory> Mul<Vector3<T>> for Vector3<T> {
	type Output = T;

	fn mul(self, rhs: Vector3<T>) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}
impl<T: Vectory> Mul<&Vector3<T>> for Vector3<T> {
	type Output = T;

	fn mul(self, rhs: &Vector3<T>) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}
impl<T: Vectory> Mul<Vector3<T>> for &Vector3<T> {
	type Output = T;

	fn mul(self, rhs: Vector3<T>) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}
impl<T: Vectory> Mul<&Vector3<T>> for &Vector3<T> {
	type Output = T;

	fn mul(self, rhs: &Vector3<T>) -> T {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}

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
