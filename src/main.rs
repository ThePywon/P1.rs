mod event;
//use glfw::{Key, Action, Context};

fn on_create() {
  println!("Window created!");
}

fn on_close() {
  println!("Window closed!");
}

fn main() {
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
 
  let mut p1 = p1::init(glfw);

  p1.event_manager.on(p1::EventType::WindowCreate, on_create);
  p1.event_manager.on(p1::EventType::WindowClose, on_close);

  p1.create_window(300, 300, "test", glfw::WindowMode::Windowed);

  p1.run();
}
