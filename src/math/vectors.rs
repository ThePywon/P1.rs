use num_traits::cast::AsPrimitive;
use num_traits::Num;
use std::ops::{Add, Sub, Mul};

// Vector2 definition
#[derive(Debug)]
pub struct Vector2<T: Num + AsPrimitive<f64>> {
	pub x: T,
	pub y: T
}

// Vector2 methods definition
impl<T: Num + AsPrimitive<f64>> Vector2<T> {
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

// Vector2 add operator definition
impl<T: Num + AsPrimitive<f64>> Add<Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Add<&Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Add<Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Add<&Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn add(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

// Vector2 sub operator definition
impl<T: Num + AsPrimitive<f64>> Sub<Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Sub<&Vector2<T>> for Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Sub<Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl<T: Num + AsPrimitive<f64>> Sub<&Vector2<T>> for &Vector2<T> {
	type Output = Vector2<T>;

	fn sub(self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

// Vector2 with Vector2 mul operator definition
impl<T: Num + AsPrimitive<f64>> Mul<Vector2<T>> for Vector2<T> {
	type Output = T;

	fn mul(self, rhs: Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Num + AsPrimitive<f64>> Mul<&Vector2<T>> for Vector2<T> {
	type Output = T;

	fn mul(self, rhs: &Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Num + AsPrimitive<f64>> Mul<Vector2<T>> for &Vector2<T> {
	type Output = T;

	fn mul(self, rhs: Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl<T: Num + AsPrimitive<f64>> Mul<&Vector2<T>> for &Vector2<T> {
	type Output = T;

	fn mul(self, rhs: &Vector2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}
}

// Vector2 with scalar mul operator definition
impl<T: Num + AsPrimitive<f64>> Mul<T> for Vector2<T> {
	type Output = Vector2<T>;

	fn mul(self, rhs: T) -> Vector2<T> {
		Vector2 { x: self.x * rhs, y: self.y * rhs }
	}
}
impl<T: Num + AsPrimitive<f64>> Mul<T> for &Vector2<T> {
	type Output = Vector2<T>;

	fn mul(self, rhs: T) -> Vector2<T> {
		Vector2 { x: self.x * rhs, y: self.y * rhs }
	}
}