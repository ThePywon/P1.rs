mod math;
use math::vectors::Vector2;
use math::matrix::Matrix;
use crate::math::matrix::SquareMatrix;
//use glfw::Context;

fn main() {
  let mut a: Matrix<2> = Matrix::new(1f64);
  let mut b: Matrix<2> = Matrix { data: [[1f64, 1f64], [0f64, 1f64]] };

  a *= b;

  println!("{}", a);

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
