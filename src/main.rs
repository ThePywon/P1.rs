mod math;
use math::vectors::Vector2;
use math::matrix::Matrix;
mod event;
use event::event_manager::KeyOnlyEventManager;
use glfw::{Action, Context, Key};

fn test() {
  println!("Call #1");
}

fn test2() {
  println!("Call #2");
}

fn main() {
  let mut event_manager: KeyOnlyEventManager<String> = KeyOnlyEventManager::new();

  event_manager.on(String::from("start"), test);
  event_manager.on(String::from("start"), test2);

  let mut a: Matrix<2> = Matrix::new(1f64);
  let b: Matrix<2> = Matrix { data: [[1f64, 1f64], [0f64, 1f64]] };

  a *= b;

  let c: Matrix<1, 3> = Matrix::new(1f64);

  println!("{}", a);
  println!("{}", Vector2::<f64>::from(c));

	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw.create_window(300, 300,
		"test", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);

	while !window.should_close() {
		window.swap_buffers();
		glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
          event_manager.emit(String::from("start"));
          window.set_should_close(true)
        },
        _ => {},
      }
    }
	}
}
