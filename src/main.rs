mod math;
use math::vectors::Vector3;
use glfw::Context;

fn main() {
	let a = Vector3::new(2, 5, 3);
	let b = Vector3::new(3, 2, 4);
	let c = &a * 2 - &b;

	println!("{}", a);
	println!("{}", b);
	println!("{}", c);
	println!("\"a\" has a magnitude of {} and an angle of {}", a.get_magnitude(), a.get_deg_dir());
	println!("\"b\" has a magnitude of {} and an angle of {}", b.get_magnitude(), b.get_deg_dir());
	println!("\"c\" has a magnitude of {} and an angle of {}", c.get_magnitude(), c.get_deg_dir());

	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, _) = glfw.create_window(300, 300,
		"test", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);

	while !window.should_close() {
		window.swap_buffers();
		glfw.poll_events();
	}
}
