mod math;
use math::vectors::Vector2;
use math::matrix::Matrix;
//use glfw::Context;

fn main() {
  let t: Matrix<3> = Matrix::translate([1f64, 2f64, 1f64]);
  let mut s: Matrix<1, 3> = Matrix::new(0f64);

  s.data = [[2f64], [3f64], [1f64]];

  println!("{}", t * s);

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
