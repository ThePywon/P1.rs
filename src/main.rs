mod math;
use math::vectors::Vector2;

fn main() {
	let a = Vector2 { x: 2, y: 5 };
	let b = Vector2 { x: 3, y: 2 };
	let c = &a * &b;

	dbg!(&a);
	dbg!(&b);
	dbg!(&c);
	//println!("\"c\" has a magnitude of {} and an angle of {}", c.get_magnitude(), c.get_deg_dir());
}

/*
use glfw::Context;

fn main() {
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
*/