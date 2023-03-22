mod math;
use math::vectors::Vector3;
use math::matrix::Matrix;

use crate::math::matrix::AsVector2;
//use glfw::Context;

fn main() {
  let a: Matrix<3> = Matrix::translate([1f64, 2f64, 1f64]);
  let mut b: Matrix<1, 3> = Matrix::new(0f64);
  b.data = [[3f64], [4f64], [1f64]];
  let c = a * b;

  println!("{}\n\n{}\n\n{}\n\n{}\n\n{}", a, b, c, b.as_vector2(), c.as_vector2());

/* 
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, _) = glfw.create_window(300, 300,
		"test", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);

	while !window.should_close() {
		window.swap_buffers();
		glfw.poll_events();
	}*/
}
